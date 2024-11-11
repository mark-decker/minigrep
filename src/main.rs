use std::env;
use std::fs;
use std::process;

fn check_args(v: Option<&String>) -> &String  {
    if let None = v {
        println!("minigrep requires two arguments, a search query and the file path");
        process::exit(1);
    }
    v.unwrap()  //return the unwrapped String ref
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let query: Option<&String> = args.get(1);
    let checked_query = check_args(query);

    let file_path: Option<&String> = args.get(2);
    let checked_file_path = check_args(file_path);

    let file_contents = fs::read_to_string(checked_file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{file_contents}");
    dbg!(args);
}
