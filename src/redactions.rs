

use std::process;
use crate::err_handlers::*;



pub fn redaction_sides(a: Vec<&str>, b: Vec<&str>, hyp: Vec<&str>) {
    let num_a = err_parse(&a);
    let num_b: f32 = err_parse(&b);

    println!("{2}²={0}²+{1}²\n{2}²={3}²+{4}²\n{2}²={5}+{6}\n{7}={5}+{6}\n{2}=√{7}",a[0],b[0],hyp[0],num_a,num_b,num_a.powf(2.0),num_b.powf(2.0),num_a.powf(2.0) + num_b.powf(2.0))
    
}



pub fn redaction_hyp(a: Vec<&str>, b: Vec<&str>, hyp: Vec<&str>) {
    if a.len() > 1 && hyp.len() > 1 {
        let num_a: f32 = err_parse(&a);
        let num_hyp: f32 = err_parse(&hyp);   
        
        if num_hyp < num_a {
            println!("ERROR: The hypotenuse's length is inferior to {}'s",a[0]);
            process::exit(1)
        }

        println!("{2}²={0}²+{1}²\n{num_hyp}²={num_a}²+{1}²\n{1}²={num_hyp}²-{num_a}²\n{1}²={3}-{4}\n{5}={3}-{4}\n{1}=√{5}",a[0],b[0],hyp[0],num_hyp.powf(2.0),num_a.powf(2.0),(num_hyp.powf(2.0))-(num_a.powf(2.0)))

    }
    else if b.len() > 1 && hyp.len() > 1 {
        let num_b: f32 = err_parse(&b);
        let num_hyp: f32 = err_parse(&hyp);   
        
        if num_hyp < num_b {
            println!("ERROR: The hypotenuse's length is inferior to {}'s",b[0]);
            process::exit(1)
        }

        println!("{2}²={0}²+{1}²\n{num_hyp}²={0}²+{num_b}²\n{0}²={num_hyp}²-{num_b}²\n{0}²={3}-{4}\n{5}={3}-{4}\n{0}=√{4}",a[0],b[0],hyp[0],num_hyp.powf(2.0),num_b.powf(2.0),(num_hyp.powf(2.0))-(num_b.powf(2.0)))
    }
    else {
        println!("ERROR: Make sure only two sides has a value.");
        process::exit(1)
    }
}


pub fn proof(a: Vec<&str>, b: Vec<&str>, hyp: Vec<&str>) {
    let num_a: f32 = err_parse(&a);
    let num_b: f32 = err_parse(&b);
    let num_hyp: f32 = err_parse(&hyp);

    println!("On one hand: {}²={num_hyp}²={}",hyp[0],num_hyp.powf(2.0));
    println!("On another hand: {0}²+{1}²={num_a}²+{num_b}²={2}+{3}={4}",a[0],a[1],num_a.powf(2.0),num_b.powf(2.0),num_a.powf(2.0)+num_b.powf(2.0));

    if num_a.powf(2.0)+num_b.powf(2.0) == num_hyp.powf(2.0)  {
        println!("According to the Pythagorean Theorem, the triangle is right-angled.")
    }
        else {
        println!("According to the Pythagorean Theorem, the triangle isn't right-angled.")
    }
}
