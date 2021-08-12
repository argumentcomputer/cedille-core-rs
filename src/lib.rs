#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(test, feature(new_uninit))]
#![cfg_attr(test, feature(box_into_inner))]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
#[cfg(test)]
extern crate rand;

#[macro_use]
extern crate alloc;

pub mod gen;
pub mod ipld_error;
pub mod kind;
pub mod name;
pub mod parse;
pub mod position;
pub mod print;
pub mod pure;
pub mod term;
pub mod typ_;
pub mod uses;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
