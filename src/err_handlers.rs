use std::process;

pub fn err_parse(var: &Vec<&str>) -> f32 {
    let var: f32 = match var[1].parse() {
            Ok(k) => k,
            Err(_) => {println!("ERROR: Expected an integer value for {}, found {}",var[0],var[1]);process::exit(1)}
    };
    var
}

pub fn err_args(args: &Vec<String>,case: i32) {
    //case 1
    if args.len() < 2 && case == 1 {
        println!("ERROR: Not enough arguments.");
        process::exit(1)}
    

    //case 2    
    else if args.len() > 5 && case == 2 {
        println!("ERROR: Too many arguments.");
        process::exit(1)}
    else if args.len() < 5 && case == 2 {
        println!("ERROR: Not enough arguments.");
        process::exit(1)}
    }