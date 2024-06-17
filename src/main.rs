use std::{env, process};

use untitled1::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments:{err}");
        process::exit(1);
    });
    println!("query:{:?}", config.query);
    println!("file path:{:?}", config.file_path);
    if let Err(e) = run(config) {
        println!("Application err {e}");
        process::exit(1);
    }

}

