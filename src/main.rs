use std::env;

fn check_args(v: Option<&String>)  {
    match v {
        Some(v) => println!("Found argument {v}"),
        None => panic!("minigrep requires both search string and file path as arguments"),
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    //println!("args length is {}",args.len());
    //if args.len() < 3 {
    //    panic!("minigrep requires the search query and file to be specified");
    //}
    //we should not hard code with [1] to prevent runtime error
    //let query = &args[1];
    //let file_path = &args[2];

    let query: Option<&String> = args.get(1);
    check_args(query);
    //match query {
    //    Some(query) => println!("Query is {query}"),
    //    None => panic!("minigrep requires a query"),
    //}

    let file_path: Option<&String> = args.get(2);
    check_args(file_path);
    //match file_path {
    //    Some(file_path) => println!("Filepath is {file_path}"),
    //    None => panic!("minigrep requires a filepath"),
    //}

    dbg!(args);
}
