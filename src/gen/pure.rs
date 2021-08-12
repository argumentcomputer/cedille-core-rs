use sp_std::{
  mem::MaybeUninit,
  ptr::NonNull,
};

use crate::{
  name::Name,
  position::Pos,
};

#[derive(Debug)]
pub enum Pure {
  Var(Pos, Name, u64),
  Lam(Pos, Name, NonNull<MaybeUninit<Pure>>),
  App(Pos, NonNull<MaybeUninit<Pure>>, NonNull<MaybeUninit<Pure>>),
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::gen::tests::{
    alloc_val,
    arbitrary_name,
    gen_range,
  };
  use sp_im::Vector;
  use sp_std::boxed::Box;

  use quickcheck::Gen;

  #[derive(Debug, Clone, Copy)]
  pub enum Case {
    VAR,
    LAM,
    APP,
  }

  pub fn next_case(g: &mut Gen, gens: &Vec<(usize, Case)>) -> Case {
    let sum: usize = gens.iter().map(|x| x.0).sum();
    let mut weight: usize = gen_range(g, 1..sum);
    for gen in gens {
      match weight.checked_sub(gen.0) {
        None | Some(0) => {
          return gen.1;
        }
        _ => {
          weight -= gen.0;
        }
      }
    }
    panic!("Calculation error for weight = {}", weight);
  }

  pub fn arbitrary_pure(g: &mut Gen, ctx0: Vector<Name>) -> Pure {
    let res = alloc_val(MaybeUninit::<Pure>::uninit());
    let mut stack = vec![(ctx0, res.clone())];
    let _term = Box::new(res);
    while let Some((ctx, mut ptr)) = stack.pop() {
      let depth = ctx.len();
      let mut gens: Vec<(usize, Case)> = vec![
        (90usize.saturating_sub(depth), Case::LAM),
        (80usize.saturating_sub(2 * depth), Case::APP),
      ];
      if ctx.len() != 0 {
        gens.push((100, Case::VAR))
      };
      match next_case(g, &gens) {
        Case::VAR => {
          if ctx.len() == 0 {
          }
          else {
            let gen = gen_range(g, 0..ctx.len());
            let n = &ctx[gen];
            let (i, _) = ctx.iter().enumerate().find(|(_, x)| *x == n).unwrap();
            unsafe {
              *ptr.as_mut() =
                MaybeUninit::new(Pure::Var(Pos::None, n.clone(), i as u64));
            }
          }
        }
        Case::LAM => {
          let n = arbitrary_name(g);
          let mut ctx2 = ctx.clone();
          ctx2.push_front(n.clone());
          let bod = alloc_val(MaybeUninit::<Pure>::uninit());
          stack.push((ctx2, bod));
          unsafe {
            *ptr.as_mut() =
              MaybeUninit::new(Pure::Lam(Pos::None, n.clone(), bod));
          }
        }
        Case::APP => {
          let fun = alloc_val(MaybeUninit::<Pure>::uninit());
          let arg = alloc_val(MaybeUninit::<Pure>::uninit());
          stack.push((ctx.clone(), fun));
          stack.push((ctx, arg));
          unsafe {
            *ptr.as_mut() = MaybeUninit::new(Pure::App(Pos::None, fun, arg));
          }
        }
      }
    }
    unsafe {
      let pure = Box::from_raw(res.as_ptr());
      let pure = pure.assume_init();
      Box::into_inner(pure)
    }
  }
}
