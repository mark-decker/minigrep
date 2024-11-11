//refactor minigrep to use this library file
use std::{fs, error::Error};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("minigrep requires a search query and file path")
        }

        let query = args[1].clone();  //use clone and return error to let caller handle them
        let file_path = args[2].clone();

        return Ok(Config {
            query ,
            file_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let file_contents = fs::read_to_string(config.file_path.clone())?;  //will return the error for caller
                                                                       //to handle
    //the entire file contents is in the string
    //split into a vector with each line as an str in the vector
    let file_lines: Vec<&str> = file_contents.split("\n").collect();

    //iterator over each line looking for the query
    let mut found: bool = false;
    for line in file_lines {
        if line.contains(config.query.as_str()) {
            println!("{}",line);
            found = true;
        }
    }

    if !found {
        println!("{} is not present in {}",config.query, config.file_path);
    }

    Ok(())  //return Ok result type

}

