use crate::{
  name::Name,
  position::Pos,
  print::pure,
};

use sp_std::{
  boxed::Box,
  fmt,
};

/// pure terms in the untyped lambda calculus
#[derive(Clone)]
pub enum Pure {
  /// variable: x
  Var(Pos, Name, u64),
  /// lambda abstraction: λ x => x
  Lam(Pos, Name, Box<Pure>),
  /// application: f x
  App(Pos, Box<Pure>, Box<Pure>),
}

impl fmt::Debug for Pure {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Var(_, n, i) => fmt.debug_tuple("Var").field(&n).field(i).finish(),
      Self::Lam(_, n, b) => fmt.debug_tuple("Lam").field(&n).field(&b).finish(),
      Self::App(_, f, a) => fmt.debug_tuple("App").field(&f).field(&a).finish(),
    }
  }
}

impl PartialEq for Pure {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Var(_, na, ia), Self::Var(_, nb, ib)) => na == nb && ia == ib,
      (Self::Lam(_, na, ba), Self::Lam(_, nb, bb)) => na == nb && ba == bb,
      (Self::App(_, fa, aa), Self::App(_, fb, ab)) => fa == fb && aa == ab,
      _ => false,
    }
  }
}

impl fmt::Display for Pure {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", pure::print(false, self))
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::gen::pure::{
    tests::arbitrary_pure,
    Pure as GenPure,
  };
  use quickcheck::{
    Arbitrary,
    Gen,
  };
  use sp_im::Vector;
  use sp_std::mem;

  impl Arbitrary for Pure {
    fn arbitrary(g: &mut Gen) -> Self {
      let gen_pure = arbitrary_pure(g, Vector::new());
      unsafe { mem::transmute::<GenPure, Pure>(gen_pure) }
    }
  }
}
