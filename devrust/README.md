
---

```rs
use std::env;
fn main() {
  let args: Vec<String> = env::args().collect();
  dbg!(args);
}

```

```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/minigrep]
└─$ cargo run text filename.txt
   Compiling minigrep v0.1.0 (/home/kali/projects/weekly67/devrust/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 2.47s
     Running `target/debug/minigrep text filename.txt`
[src/main.rs:4] args = [
    "target/debug/minigrep",
    "text",
    "filename.txt",
]
```

---

```rs
use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for [{}]", query);
    println!("In file [{}]", file_path);
}
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/minigrep]
└─$ cargo run -- text filename
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/minigrep text filename`
Searching for [text]
In file [filename]
```

---

```rs
use std::env;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for: {} ", query);
    println!("In file      : {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file."); //std::io::Result<String>
    println!("With text: \n{contents}");   
}

```

---

```rs
use std::env;
use std::fs;
fn main(){
  let args: Vec<String> = env::args().collect();
  let (query, file_path) = parse_config(&args);
    println!("Searching for: {} ", query);
    println!("In file      : {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file.");
    println!("With text: \n{contents}");   
}
fn parse_config(args: &[String]) -> (&str, &str){
  let query = &args[1];
  let file_path = &args[2];
  (query, file_path)
}

```

---


```rs
use std::env;
use std::fs;

fn main(){
  let args: Vec<String> = env::args().collect();
  let config = parse_config(&args);
  println!("Searching for  :{}", config.query);
  println!("In file        :{}", config.file_path);
  let contents = fs::read_to_string(config.file_path)
                 .expect("Should have been able to read the file");
  println!("With text: \n{contents}");   
}
struct Config{
  query: String,
  file_path: String,
}
fn parse_config(args: &[String]) -> Config {
  let query = args[1].clone();
  let file_path = args[2].clone();
  Config {query, file_path}
}

```
---


```rs
fn main(){
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args);
  println!("Searching for  :{}", config.query);
  println!("In file        :{}", config.file_path);
  let contents = fs::read_to_string(config.file_path)
                 .expect("Should have been able to read the file");
  println!("With text: \n{contents}");   
}
struct Config{
  query: String,
  file_path: String,
}
impl Config{
  fn new(args: &[String]) -> Config{
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config {query, file_path}
  }
}

```

---

```rs
use std::env;
use std::fs;
use std::process;

fn main(){
  let args: Vec<String> = env::args().collect();
  //let config = Config::new(&args);
  let config = Config::build(&args).unwrap_or_else(|err| {
    println!("Problem passing arguments {err}");
    process::exit(1);
  });
  println!("Searching for  :{}", config.query);
  println!("In file        :{}", config.file_path);
  let contents = fs::read_to_string(config.file_path)
                 .expect("Should have been able to read the file");
  println!("With text: \n{contents}");   
}
struct Config{
  query: String,
  file_path: String,
}
// warning: associated function `new` is never used
//impl Config{
//  fn new(args: &[String]) -> Config{
//    if args.len() < 3 {
//      panic!("Not enough arguments");
//    }
//    let query = args[1].clone();
//    let file_path = args[2].clone();
//    Config {query, file_path}
//  }
//}
impl Config{
  fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();
    Ok(Config { query, file_path })
  }
}
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/minigrep]
└─$ cargo run  --
   Compiling minigrep v0.1.0 (/home/kali/projects/weekly67/devrust/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.74s
     Running `target/debug/minigrep`
Problem passing arguments not enough arguments
```

--- 


```rs
use std::env;
use std::fs;
use std::process;
use std::error::Error;
fn main(){
  let args: Vec<String> = env::args().collect();
  let config = Config::build(&args).unwrap_or_else(|err| {
    println!("Problem passing arguments {err}");
    process::exit(1);
  });
  println!("Searching for  :{}", config.query);
  println!("In file        :{}", config.file_path);
  if let Err(e) = run(config) {
    println!("Application Error: {e}");
    process::exit(1);
  }
}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;
  println!("With text:\n{contents}");
  Ok(())
}
struct Config{
  query: String,
  file_path: String,
}
impl Config{
  fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();
    Ok(Config { query, file_path })
  }
}

```
```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/minigrep]
└─$ cargo run -- file poem.txt
   Compiling minigrep v0.1.0 (/home/kali/projects/weekly67/devrust/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.86s
     Running `target/debug/minigrep file poem.txt`
Searching for  :file
In file        :poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!

                                                                                                                                                                   
┌──(kali㉿kali)-[~/projects/weekly67/devrust/minigrep]
└─$ cargo run --              
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/minigrep`
Problem passing arguments not enough arguments
```

---

```rs
// src/main.rs
use std::env;
use std::process;
use minigrep::Config;
fn main(){
  let args: Vec<String> = env::args().collect();
  let config = Config::build(&args).unwrap_or_else(|err| {
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

```

```rs
// src/lib.rs
use std::error::Error;
use std::fs;
pub struct Config{
  pub query: String,
  pub file_path: String,
}
impl Config{
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();
    Ok(Config { query, file_path })
  }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;
  println!("With text:\n{contents}");
  Ok(())
}
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/minigrep]
└─$ cargo run --
   Compiling minigrep v0.1.0 (/home/kali/projects/weekly67/devrust/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 1.33s
     Running `target/debug/minigrep`
Problem passing arguments not enough arguments
                                                                                                                                                                   
┌──(kali㉿kali)-[~/projects/weekly67/devrust/minigrep]
└─$ cargo run -- file peom.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/minigrep file peom.txt`
Searching for  :file
In file        :peom.txt
Application Error: No such file or directory (os error 2)
                                                                                                                                                                   
┌──(kali㉿kali)-[~/projects/weekly67/devrust/minigrep]
└─$ cargo run -- fils poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/minigrep fils poem.txt`
Searching for  :fils
In file        :poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

---

```rs
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//  vec![]
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.contains(query){
      results.push(line);
    }
  }
  results
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn one_result(){
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick here.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
}
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/minigrep]
└─$ cargo test
   Compiling minigrep v0.1.0 (/home/kali/projects/weekly67/devrust/minigrep)
    Finished test [unoptimized + debuginfo] target(s) in 1.18s
     Running unittests src/lib.rs (target/debug/deps/minigrep-61eb4d646ceaf9af)
running 1 test
test tests::one_result ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests src/main.rs (target/debug/deps/minigrep-3cfce48fd3b2ccf4)
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   Doc-tests minigrep
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---


---

