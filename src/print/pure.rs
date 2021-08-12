use crate::pure::Pure;

use alloc::string::{
  String,
  ToString,
};

fn print_name(nam: &str) -> &str { if nam.is_empty() { "_" } else { nam } }

fn is_atom(term: &Pure) -> bool { matches!(term, Pure::Var(..)) }

fn print_parens(ind: bool, term: &Pure) -> String {
  if is_atom(term) {
    print(ind, term)
  }
  else {
    format!("({})", print(ind, term))
  }
}

fn print_lambdas(ind: bool, name: &str, bod: &Pure) -> String {
  match bod {
    Pure::Lam(_, bod_nam, bod_bod) => {
      format!("{} {}", print_name(name), print_lambdas(ind, bod_nam, bod_bod))
    }
    _ => format!("{} => {}", print_name(name), print(ind, bod)),
  }
}

fn print_applications(ind: bool, fun: &Pure) -> String {
  match fun {
    Pure::App(_, f_fun, f_arg) => {
      format!("{} {}", print_applications(ind, f_fun), print_parens(ind, f_arg),)
    }
    _ => format!("{}", print_parens(ind, fun)),
  }
}

pub fn print(ind: bool, term: &Pure) -> String {
  match term {
    Pure::Var(_, nam, index) => {
      if ind {
        format!("{}^{}", nam, index)
      }
      else {
        nam.to_string()
      }
    }
    Pure::Lam(_, name, bod) => {
      format!("λ {}", print_lambdas(ind, &name.to_string(), bod))
    }
    Pure::App(_, fun, arg) => {
      format!("{} {}", print_applications(ind, fun), print_parens(ind, arg))
    }
  }
}
