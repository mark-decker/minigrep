use std::env;
use std::fs;

fn check_args(v: Option<&String>) -> &String  {
    match v {
        Some(v) => println!(""),
        None => panic!("minigrep requires both search string and file path as arguments"),
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
