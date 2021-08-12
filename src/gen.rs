pub mod pure;

#[cfg(test)]
pub mod tests {
  use quickcheck::{
    Arbitrary,
    Gen,
  };

  use crate::name::{
    is_valid_symbol_char,
    Name,
  };

  use sp_std::{
    boxed::Box,
    ops::Range,
    ptr::NonNull,
  };

  pub fn gen_range(g: &mut Gen, range: Range<usize>) -> usize {
    if range.end <= range.start {
      range.start
    }
    else {
      let res: usize = Arbitrary::arbitrary(g);
      (res % (range.end - range.start)) + range.start
    }
  }

  pub fn arbitrary_name(g: &mut Gen) -> Name {
    let s: String = Arbitrary::arbitrary(g);
    let mut s: String = s
      .chars()
      .filter(|x| is_valid_symbol_char(*x) && char::is_ascii_alphabetic(x))
      .collect();
    s.truncate(1);
    Name::from(format!("_{}", s))
  }
  #[inline]
  pub fn alloc_val<T>(val: T) -> NonNull<T> {
    NonNull::new(Box::leak(Box::new(val))).unwrap()
  }
}
