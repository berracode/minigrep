use std::env;
use mini_grep::Params;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    let params = Params::new(&args);

    mini_grep::run(&params);
    println!("file {}", params.filename);
    println!("query {}", params.query);

}
