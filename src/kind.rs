use crate::{
  position::Pos,
  term::Tm,
  typ_::BaseType,
};

use sp_std::{
  boxed::Box,
  fmt,
};

/// Kind former for pure and annotated terms
#[derive(Clone)]
pub enum BaseKind<const T: Tm> {
  /// Type
  Type(Pos),
  /// Type constructor, `Π T -> Type`
  Pi(Pos, Box<BaseType<T>>, Box<BaseKind<T>>),
  /// Type arrow `Π Type -> Type`
  PiTy(Pos, Box<BaseKind<T>>, Box<BaseKind<T>>),
}

/// The Kind of Types of Pure terms
pub type PureKind = BaseKind<true>;
/// The Kind of Types of annotated Terms
pub type Kind = BaseKind<false>;

impl<const T: Tm> fmt::Debug for BaseKind<T> {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Type(_) => fmt.debug_tuple("Type").finish(),
      Self::Pi(_, t, b) => fmt.debug_tuple("Pi").field(&t).field(&b).finish(),
      Self::PiTy(_, k, b) => {
        fmt.debug_tuple("PiTy").field(&k).field(&b).finish()
      }
    }
  }
}

impl<const T: Tm> PartialEq for BaseKind<T> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Type(_), Self::Type(_)) => true,
      (Self::Pi(_, ta, ba), Self::Pi(_, tb, bb)) => ta == tb && ba == bb,
      (Self::PiTy(_, ka, ba), Self::PiTy(_, kb, bb)) => ka == kb && ba == bb,
      _ => false,
    }
  }
}
