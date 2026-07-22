use std::env;

mod init;
mod repository;

fn main() {
    let args: Vec<String> = env::args().collect();
    // args.push(String::from("init")); // just for vs code debug
    if args.len() < 2 {
        panic!("At least 1 argument are necessary \nHelp: without a query I don't know what to do!");
    }
    let query = args[1].as_str();
    let output = match query {
        "init" => init::init(),
        _ => panic!("Query not recognized! Try one of the following: init"),
    };
    if let Err(init::InitError::OS(error)) = &output {
        println!("There was an error: {error}");
    }
    println!("{query} resulted in: {output:?}");
}
