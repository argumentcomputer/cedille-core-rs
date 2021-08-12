#![cfg_attr(not(any(feature = "std", test)), no_std)]

#[macro_use]
extern crate alloc;

pub mod ipld_error;
pub mod kind;
pub mod name;
pub mod parse;
pub mod position;
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
