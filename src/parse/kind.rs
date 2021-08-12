use crate::{
  kind::BaseKind,
  parse::{
    error::ParseError,
    span::Span,
    util::parse_space,
  },
  position::Pos,
  term::Tm,
  uses::Uses,
};
use nom::{
  branch::alt,
  bytes::complete::tag,
  combinator::value,
  error::context,
  multi::many1,
  sequence::{
    delimited,
    preceded,
  },
  IResult,
};
use sp_cid::Cid;
use sp_std::boxed::Box;

pub fn parse_kind_type<const T: Tm>(
  input: Cid,
) -> impl Fn(Span) -> IResult<Span, BaseKind<T>, ParseError<Span>> {
  move |from: Span| {
    let (upto, _) = tag("Type")(from)?;
    let pos = Pos::from_upto(input, from, upto);
    Ok((upto, BaseKind::<T>::Type(pos)))
  }
}

/// Parses `Π Type -> Type`
pub fn parse_kind_pi_ty<const T: Tm>(
  input: Cid,
) -> impl Fn(Span) -> IResult<Span, BaseKind<T>, ParseError<Span>> {
  move |from: Span| {
    let (i, _) = tag("Π")(from)?;
    let (i, ks) = many1(preceded(parse_space, parse_kind(input)))(i)?;
    let (i, _) = parse_space(i)?;
    let (i, _) = tag("->")(i)?;
    let (i, _) = parse_space(i)?;
    let (upto, bod) = parse_kind(input)(i)?;
    let pos = Pos::from_upto(input, from, upto);
    let kind = ks
      .into_iter()
      .fold(bod, |acc, k| BaseKind::<T>::PiTy(pos, Box::new(k), Box::new(acc)));
    Ok((upto, kind))
  }
}
pub fn parse_kind<const T: Tm>(
  input: Cid,
) -> impl Fn(Span) -> IResult<Span, BaseKind<T>, ParseError<Span>> {
  move |i: Span| {
    alt((
      context(
        "parenthesized kind",
        delimited(
          preceded(tag("("), parse_space),
          parse_kind(input),
          preceded(parse_space, tag(")")),
        ),
      ),
      context("Type kind", parse_kind_type(input)),
      context("Π kind", parse_kind_pi_ty(input)),
    ))(i)
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::{
    kind::Kind,
    parse::util::input_cid,
  };

  #[test]
  fn test_parse_kind() {
    fn test<const T: Tm>(
      i: &str,
    ) -> IResult<Span, BaseKind<T>, ParseError<Span>> {
      parse_kind(input_cid(i))(Span::new(i))
    }

    let res = test("Π Type -> Type");
    assert!(res.is_ok());
    let res: Kind = res.unwrap().1;
    assert_eq!(
      res,
      Kind::PiTy(
        Pos::None,
        Box::new(Kind::Type(Pos::None)),
        Box::new(Kind::Type(Pos::None))
      )
    );
    let res = test("Π (Π Type -> Type) -> Type");
    assert!(res.is_ok());
    let res: Kind = res.unwrap().1;
    assert_eq!(
      res,
      Kind::PiTy(
        Pos::None,
        Box::new(Kind::PiTy(
          Pos::None,
          Box::new(Kind::Type(Pos::None)),
          Box::new(Kind::Type(Pos::None))
        )),
        Box::new(Kind::Type(Pos::None))
      )
    );
    let res = test::<false>("Π (Π Type -> Type) Type Type -> Type");
    assert!(res.is_ok());
  }
}
