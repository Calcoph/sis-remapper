use std::ops::Range;

use nom::{error::ParseError, Parser};

use crate::token::{Spanned, StrResult, ToRange};

pub fn map_with_span<I, O1, O2, E: ParseError<I>, F, G>(
    mut parser: F,
    mut mapper: G
) -> impl FnMut(I) -> StrResult<I, O2, E>
where
  F: Parser<I, O1, E>,
  I: ToRange + Copy,
  G: FnMut(O1, Range<usize>) -> O2,
{
    move |i: I| {
        let (remaining, o) = parser.parse(i)?;
        let span = i.consumed_span(remaining.span().start);

        Ok((remaining, mapper(o, span)))
    }
}

pub fn spanned<I, O, E: ParseError<I>, F>(
    mut parser: F
) -> impl FnMut(I) -> StrResult<I, Spanned<O>, E>
where
  F: Parser<I, O, E>,
  I: ToRange + Copy
{
    move |i: I| {
        let (remaining, o) = parser.parse(i)?;
        let span = i.consumed_span(remaining.span().start);

        Ok((remaining, (o, span)))
    }
}
