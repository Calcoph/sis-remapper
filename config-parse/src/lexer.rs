use std::{cell::RefCell, ops::{Range, RangeFrom, RangeTo}};

use nom::{branch::alt, bytes::complete::tag, character::{complete::{alpha1, alphanumeric1, multispace1, not_line_ending}, streaming::one_of}, combinator::{eof, map, recognize}, error::{ErrorKind, ParseError}, multi::{many0, many1, many_till}, sequence::{delimited, pair, preceded, tuple}, AsChar, Compare, CompareResult, IResult, InputIter, InputLength, Slice};

use crate::{combinators::map_with_span, token::{FromStrSpan, Keyword, ParseState, RecoveredError, StrResult, StrSpan, ToRange, TokSpan, Token}};

pub fn anything_until_multicomment_end<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
  T: InputIter + InputLength,
  T: Compare<&'static str>,
  <T as InputIter>::Item: AsChar,
  <T as InputIter>::Item: AsChar,
{
  let mut slice_start = 0;
  loop {
    match input.slice(slice_start..).position(|item| {
        let c = item.as_char();
        c == '*'
    }) {
        None => return Ok((input.slice(input.input_len()..), input)),
        Some(index) => {
            let compensated_index = slice_start+index;
            let mut it = input.slice(compensated_index..).iter_elements();
            let _asterisk = it.next().unwrap();
            if let Some(nth) = it.next() {
                let nth = nth.as_char();
                if nth == '/' {
                    let sliced = input.slice(compensated_index..);
                    let comp = sliced.compare("*/");
                    match comp {
                        //FIXME: calculate the right index
                        CompareResult::Ok => return Ok((input.slice(compensated_index..), input.slice(..compensated_index))),
                        _ => {
                            let e: ErrorKind = ErrorKind::Tag;
                            return Err(nom::Err::Error(E::from_error_kind(input, e)))
                        }
                    }
                } else {
                    slice_start = compensated_index+1
                }
            } else {
                return Ok((input.slice(input.input_len()..), input))
            }
        }
    }
  }
}


fn lexer<'a, 'b>(input: StrSpan<'a, 'b>) -> StrResult<StrSpan<'a, 'b>, Vec<TokSpan<'a, 'b>>> {
    let float = map(recognize(tuple((
            many1(one_of("0123456789")),
            tag("."),
            many1(one_of("0123456789"))
        ))),
        |s: StrSpan| {
            let float: f32 = s.fragment().parse().unwrap();
            TokSpan::from_strspan(Token::Float(float), s.extra, s.span())
    });
    let integer = map(recognize(many1(one_of("0123456789"))),
        |s: StrSpan| {
            let integer: i32 = s.fragment().parse().unwrap();
            TokSpan::from_strspan(Token::Integer(integer), s.extra, s.span())
        }
    );

    let ctrl = map(
        alt((
            tag("("),
            tag(")"),
            tag("["),
            tag("]"),
            tag("{"),
            tag("}"),
            tag(";"),
            tag(","),
            tag("."),
            tag(":"),
            tag("="),
            tag(">"),
        )),
        |s: StrSpan| {
            let state = s.extra;
            TokSpan::from_strspan(
                Token::Separator(s.fragment().chars().next().unwrap()),
                state,
                s.span()
            )
        }
    );

    let ident = map(
        recognize(
            pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_"))))
            )
        ),
        |s: StrSpan| {
            let token = match *s.fragment() {
                "fn" => Token::K(Keyword::Fn),
                "color_animation" => Token::K(Keyword::ColorAnimation),
                "profile" => Token::K(Keyword::Profile),
                "macro" => Token::K(Keyword::Macro),
                "loop" => Token::K(Keyword::Loop),
                "true" => Token::Bool(true),
                "false" => Token::Bool(false),
                s => Token::Ident{ name: s },
            };
            let state = s.extra;
            TokSpan::from_strspan(token, state, s.span())
        }
    );

    let token = alt((
        float,
        integer,
        ctrl,
        ident,
    ));

    let comment = alt((
        preceded(tag("//"), not_line_ending),
        delimited(tag("/*"), anything_until_multicomment_end, tag("*/"))
    ));

    let padding = map_with_span(
        alt((
            comment,
            multispace1
        )),
        |s: StrSpan, span| {
            let state = s.extra;
            TokSpan::from_strspan(Token::Comment{content: s.fragment()}, state, span)
        }
    );

    let pos_inputs = alt((padding, token));

    map(
        many_till(
            pos_inputs,
            eof
        ),
        |(v, _)| v
    )(input)
}

pub fn lex<'a, 'b>(input: &'a str, errors: &'b RefCell<Vec<RecoveredError>>) -> Vec<TokSpan<'a, 'b>> {
    let input = StrSpan::new_extra(input, ParseState(errors));
    let (_, tokens) = lexer(input).expect("Unrecovered error happenned in lexer");

    tokens
}
