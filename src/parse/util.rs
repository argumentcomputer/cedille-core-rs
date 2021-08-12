use crate::{
  name::{
    is_valid_symbol_string,
    Name,
  },
  parse::{
    error::{
      ParseError,
      ParseErrorKind,
    },
    span::Span,
  },
  uses::Uses,
};
use nom::{
  branch::alt,
  bytes::complete::{
    tag,
    take_till,
    take_till1,
  },
  character::complete::{
    multispace0,
    multispace1,
  },
  combinator::{
    eof,
    peek,
    value,
  },
  multi::many0,
  sequence::terminated,
  Err,
  IResult,
};

use sp_cid::Cid;
use sp_im::vector::Vector;
use sp_ipld::{
  dag_cbor::DagCborCodec,
  Codec,
};
use sp_multihash::{
  Code,
  MultihashDigest,
};

use alloc::string::String;

use sp_std::{
  borrow::ToOwned,
  vec::Vec,
};

pub type Ctx = Vector<Name>;

pub fn reserved_symbols() -> Vector<String> {
  Vector::from(vec![
    String::from("//"),
    String::from("λ"),
    String::from("∀"),
    String::from("lambda"),
    String::from("forall"),
    String::from("⁰"),
    String::from("¹"),
    String::from("˚"),
    String::from("⁺"),
    String::from("=>"),
    String::from("->"),
    String::from("="),
    String::from(";"),
    String::from("::"),
    String::from("let"),
    String::from("in"),
    String::from("type"),
    String::from("data"),
    String::from("self"),
    String::from("def"),
    String::from("case"),
    String::from("Type"),
  ])
}
pub fn parse_line_comment(i: Span) -> IResult<Span, Span, ParseError<Span>> {
  let (i, _) = tag("//")(i)?;
  let (i, com) = take_till(|c| c == '\n')(i)?;
  Ok((i, com))
}
pub fn parse_space(i: Span) -> IResult<Span, Vec<Span>, ParseError<Span>> {
  let (i, _) = multispace0(i)?;
  let (i, com) = many0(terminated(parse_line_comment, multispace1))(i)?;
  Ok((i, com))
}
pub fn parse_space1(i: Span) -> IResult<Span, Vec<Span>, ParseError<Span>> {
  let (i, _) = multispace1(i)?;
  let (i, com) = many0(terminated(parse_line_comment, multispace1))(i)?;
  Ok((i, com))
}

pub fn input_cid(i: &str) -> Cid {
  Cid::new_v1(
    0x55,
    Code::Blake2b256.digest(
      DagCborCodec.encode(&i.to_owned()).unwrap().into_inner().as_ref(),
    ),
  )
}
pub fn parse_uses() -> impl Fn(Span) -> IResult<Span, Uses, ParseError<Span>> {
  move |i: Span| {
    alt((
      value(Uses::None, tag("⁰")),
      value(Uses::Once, tag("¹")),
      value(Uses::Affi, tag("˚")),
      value(Uses::Many, tag("⁺")),
    ))(i)
  }
}

pub fn is_numeric_symbol_string1(s: &str) -> bool {
  s.starts_with('0')
    || s.starts_with('1')
    || s.starts_with('2')
    || s.starts_with('3')
    || s.starts_with('4')
    || s.starts_with('5')
    || s.starts_with('6')
    || s.starts_with('7')
    || s.starts_with('8')
    || s.starts_with('9')
}
pub fn is_numeric_symbol_string2(s: &str) -> bool {
  s.starts_with("-0")
    || s.starts_with("-1")
    || s.starts_with("-2")
    || s.starts_with("-3")
    || s.starts_with("-4")
    || s.starts_with("-5")
    || s.starts_with("-6")
    || s.starts_with("-7")
    || s.starts_with("-8")
    || s.starts_with("-9")
    || s.starts_with("+0")
    || s.starts_with("+1")
    || s.starts_with("+2")
    || s.starts_with("+3")
    || s.starts_with("+4")
    || s.starts_with("+5")
    || s.starts_with("+6")
    || s.starts_with("+7")
    || s.starts_with("+8")
    || s.starts_with("+9")
}

pub fn parse_name(from: Span) -> IResult<Span, Name, ParseError<Span>> {
  let (i, s) = take_till1(|x| {
    char::is_whitespace(x)
      | (x == ':')
      | (x == ';')
      | (x == ')')
      | (x == '(')
      | (x == '{')
      | (x == '}')
      | (x == ',')
  })(from)?;
  let s: String = String::from(s.fragment().to_owned());
  if reserved_symbols().contains(&s) {
    Err(Err::Error(ParseError::new(from, ParseErrorKind::ReservedKeyword(s))))
  }
  else if s.starts_with('#') {
    Err(Err::Error(ParseError::new(from, ParseErrorKind::ReservedSyntax(s))))
  }
  else if is_numeric_symbol_string1(&s) | is_numeric_symbol_string2(&s) {
    Err(Err::Error(ParseError::new(from, ParseErrorKind::NumericSyntax(s))))
  }
  else if !is_valid_symbol_string(&s) {
    Err(Err::Error(ParseError::new(from, ParseErrorKind::InvalidSymbol(s))))
  }
  else {
    Ok((i, Name::from(s)))
  }
}

pub fn parse_tele_end(i: Span) -> IResult<Span, (), ParseError<Span>> {
  let (i, _) = alt((
    peek(tag("def")),
    peek(tag("type")),
    peek(tag("in")),
    peek(tag("=")),
    peek(tag("->")),
    peek(tag(";")),
    peek(tag(")")),
    peek(tag("{")),
    peek(tag("}")),
    peek(tag(",")),
    peek(eof),
  ))(i)?;
  Ok((i, ()))
}
