//minigrep
//simple grep implementation in Rust
//Learning project by Mark Decker
//11/11/2024
//
use std::{env, fs, process, error::Error};

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

    if let Err(e) = run(config) {
        println!("Runtime error: {e}");
        process::exit(2);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let file_contents = fs::read_to_string(config.file_path)?;  //will return the error for caller
                                                                //to handle
    println!("With text:\n{file_contents}");

    println!("The query is {}",config.query);

    Ok(())  //return Ok result type 

}
