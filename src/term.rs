use crate::{
  kind::Kind,
  name::Name,
  position::Pos,
  pure::Pure,
  typ_::{
    PureType,
    Type,
  },
  uses::Uses,
};

use sp_std::{
  boxed::Box,
  fmt,
};

/// purity marker for Types
pub type Tm = bool;

/// type-annotated terms
#[derive(Clone)]
pub enum Term {
  /// variable: x
  Var(Pos, Name, u64),
  /// lambda abstraction: λ (⁰x: T) => x, λ (0 x: T) => x
  Lam(Pos, Uses, Name, Box<Type>, Box<Term>),
  /// typed lambda abstraction: λ (x: Type) => x
  LamTy(Pos, Name, Box<Kind>, Box<Term>),
  /// application of term to term: f ⁺x, f ¹x, f ˚x, f ⁰x
  App(Pos, Uses, Box<Term>, Box<Term>),
  /// application of term to type, f T
  AppTy(Pos, Box<Term>, Box<Type>),
  /// dependent intersection, `intersect x y T`
  Intersect(Pos, Box<Term>, Box<Term>, Box<Type>),
  /// Let-term, let ⁰x = y in B
  Let(Pos, Uses, Name, Box<Term>, Box<Term>),
  /// Let-type, let T: K = A in B
  LetTy(Pos, Name, Box<Kind>, Box<Type>, Box<Term>),
  /// dep.intersection left projection `projectL x`
  ProjectL(Pos, Box<Term>),
  /// dep.intersection right projection `projectR x`
  ProjectR(Pos, Box<Term>),
  /// proof of x ≃ x which erases to y, `refl x y`
  Refl(Pos, Box<Pure>, Box<Pure>),
  /// symmetry of equality, `sym x`
  Sym(Pos, Box<Term>),
  /// anything by absurd equality, `absurd T x`
  Absurd(Pos, Box<Type>, Box<Term>),
  /// rewrite by equality, `rewrite x T y`
  Rewrite(Pos, Box<Term>, Box<PureType>, Box<Term>),
  /// cast by equality, `cast x y z`
  Cast(Pos, Box<Term>, Box<Term>, Box<Pure>),
}

impl fmt::Debug for Term {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Var(_, n, i) => fmt.debug_tuple("Var").field(&n).field(i).finish(),
      Self::Lam(_, u, n, t, b) => {
        fmt.debug_tuple("Lam").field(&u).field(&n).field(&t).field(&b).finish()
      }
      Self::LamTy(_, n, t, b) => {
        fmt.debug_tuple("LamTy").field(&n).field(&t).field(&b).finish()
      }
      Self::App(_, u, f, a) => {
        fmt.debug_tuple("App").field(&u).field(&f).field(&a).finish()
      }
      Self::AppTy(_, f, a) => {
        fmt.debug_tuple("AppTy").field(&f).field(&a).finish()
      }
      Self::Intersect(_, t, a, b) => {
        fmt.debug_tuple("Intersect").field(&t).field(&a).field(&b).finish()
      }
      Self::Let(_, u, n, a, b) => {
        fmt.debug_tuple("Let").field(&u).field(&n).field(&a).field(&b).finish()
      }
      Self::LetTy(_, n, k, a, b) => fmt
        .debug_tuple("LetTy")
        .field(&n)
        .field(&k)
        .field(&a)
        .field(&b)
        .finish(),
      Self::ProjectL(_, x) => fmt.debug_tuple("ProjectL").field(&x).finish(),
      Self::ProjectR(_, x) => fmt.debug_tuple("ProjectR").field(&x).finish(),
      Self::Refl(_, x, y) => {
        fmt.debug_tuple("Refl").field(&x).field(&y).finish()
      }
      Self::Sym(_, x) => fmt.debug_tuple("Sym").field(&x).finish(),
      Self::Absurd(_, t, x) => {
        fmt.debug_tuple("Absurd").field(&t).field(&x).finish()
      }
      Self::Rewrite(_, x, p, y) => {
        fmt.debug_tuple("Rewrite").field(&x).field(&p).field(&y).finish()
      }
      Self::Cast(_, x, y, p) => {
        fmt.debug_tuple("Cast").field(&x).field(&y).field(&p).finish()
      }
    }
  }
}

impl PartialEq for Term {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Var(_, na, ia), Self::Var(_, nb, ib)) => na == nb && ia == ib,
      (Self::Lam(_, ua, na, ta, ba), Self::Lam(_, ub, nb, tb, bb)) => {
        ua == ub && na == nb && ta == tb && ba == bb
      }
      (Self::App(_, ua, fa, aa), Self::App(_, ub, fb, ab)) => {
        ua == ub && fa == fb && aa == ab
      }
      (Self::AppTy(_, fa, aa), Self::AppTy(_, fb, ab)) => fa == fb && aa == ab,
      (Self::Intersect(_, xa, ya, ta), Self::Intersect(_, xb, yb, tb)) => {
        xa == xb && ya == yb && ta == tb
      }
      (Self::Let(_, ua, na, xa, ba), Self::Let(_, ub, nb, xb, bb)) => {
        ua == ub && na == nb && xa == xb && ba == bb
      }
      (Self::LetTy(_, na, ka, xa, ba), Self::LetTy(_, nb, kb, xb, bb)) => {
        na == nb && xa == xb && ka == kb && ba == bb
      }
      (Self::ProjectL(_, xa), Self::ProjectL(_, xb)) => xa == xb,
      (Self::ProjectR(_, xa), Self::ProjectR(_, xb)) => xa == xb,
      (Self::Refl(_, xa, ea), Self::Refl(_, xb, eb)) => xa == xb && ea == eb,
      (Self::Sym(_, xa), Self::Sym(_, xb)) => xa == xb,
      (Self::Absurd(_, ta, xa), Self::Absurd(_, tb, xb)) => {
        ta == tb && xa == xb
      }
      (Self::Rewrite(_, xa, ta, ya), Self::Rewrite(_, xb, tb, yb)) => {
        xa == xb && ta == tb && ya == yb
      }
      (Self::Cast(_, xa, ya, za), Self::Cast(_, xb, yb, zb)) => {
        xa == xb && ya == yb && za == zb
      }
      _ => false,
    }
  }
}
