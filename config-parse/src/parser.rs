use std::{io::ErrorKind, ops::Range};

use nom::{branch::alt, bytes::complete::{tag, take}, combinator::{eof, map, map_res}, multi::{many0, many_till, separated_list0}, sequence::{delimited, pair, preceded, separated_pair}, InputTake};
use nom_supreme::{error::{BaseErrorKind, ErrorTree, GenericErrorTree}, ParserExt};

use crate::{combinators::{map_with_span, spanned}, statement::{Color, FuncName, Keyframe, Statement, Value}, token::{Keyword, Spanned, ToRange, TokError, TokResult, TokSpan, Token, Tokens}};

pub(crate) fn ident<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<String>> {
    map_res(
        spanned(take(1 as usize)),
        |(consumed, span): (Tokens, Range<usize>)|{
            match consumed.tokens[0].fragment() {
                Token::Ident{name} => Ok((String::from(*name), span)),
                _ => Err(ErrorTree::Base {
                    location: consumed,
                    kind: BaseErrorKind::External(Box::new(tokio::io::Error::new(ErrorKind::Other, "Expected identifier")))
                })
            }
        }
    )(input)
}

pub(crate) fn float<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<f32>> {
    map_res(
        spanned(take(1 as usize)),
        |(consumed, span): (Tokens, Range<usize>)|{
            match consumed.tokens[0].fragment() {
                Token::Float(f) => Ok((*f, span)),
                _ => Err(ErrorTree::Base {
                    location: consumed,
                    kind: BaseErrorKind::External(Box::new(tokio::io::Error::new(ErrorKind::Other, "Expected float")))
                })
            }
        }
    )(input)
}

pub(crate) fn integer<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<i32>> {
    map_res(
        spanned(take(1 as usize)),
        |(consumed, span): (Tokens, Range<usize>)|{
            match consumed.tokens[0].fragment() {
                Token::Integer(i) => Ok((*i, span)),
                _ => Err(ErrorTree::Base {
                    location: consumed,
                    kind: BaseErrorKind::External(Box::new(tokio::io::Error::new(ErrorKind::Other, "Expected integer")))
                })
            }
        }
    )(input)
}

pub(crate) fn boolean<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<bool>> {
    map_res(
        spanned(take(1 as usize)),
        |(consumed, span): (Tokens, Range<usize>)|{
            match consumed.tokens[0].fragment() {
                Token::Bool(b) => Ok((*b, span)),
                _ => Err(ErrorTree::Base {
                    location: consumed,
                    kind: BaseErrorKind::External(Box::new(tokio::io::Error::new(ErrorKind::Other, "Expected integer")))
                })
            }
        }
    )(input)
}

fn value<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Value>> {
    alt((
        map_with_span(separated_pair(
            ident,
            pair(
                Token::Separator(':'),
                Token::Separator(':')
            ),
            ident
        ),
        |((enum_ident, _), (variant, _)), span| (Value::EnumVariant { enum_name: enum_ident, variant: variant }, span)),
        map(
            ident,
            |(s, span)| (Value::Variable { name: s }, span)
        ),
        map(
            integer,
            |(s, span)| (Value::Integer(s), span)
        ),
        map(
            float,
            |(s, span)| (Value::Float(s), span)
        ),
        map(
            color,
            |(s, span)| (Value::Color(s), span)
        ),
        map(
            boolean,
            |(s, span)| (Value::Bool(s), span)
        )
    ))(input)
}

pub(crate) fn func_name<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<FuncName>> {
    map(
        ident,
        |(name, span)| {
            (match name.as_str() {
                "set_hotkey" => FuncName::SetHotkey,
                "press_key" => FuncName::PressKey,
                "release_key" => FuncName::ReleaseKey,
                "switch_profile" => FuncName::SwitchProfile,
                "wave_effect" => FuncName::WaveEffect,
                "ripple_effect" => FuncName::RippleEffect,
                "static_color" => FuncName::StaticColor,
                _ => FuncName::Other(name)
            }, span)
    })(input)
}

fn function_call<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Statement>> {
    map_with_span(
    pair(
            func_name,
            delimited(
                Token::Separator('('),
                separated_list0(
                    Token::Separator(','),
                    value
                ),
                Token::Separator(')')
            )
        ),
        |(func_name, args), span| {
            (
                Statement::Call { name: func_name, args },
                span
            )
        }
    )(input)
}

fn loop_block<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Statement>> {
    map_with_span(
    preceded(
            Token::K(Keyword::Loop),
            delimited(
                Token::Separator('{'),
                many0(function_call),
                Token::Separator('}')
            )
        ),
        |calls, span| {
            (
                Statement::Loop { body: calls.into_iter().map(|(s, _)| s).collect() },
                span
            )
        }
    )(input)
}

pub(crate) fn statement<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Statement>> {
    alt((
        function_call,
        loop_block,
    ))(input)
}

