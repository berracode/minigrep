use std::fs;

pub struct Params{
    pub filename: String,
    pub query: String
}

impl Params {

    pub fn new(args: &[String]) -> Params{
        let filename = args[1].clone();
        let query = args[2].clone();
        Params{filename,query}
    }
}

pub fn run(params: &Params){
    let file = &(params.filename);
    let content_file = fs::read_to_string(file).expect("We can't open file");
    let found_lines = search(&params.query, &content_file);

    for line in found_lines {
        println!("line {}", line);
    }
}

fn search<'a>(query: &str, content_file: &'a str) -> Vec<&'a str>{
    let mut found_lines = Vec::new();

    for i in content_file.lines() {
        if i.contains(query){
            found_lines.push(i);
        }
    }

    found_lines

}