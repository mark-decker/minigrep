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

    let file_contents = fs::read_to_string(config.file_path)?;  //will return the error for caller
                                                                       //to handle
    let results = search(&config.query, &file_contents);

    for found in results {
        println!("{found}");
    }

    Ok(())  //return Ok result type

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in contents.split("\n") {
        if line.contains(&query) {
            results.push(line);
        }
    }

    return results
}

