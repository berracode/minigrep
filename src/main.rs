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

#[test]
#[should_panic]
fn params_not_found() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    let params_test = Params::new(&args);

    mini_grep::run(&params_test);
}

#[test]
#[should_panic]
fn file_does_not_exist(){
    let params_test = Params{
        filename: ".\\mini_grep.imel".to_string(),
        query: "so".to_string()
    };

    mini_grep::run(&params_test);
}


#[test]
fn file_exist(){
    let params_test = Params{
        filename: ".\\mini_grep.iml".to_string(),
        query: "so".to_string()
    };

    mini_grep::run(&params_test);
}

