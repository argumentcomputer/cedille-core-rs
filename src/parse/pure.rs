use crate::{
  parse::{
    error::{
      ParseError,
      ParseErrorKind,
    },
    span::Span,
    util::{
      parse_name,
      parse_space,
      parse_tele_end,
      Ctx,
    },
  },
  position::Pos,
  pure::Pure,
};
use nom::{
  branch::alt,
  bytes::complete::tag,
  combinator::peek,
  error::context,
  multi::many1,
  sequence::{
    delimited,
    preceded,
    terminated,
  },
  Err,
  IResult,
};
use sp_cid::Cid;
use sp_std::{
  boxed::Box,
  vec::Vec,
};

pub fn parse_var(
  input: Cid,
  ctx: Ctx,
) -> impl Fn(Span) -> IResult<Span, Pure, ParseError<Span>> {
  move |from: Span| {
    let (upto, nam) = context("local or global reference", parse_name)(from)?;
    let pos = Pos::from_upto(input, from, upto);
    if let Some((idx, _)) = ctx.iter().enumerate().find(|(_, x)| **x == nam) {
      Ok((upto, Pure::Var(pos, nam.clone(), idx as u64)))
    }
    else {
      Err(Err::Error(ParseError::new(
        upto,
        ParseErrorKind::UndefinedReference(nam.clone(), ctx.clone()),
      )))
    }
  }
}

pub fn parse_lam(
  input: Cid,
  ctx: Ctx,
) -> impl Fn(Span) -> IResult<Span, Pure, ParseError<Span>> {
  move |from: Span| {
    let (i, _) = tag("λ")(from)?;
    let (i, _) = parse_space(i)?;
    let (i, bs) = many1(preceded(parse_space, parse_name))(i)?;
    let (i, _) = parse_space(i)?;
    let (i, _) = terminated(tag("=>"), parse_space)(i)?;
    let mut ctx2 = ctx.clone();
    for b in bs.iter() {
      ctx2.push_front(b.clone());
    }
    let (upto, bod) = parse_telescope(input, ctx2)(i)?;
    let pos = Pos::from_upto(input, from, upto);
    let trm =
      bs.into_iter().rev().fold(bod, |acc, n| Pure::Lam(pos, n, Box::new(acc)));
    Ok((upto, trm))
  }
}

pub fn parse_args(
  input: Cid,
  ctx: Ctx,
) -> impl FnMut(Span) -> IResult<Span, Vec<Pure>, ParseError<Span>> {
  move |mut i: Span| {
    let mut res = Vec::new();

    loop {
      match preceded(parse_space, peek(parse_tele_end))(i) {
        Ok((i2, _)) => return Ok((i2, res)),
        _ => {}
      }
      match preceded(parse_space, parse_pure(input, ctx.clone()))(i) {
        Err(e) => return Err(e),
        Ok((i2, x)) => {
          res.push(x);
          i = i2;
        }
      }
    }
  }
}

pub fn parse_telescope(
  input: Cid,
  ctx: Ctx,
) -> impl Fn(Span) -> IResult<Span, Pure, ParseError<Span>> {
  move |from: Span| {
    let (i, fun) = context("app fun", parse_pure(input, ctx.clone()))(from)?;
    let (i, _) = parse_space(i)?;
    let (upto, args) = parse_args(input, ctx.clone())(i)?;
    let pos = Pos::from_upto(input, from, upto);
    let trm = args
      .into_iter()
      .fold(fun, |acc, arg| Pure::App(pos, Box::new(acc), Box::new(arg)));
    return Ok((upto, trm));
  }
}

pub fn parse_pure(
  input: Cid,
  ctx: Ctx,
) -> impl Fn(Span) -> IResult<Span, Pure, ParseError<Span>> {
  move |i: Span| {
    alt((
      context(
        "Pure application telescope",
        delimited(
          preceded(tag("("), parse_space),
          parse_telescope(input, ctx.clone()),
          preceded(parse_space, tag(")")),
        ),
      ),
      context("Pure lambda", parse_lam(input, ctx.clone())),
      context("Pure variable", parse_var(input, ctx.clone())),
    ))(i)
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::{
    name::Name,
    parse::util::input_cid,
  };

  #[test]
  fn test_parse_pure() {
    fn test(i: &str) -> IResult<Span, Pure, ParseError<Span>> {
      parse_pure(input_cid(i), Ctx::new())(Span::new(i))
    }

    let res = test("λ x y z => x");
    // println!("{:?}", res);
    assert!(res.is_ok());
    let res: Pure = res.unwrap().1;
    assert_eq!(
      res,
      Pure::Lam(
        Pos::None,
        Name::from("x"),
        Box::new(Pure::Lam(
          Pos::None,
          Name::from("y"),
          Box::new(Pure::Lam(
            Pos::None,
            Name::from("z"),
            Box::new(Pure::Var(Pos::None, Name::from("x"), 2u64))
          ))
        ))
      ),
    );
    let res = test("λ f a b => f a b");
    println!("{:?}", res);
    assert!(res.is_ok());
    let res: Pure = res.unwrap().1;
    assert_eq!(
      res,
      Pure::Lam(
        Pos::None,
        Name::from("f"),
        Box::new(Pure::Lam(
          Pos::None,
          Name::from("a"),
          Box::new(Pure::Lam(
            Pos::None,
            Name::from("b"),
            Box::new(Pure::App(
              Pos::None,
              Box::new(Pure::App(
                Pos::None,
                Box::new(Pure::Var(Pos::None, Name::from("f"), 2u64)),
                Box::new(Pure::Var(Pos::None, Name::from("a"), 1u64))
              )),
              Box::new(Pure::Var(Pos::None, Name::from("b"), 0u64))
            ))
          ))
        ))
      )
    );
  }
}
