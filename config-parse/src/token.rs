use std::{cell::RefCell, error::Error, fmt::{self, Display}, iter::{Copied, Enumerate}, ops::Range, slice::Iter};
use nom::{bytes::complete::tag, Compare, CompareResult, InputIter, InputLength, InputTake, Needed, Parser};

use nom_locate::LocatedSpan;
use nom_supreme::error::{ErrorTree, GenericErrorTree};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'a> {
    Ident {name: &'a str},
    K(Keyword),
    Separator(char),
    Comment{content: &'a str},
    Float(f32),
    Integer(i32),
    Bool(bool)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Fn,
    Macro,
    Profile,
    ColorAnimation,
    Loop,
}

/// Error containing a text span and an error message to display.
#[derive(Debug)]
pub struct RecoveredError(pub Range<usize>, pub String);

/// Carried around in the `LocatedSpan::extra` field in
/// between `nom` parsers.
#[derive(Clone, Copy, Debug)]
pub struct ParseState<'a>(pub &'a RefCell<Vec<RecoveredError>>);
pub type StrSpan<'a, 'b> = LocatedSpan<&'a str, ParseState<'b>>;
pub type StrResult<I, O, E=ErrorTree<I>> = Result<(I, O), nom::Err<E>>;
pub type Spanned<T> = (T, Range<usize>);
pub type TokSpan<'a, 'b> = LocatedSpan<Token<'a>, (ParseState<'b>, usize)>;
pub type TokError<'a, 'b> = GenericErrorTree<Tokens<'a, 'b>, &'a [TokSpan<'a, 'b>], &'static str, Box<dyn Error + 'a>>;
pub type TokResult<'a, 'b, O, I=Tokens<'a, 'b>, E=TokError<'a, 'b>> = Result<(I, O), nom::Err<E>>;

pub trait FromStrSpan<'a, 'b> {
    fn from_strspan(token: Token<'a>, state: ParseState<'b>, span: Range<usize>) -> TokSpan<'a, 'b>;
}

impl<'a, 'b> FromStrSpan<'a, 'b> for TokSpan<'a, 'b> {
    #[inline]
    fn from_strspan(token: Token<'a>, state: ParseState<'b>, span: Range<usize>) -> TokSpan<'a, 'b> {
        unsafe{TokSpan::new_from_raw_offset(span.start, 0, token, (state, span.end-span.start))}
    }
}

pub trait ToRange {
    fn span(&self) -> Range<usize>;
    fn consumed_span(&self, next_start: usize) -> Range<usize>;
}


impl<'a, 'b> ToRange for TokSpan<'a, 'b> {
    fn span(&self) -> Range<usize> {
        let start = self.location_offset();
        start..start+self.extra.1
    }

    #[allow(unused_variables)]
    fn consumed_span(&self, next_start: usize) -> Range<usize> {
        unimplemented!()
    }
}

impl<'a, 'b> ToRange for Tokens<'a, 'b> {
    fn span(&self) -> Range<usize> {
        let start = self.offset;
        let end = match self.tokens.len() {
            0 => start+1,
            _ => {
                let end = self.tokens[self.tokens.len()-1];
                end.location_offset()+end.extra.1
            }
        };
        start..end
    }

    fn consumed_span(&self, next_start: usize) -> Range<usize> {
        let start = self.span().start;
        let mut end = start;
        for token in self.tokens {
            let tok_span = token.span();
            if tok_span.start >= next_start {
                break
            } else {
                end = tok_span.end
            }
        }

        start..end
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Tokens<'a, 'b> {
    pub tokens: &'a [TokSpan<'a, 'b>],
    offset: usize,
    pub state: ParseState<'a>
}

impl<'a, 'b> Tokens<'a, 'b> {
    pub fn new(tokens: &'a [TokSpan<'a, 'b>], state: ParseState<'a>) -> Tokens<'a, 'b> {
        Tokens { tokens, offset: 0, state }
    }
}

impl<'a, 'b> Compare<Token<'a>> for Tokens<'a, 'b> {
    fn compare(&self, t: Token) -> CompareResult {
        if self.tokens.len() == 0 || *self.tokens[0].fragment() != t {
            CompareResult::Error
        } else {
            CompareResult::Ok
        }
    }

    fn compare_no_case(&self, t: Token) -> CompareResult {
        self.compare(t)
    }
}

impl<'a, 'b> Display for Tokens<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.tokens)
    }
}


impl<'a, 'b> ToRange for StrSpan<'a, 'b> {
    fn span(&self) -> Range<usize> {
        let start = self.get_column_first_line()-1;
        start..start+self.fragment().chars().count()
    }

    #[allow(unused_variables)]
    fn consumed_span(&self, next_start: usize) -> Range<usize> {
        let start = self.span().start;

        start..next_start
    }
}

impl<'a,I, E: nom::error::ParseError<I>> Parser<I,I,E> for Token<'a>
where
  I: InputTake + Compare<Token<'a>>,
{
    fn parse(&mut self, input: I) -> nom::IResult<I, I, E> {
        tag(self.to_owned()).parse(input)
    }
}

impl<'a, 'b> InputLength for Tokens<'a, 'b> {
    #[inline]
    fn input_len(&self) -> usize {
        self.tokens.input_len()
    }
}

impl<'a, 'b> InputLength for &mut Tokens<'a, 'b> {
    #[inline]
    fn input_len(&self) -> usize {
        self.tokens.input_len()
    }
}

impl<'a> InputLength for Token<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        1
    }
}

impl<'a, 'b> InputTake for Tokens<'a, 'b> {
    fn take(&self, count: usize) -> Self {
        Tokens::new(&self.tokens[0..count], self.state)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tokens.split_at(count);
        let suf_offset = match suffix.len() {
            0 => match prefix.len() {
                0 => self.offset,
                _ => prefix[0].span().end
            },
            _ => suffix[0].span().start
        };
        (Tokens{tokens: suffix, offset: suf_offset, state: self.state}, Tokens{tokens: prefix, offset: self.offset, state: self.state})
    }
}

impl<'a, 'b> InputIter for Tokens<'a, 'b> {
    type Item = TokSpan<'a, 'b>;

    type Iter = Enumerate<Self::IterElem>;

    type IterElem = Copied<Iter<'a, Self::Item>>;

    fn iter_indices(&self) -> Self::Iter {
        unimplemented!()
    }

    fn iter_elements(&self) -> Self::IterElem {
        unimplemented!()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Item) -> bool {
        self.tokens.iter().position(|b| predicate(*b))
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        if self.tokens.len() >= count {
            Ok(count)
        } else {
        Err(Needed::new(count - self.tokens.len()))
        }
    }
}
