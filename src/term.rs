use sp_std::collections::btree_map;

use crate::name::Name;
use sp_std::{
  boxed::Box,
  marker::PhantomData,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Uses {
  None,
  Many,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tm {
  Pure,
  Term,
}

enum Pure {
  Var(u64),
  Lam(Box<Pure>),
  App(Box<Pure>, Box<Pure>),
}

enum Term {
  /// variable
  Var(u64),
  /// lambda abstraction
  Lam(Uses, Box<Type>, Box<Term>),
  /// typed lambda abstraction
  LamTy(Box<Kind>, Box<Term>),
  /// application of term to term
  AppTm(Uses, Box<Term>, Box<Term>),
  /// application of term to type
  AppTy(Box<Term>, Box<Type>),
  /// dependent intersection
  Iota(Box<Term>, Box<Term>, Box<Type>),
  /// Let term
  LetTm(Uses, Box<Term>, Box<Term>),
  /// Let type
  LetTp(Box<Kind>, Box<Type>, Box<Term>),
  /// dep.intersection left projection
  Proj1(Box<Term>),
  /// dep.intersection right projection
  Proj2(Box<Term>),
  /// proof of t' ~ t' which erases to t
  Beta(Box<Pure>, Box<Pure>),
  /// symmetry of equality
  Sigma(Box<Term>),
  /// anything by absurd equality
  Delta(Box<Type>, Box<Term>),
  /// rewrite by equality
  Rho(Box<Term>, Box<PrType>, Box<Term>),
  /// cast by equality
  Phi(Box<Term>, Box<Term>, Box<Pure>),
}

enum BaseType<const T: Tm> {
  /// Type variable
  Var(u64),
  /// Type abstraction
  Lam(Box<BaseType<T>>, Box<BaseType<T>>),
  /// Kind abstraction
  LamTy(Box<BaseKind<T>>, Box<BaseType<T>>),
  /// dependent product
  Pi(Uses, Box<BaseType<T>>, Box<BaseType<T>>),
  PiTy(Box<BaseKind<T>>, Box<BaseType<T>>),
  Iota(Box<BaseType<T>>, Box<BaseType<T>>),
  Eq(Box<Pure>, Box<Pure>),
  AppTp(Box<BaseType<T>>, Box<BaseType<T>>),
}

enum BaseKind<const T: Tm> {
  Star,
  Pi(Box<BaseType<T>>, Box<BaseKind<T>>),
  PiTy(Box<BaseKind<T>>, Box<BaseKind<T>>),
}

type Type = BaseType<{ Tm::Term }>;
type Kind = BaseKind<{ Tm::Term }>;

type PrType = BaseType<{ Tm::Pure }>;
type PrKind = BaseKind<{ Tm::Pure }>;
