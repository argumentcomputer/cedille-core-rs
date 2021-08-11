#![feature(const_generics)]

#[macro_use]
extern crate alloc;

pub mod name;
pub mod term;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
