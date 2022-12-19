use std::fs;
use std::{process};


pub struct Params{
    pub filename: String,
    pub query: String
}

impl Params {

    pub fn new(args: &[String]) -> Result<Params, &'static str> {
        if args.len() == 3 {
            let filename = args[1].clone();
            let query = args[2].clone();
            Ok(Params{filename,query})
            
        }else {
            return Err("Some arguments not found");
        }
        
    }
}

pub fn run(params: &Params){
    let file = &(params.filename);
    let content_file = fs::read_to_string(file).unwrap_or_else(|err|{
        eprintln!("ERROR: {err}");
        process::exit(0);
    });
    let found_lines = search(&params.query, &content_file);
    println!("found_lines: {}", found_lines.len());
    for line in found_lines {
        println!("line {}", line);
    }
}

fn search<'a>(query: &str, content_file: &'a str) -> Vec<&'a str>{
    content_file.lines().filter(|line| line.contains(query)).collect()
}

