use crate::{
  kind::BaseKind,
  name::Name,
  position::Pos,
  pure::Pure,
  term::Tm,
  uses::Uses,
};
use sp_std::{
  boxed::Box,
  fmt,
};

/// Type former for pure and annotated terms
#[derive(Clone)]
pub enum BaseType<const T: Tm> {
  /// Type variable
  Var(Pos, Name, u64),
  /// Type abstraction
  Lam(Pos, Name, Box<BaseType<T>>, Box<BaseType<T>>),
  /// Kind abstraction
  LamTy(Pos, Name, Box<BaseKind<T>>, Box<BaseType<T>>),
  /// dependent product, ∀ (⁰x: A) -> B
  Pi(Pos, Uses, Name, Box<BaseType<T>>, Box<BaseType<T>>),
  /// dependent product, ∀ (x: Type) -> B
  PiTy(Pos, Name, Box<BaseKind<T>>, Box<BaseType<T>>),
  /// ι x: A. B
  Iota(Pos, Name, Box<BaseType<T>>, Box<BaseType<T>>),
  /// x ≃ y
  Eql(Pos, Box<Pure>, Box<Pure>),
  /// f x
  AppTy(Pos, Box<BaseType<T>>, Box<BaseType<T>>),
}

/// The Type of Pure terms
pub type PureType = BaseType<true>;
/// The Type of annotated Terms
pub type Type = BaseType<false>;

impl<const T: Tm> fmt::Debug for BaseType<T> {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Var(_, n, i) => fmt.debug_tuple("Var").field(&n).field(i).finish(),
      Self::Lam(_, n, t, b) => {
        fmt.debug_tuple("Lam").field(&n).field(&t).field(&b).finish()
      }
      Self::LamTy(_, n, k, b) => {
        fmt.debug_tuple("LamTy").field(&n).field(&k).field(&b).finish()
      }
      Self::Pi(_, u, n, t, b) => {
        fmt.debug_tuple("Pi").field(&u).field(&n).field(&t).field(&b).finish()
      }
      Self::PiTy(_, n, k, b) => {
        fmt.debug_tuple("PiTy").field(&n).field(&k).field(&b).finish()
      }
      Self::Iota(_, n, a, b) => {
        fmt.debug_tuple("Iota").field(&n).field(&a).field(&b).finish()
      }
      Self::Eql(_, a, b) => fmt.debug_tuple("Eql").field(&a).field(&b).finish(),
      Self::AppTy(_, f, a) => {
        fmt.debug_tuple("AppTy").field(&f).field(&a).finish()
      }
    }
  }
}

impl<const T: Tm> PartialEq for BaseType<T> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Var(_, na, ia), Self::Var(_, nb, ib)) => na == nb && ia == ib,
      (Self::Lam(_, na, ta, ba), Self::Lam(_, nb, tb, bb)) => {
        na == nb && ta == tb && ba == bb
      }
      (Self::LamTy(_, na, ka, ba), Self::LamTy(_, nb, kb, bb)) => {
        na == nb && ka == kb && ba == bb
      }
      (Self::Pi(_, ua, na, ta, ba), Self::Pi(_, ub, nb, tb, bb)) => {
        ua == ub && na == nb && ta == tb && ba == bb
      }
      (Self::PiTy(_, na, ka, ba), Self::PiTy(_, nb, kb, bb)) => {
        na == nb && ka == kb && ba == bb
      }
      (Self::Iota(_, na, ka, ba), Self::Iota(_, nb, kb, bb)) => {
        na == nb && ka == kb && ba == bb
      }
      (Self::Eql(_, aa, ba), Self::Eql(_, ab, bb)) => aa == ab && ba == bb,
      (Self::AppTy(_, fa, aa), Self::AppTy(_, fb, ab)) => fa == fb && aa == ab,
      _ => false,
    }
  }
}