pub(crate) fn profile_definition<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Statement>> {
    map_with_span(
        preceded(
            tag(Token::K(Keyword::Profile)),
            pair(
                ident.context("Expected profile name"),
                delimited(
                    tag(Token::Separator('{')).context("Missing {"),
                    spanned(many0(statement)),
                    tag(Token::Separator('}')).context("Expected } or valid statement")
                )
            )
        ),
        |(name, body), span| (
            Statement::Profile {
                name: name.0,
                body: body.0.into_iter().map(|(s, _)| s).collect()
            },
            span
        )
    )(input)
}

pub(crate) fn custom_func_name<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<String>> {
    map_res(
        func_name,
        |(name, span)| {
            let name = match name {
                FuncName::Other(name) => name,
                _ => return Err(ErrorTree::Base {
                    location: input.take(1),
                    kind: BaseErrorKind::External(Box::new(tokio::io::Error::new(ErrorKind::Other, "Expected float")))
                })
            };

            Ok((name, span))
        }
    )(input)
}

pub(crate) fn function_definition<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Statement>> {
    map_with_span(
        preceded(
            Token::K(Keyword::Fn),
            pair(
                custom_func_name.context("Expected function name"),
                delimited(
                    tag(Token::Separator('{')).context("Missing {"),
                    spanned(many0(statement)),
                    tag(Token::Separator('}')).context("Expected } or valid statement")
                )
            )
        ),
        |(name, body), span| (
            Statement::Func {
                name: name.0,
                body: body.0.into_iter().map(|(s, _)| s).collect()
            },
            span
        )
    )(input)
}

pub(crate) fn color<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Color>> {
    map_with_span(delimited(
        Token::Separator('('),
        separated_pair(
            integer,
            Token::Separator(','),
            separated_pair(
                integer,
                Token::Separator(','),
            separated_pair(
                    integer,
                    Token::Separator(','),
                    integer
                )
            )
        ),
        Token::Separator(')')
        ),
        |(r, (g, (b, a))), span| {
            (Color(r,g,b,a), span)
        }
    )(input)
}

pub(crate) fn keyframe<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Keyframe>> {
    map_with_span(separated_pair(
            float,
            pair(
                Token::Separator('='),
                Token::Separator('>')
            ),
            color
        ),
    |(timestamp, color), span   | {
        (Keyframe {
            timestamp,
            color,
        }, span)
    })(input)
}

pub(crate) fn color_animaiton_definition<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Statement>> {
    map_with_span(
        preceded(
            Token::K(Keyword::ColorAnimation),
            pair(
                ident.context("Expected color animation name"),
                delimited(
                    tag(Token::Separator('{')).context("Missing {"),
                    spanned(separated_list0(Token::Separator(','), keyframe)),
                    tag(Token::Separator('}')).context("Expected } or valid statement")
                )
            )
        ),
        |(name, body), span| (
            Statement::ColorAnimation {
                name: name.0,
                body: body.0.into_iter().collect()
            },
            span
        )
    )(input)
}

pub(crate) fn macro_definition<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Statement>> {
    map_with_span(
        preceded(
            Token::K(Keyword::Macro),
            pair(
                ident.context("Expected macro name"),
                delimited(
                    tag(Token::Separator('{')).context("Missing {"),
                    spanned(many0(statement)),
                    tag(Token::Separator('}')).context("Expected } or valid statement")
                )
            )
        ),
        |(name, body), span| (
            Statement::Macro {
                name: name.0,
                body: body.0.into_iter().map(|(s, _)| s).collect()
            },
            span
        )
    )(input)
}

fn top_level_statements<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Statement>> {
    alt((
        function_definition,
        profile_definition,
        macro_definition,
        color_animaiton_definition
    ))(input)
}

fn parser<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Vec<Spanned<Statement>>>> {
    map_with_span(
        many_till(
            |input: Tokens<'a, 'b>| match top_level_statements(input) {
                Ok(r) => Ok(r),
                Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                    let input = recover_err(&e);
                    let (_rest, input) = input.take_split(1);
                    let span = input.span();
                    panic!("Unexpected token '{:?}' at {span:?}", input.tokens[0]);
                },
                Err(e) => Err(e)
            },
            eof
        ),
        |(list, _), span| (list, span)
    )(input)
}

fn recover_err<'a, 'b>(e: &TokError<'a, 'b>) -> Tokens<'a, 'b> {
    match e {
        GenericErrorTree::Base { location, kind: _ } => *location,
        GenericErrorTree::Stack { base, contexts: _ } => recover_err(base),
        GenericErrorTree::Alt(v) => recover_err(v.get(0).unwrap()),
    }
}

// Hashmap contains the names of named expressions and their clones
pub(crate) fn token_parse(tokens: Vec<TokSpan>) -> Spanned<Vec<Spanned<Statement>>> {
    let ex = match tokens.len() {
        0 => (vec![], 0..0),
        _ => parser(Tokens::new(&tokens, tokens[0].extra.0)).expect("Unrecovered error happened in parser").1
    };
    //let ex = (Expr::Dollar, 0..1);
    ex
}
