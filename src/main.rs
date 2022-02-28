use std::env;
use std::process;
use pyth::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    err_args(&args,1);
    
    match args[1].as_str() {
        "theo" => {err_args(&args, 2);pythagoras(&args[2],&args[3],&args[4])},
        "-t" => println!("Work in progress..."),
        "help" => println!("This programs has a specific syntax, for accurate results please follow the syntax.\nIf you don't know how to use the program check out the documentation by adding the \"docs\" argument.\n-t\tThales theorem\n-p\tPythagorean theorem\ndocs\tOpens up the documentation\nhelp\tDisplays valid arguments\nKeep in mind that this tool is case-sensitive, be sure to write arguments all in lowercase."),
        "docs" => println!("This programs assumes you know the theorem and respect the conditions, if you don't, learn them to get accurate results.\n\nPythagorean theorem argument syntax = pyth theo [SIDE-NAME*=[SIDE-LENGTH]*] [SIDE-NAME*=[SIDE-LENGTH]*] [HYP-NAME*=[HYP-LENGTH]*]\n\nIf one side has no length, it will be calculated.\nIf all the sides contain each their length, the program assume you're trying to prove if the triangle is right-angled or not.\n\nNote that eveyrthing between \"* *\" is optional and the results are accurate if the rules are followed.\nRULES:\nThere must be at least two sides (hypotenuse included) with a specified length.\nThe hypotenuse (the longest side) must be the last argument, otherwise the results are inaccurate.\nAll the lengths must be the same  unit.\nThe hypotenuse at the power of 2 must be equal or superior to the sum of both sides (both to the power of 2).\nThe triangle must be right-angled if you're trying to calculate a side's length.\n\nIf any of the rules above aren't followed, the program will either display inaccurate results or exit.\n\nExamples:\npyth theo A=5 B C=9\tOutputs the length of C with detailed steps.\npyth theo I=9 B=4.5 C=12.7\tVerifies if the triangle is righ-angled.\n"), 
        _ => {println!("ERROR: Invalid argument, type \"help\" to display valid ones.");process::exit(1)}
    }
}

