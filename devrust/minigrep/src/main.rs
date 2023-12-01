// src/main.rs
use std::env;
use std::process;
use minigrep::Config;
fn main(){
  let args: Vec<String> = env::args().collect();
  let config = Config::build(&    args).unwrap_or_else(|err| {
    println!("Problem passing arguments {err}");
    process::exit(1);
  });
  println!("Searching for  :{}", config.query);
  println!("In file        :{}", config.file_path);
  if let Err(e) = minigrep::run(config) {
    println!("Application Error: {e}");
    process::exit(1);
  }
}

//
//fn main(){
//  let args: Vec<String> = env::args().collect();
//  //let config = Config::new(&args);
//  let config = Config::build(&args).unwrap_or_else(|err| {
//    println!("Problem passing arguments {err}");
//    process::exit(1);
//  });
//  println!("Searching for  :{}", config.query);
//  println!("In file        :{}", config.file_path);
//  let contents = fs::read_to_string(config.file_path)
//                 .expect("Should have been able to read the file");
//  println!("With text: \n{contents}");   
//}
//struct Config{
//  query: String,
//  file_path: String,
//}
//// warning: associated function `new` is never used
////impl Config{
////  fn new(args: &[String]) -> Config{
////    if args.len() < 3 {
////      panic!("Not enough arguments");
////    }
////    let query = args[1].clone();
////    let file_path = args[2].clone();
////    Config {query, file_path}
////  }
////}
//impl Config{
//  fn build(args: &[String]) -> Result<Config, &'static str> {
//    if args.len() < 3 {
//      return Err("not enough arguments");
//    }
//    let query = args[1].clone();
//    let file_path = args[2].clone();
//    Ok(Config { query, file_path })
//  }
//}
//

//fn main(){
//  let args: Vec<String> = env::args().collect();
//  let config = parse_config(&args);
//  println!("Searching for  :{}", config.query);
//  println!("In file        :{}", config.file_path);
//  let contents = fs::read_to_string(config.file_path)
//                 .expect("Should have been able to read the file");
//  println!("With text: \n{contents}");   
//}
//struct Config{
//  query: String,
//  file_path: String,
//}
//fn parse_config(args: &[String]) -> Config {
//  let query = args[1].clone();
//  let file_path = args[2].clone();
//  Config {query, file_path}
//}

//fn main(){
//  let args: Vec<String> = env::args().collect();
//  let (query, file_path) = parse_config(&args);
//    println!("Searching for: {} ", query);
//    println!("In file      : {}", file_path);
//    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file.");
//    println!("With text: \n{contents}");   
//}
//fn parse_config(args: &[String]) -> (&str, &str){
//  let query = &args[1];
//  let file_path = &args[2];
//  (query, file_path)
//}

//fn main(){
//    let args: Vec<String> = env::args().collect();
//    let query = &args[1];
//    let file_path = &args[2];
//    println!("Searching for: {} ", query);
//    println!("In file      : {}", file_path);
//    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file.");
//    println!("With text: \n{contents}");   
//}

//fn main(){
//    let args: Vec<String> = env::args().collect();
//    let query = &args[1];
//    let file_path = &args[2];
//    println!("Searching for [{}]", query);
//    println!("In file [{}]", file_path);
//}

//fn main() {
//  let args: Vec<String> = env::args().collect();
//  dbg!(args);
//}

