use std::process;
mod redactions;
use redactions::{redaction_hyp,redaction_sides,proof};
mod err_handlers;
pub use err_handlers::*;

pub fn pythagoras(a: &String, b: &String, hyp: &String) {
    let a: Vec<&str> = a.split("=").collect();
    let b: Vec<&str> = b.split("=").collect();
    let hyp: Vec<&str> = hyp.split("=").collect();

    
    if a.len() == 2 && b.len() == 2 && hyp.len() == 2 {
        proof(a, b, hyp)
    }
    else if a.len() == 2 && b.len() == 2 {
        redaction_sides(a, b, hyp)
    }
  
    else if a.len() == 2 && hyp.len() == 2 {
        redaction_hyp(a, b, hyp)
    }
  
    else if b.len() == 2 && hyp.len() == 2 {
        redaction_hyp(a, b, hyp)
    }

    else {
        println!("ERROR: Make sure you're respecting the argument syntax.");
        process::exit(1)
    }

}

