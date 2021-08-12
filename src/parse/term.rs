// use crate::{
//  kind::{
//    BaseKind,
//    Kind,
//  },
//  name::{
//    is_valid_symbol_string,
//    Name,
//  },
//  parse::{
//    error::{
//      ParseError,
//      ParseErrorKind,
//    },
//    span::Span,
//  },
//  position::Pos,
//  term::{
//    Term,
//    Tm,
//    Uses,
//  },
//};
// use alloc::string::{
//  String,
//  ToString,
//};
// use nom::{
//  branch::alt,
//  bytes::complete::{
//    tag,
//    take_till,
//    take_till1,
//  },
//  character::complete::{
//    digit1,
//    multispace0,
//    multispace1,
//    satisfy,
//  },
//  combinator::{
//    eof,
//    peek,
//    value,
//  },
//  error::context,
//  multi::{
//    many0,
//    many1,
//  },
//  sequence::{
//    delimited,
//    preceded,
//    terminated,
//  },
//  Err,
//  IResult,
//};
// use sp_cid::Cid;
// use sp_im::vector::Vector;
// use sp_ipld::{
//  dag_cbor::DagCborCodec,
//  Codec,
//};
// use sp_multihash::{
//  Code,
//  MultihashDigest,
//};
// use sp_std::{
//  borrow::ToOwned,
//  boxed::Box,
//  vec::Vec,
//};
//
//// pub fn parse_pure_var() -> impl Fn(Span) -> IResult<Span, Uses,
//// ParseError<Span>>
//// {
////  move |i: Span| {
////    alt((
////      value(Uses::None, terminated(tag("⁰"), multispace1)),
////      value(Uses::Once, terminated(tag("¹"), multispace1)),
////      value(Uses::Affi, terminated(tag("˚"), multispace1)),
////      value(Uses::Many, terminated(tag("⁺"), multispace1)),
////    ))(i)
////  }
//// }
//// pub fn parse_var(
////  input: Cid,
////  defs: Defs,
////  rec: Option<Name>,
////  ctx: Ctx,
//// ) -> impl Fn(Span) -> IResult<Span, Term, ParseError<Span>> {
////  move |from: Span| {
////    let (upto, nam) = context("local or global reference",
//// parse_name)(from)?;    let pos = Pos::from_upto(input, from, upto);
////    let is_rec_name = match rec.clone() {
////      Some(rec_ref) => nam == rec_ref,
////      _ => false,
////    };
////    if let Some((idx, _)) = ctx.iter().enumerate().find(|(_, x)| **x == nam)
//// {      Ok((upto, Term::Var(pos, nam.clone(), idx as u64)))
////    }
////    else if is_rec_name {
////      Ok((upto, Term::Rec(pos)))
////    }
////    else if let Some(def) = defs.get(&nam) {
////      Ok((upto, Term::Ref(pos, nam.clone(), def.def_cid, def.ast_cid)))
////    }
////    else {
////      Err(Err::Error(ParseError::new(
////        upto,
////        ParseErrorKind::UndefinedReference(nam.clone(), ctx.clone()),
////      )))
////    }
////  }
//// }
// pub fn parse_uses() -> impl Fn(Span) -> IResult<Span, Uses, ParseError<Span>>
// {  move |i: Span| {
//    alt((
//      value(Uses::None, tag("⁰")),
//      value(Uses::Once, tag("¹")),
//      value(Uses::Affi, tag("˚")),
//      value(Uses::Many, tag("⁺")),
//    ))(i)
//  }
//}
//
////#[cfg(test)]
//// pub mod tests {
////  use super::*;
////
////  #[test]
////  fn test_parse_kind() {
////    fn test<const T: Tm>(
////      i: &str,
////    ) -> IResult<Span, BaseKind<T>, ParseError<Span>> {
////      parse_kind(input_cid(i))(Span::new(i))
////    }
////
////    let res = test("Π Type -> Type");
////    assert!(res.is_ok());
////    let res: Kind = res.unwrap().1;
////    assert_eq!(
////      res,
////      Kind::PiKd(
////        Pos::None,
////        Box::new(Kind::Type(Pos::None)),
////        Box::new(Kind::Type(Pos::None))
////      )
////    );
////  }
//// }
