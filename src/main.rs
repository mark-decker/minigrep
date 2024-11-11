//minigrep
//simple grep implementation in Rust
//Learning project by Mark Decker
//11/11/2024
//
use std::{env, fs, process};

struct Config {
    query: String,
    file_path: String,
}

//return a Config storing query and file_path 
//Check to ensure the input slice has both the query and file_path
//exit program with either or both are missing
fn parse_args(args: &[String]) -> Config {
    let first: Option<&String> = args.get(1);
    let second: Option<&String> = args.get(2);

    Config {
        query: check_args(first),
        file_path: check_args(second),
    }
}

fn check_args(v: Option<&String>) -> String  {
    if let None = v {
        println!("minigrep requires two arguments, a search query and the file path");
        process::exit(1);
    }
    v.unwrap().to_string()  //return the unwrapped String
}


fn main() {
    let args: Vec<String> = env::args().collect();

    //Parse the input arguments
    let config = parse_args(&args[..]);  //pass the slice not of the whole vec

    //Verify file exists before attempting to read it
    if !fs::exists(config.file_path.clone()).unwrap() {
        println!("The file {} does not exist",config.file_path);
        process::exit(2);
    }

    let file_contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{file_contents}");

    println!("The query is {}",config.query);

}
