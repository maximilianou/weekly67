
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
  for line  in search(&config.query, &contents){
    println!("{line}");
  }
  Ok(())
}

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
└─$ cargo run -- public poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/minigrep public poem.txt`
Searching for  :public
In file        :poem.txt
How public, like a frog
                           
```

---


```rs
// src/main.rs
use std::env;
use std::process;
use minigrep::Config;
fn main(){
  let args: Vec<String> = env::args().collect();
  let config = Config::build(&    args).unwrap_or_else(|err| {
    eprintln!("Problem passing arguments {err}");
    process::exit(1);
  });
  println!("Searching for  :{}", config.query);
  println!("In file        :{}", config.file_path);
  if let Err(e) = minigrep::run(config) {
    eprintln!("Application Error: {e}");
    process::exit(1);
  }
}
```

---

### Functional Programming with Rust

```rs
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
  Red,
  Blue,
}
struct Inventory {
  shirts: Vec<ShirtColor>,
}

impl Inventory {
  fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocked())
  }
  fn most_stocked(&self) -> ShirtColor {
    let mut num_red = 0;
    let mut num_blue = 0;
    for color in &self.shirts {
      match color {
        ShirtColor::Red => num_red += 1,
        ShirtColor::Blue => num_blue += 1,
      }
    }
    if num_red > num_blue {
      ShirtColor::Red
    } else {
      ShirtColor::Blue
    }
  }
}

fn main() {
  let store = Inventory {
    shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
  };
  let user_pref1 = Some(ShirtColor::Red);
  let giveaway1 = store.giveaway(user_pref1);
  println!(
    "The user with preference {:?} gets {:?}",
    user_pref1, giveaway1
  );
  let user_pref2 = None;
  let giveaway2 = store.giveaway(user_pref2);
  println!(
    "The user with preference {:?} gets {:?}",
    user_pref2, giveaway2 
  );

}

```

---

- Smart Pointers

Box<T>



```rs
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T>{
    MyBox(x)
  }
}
///////////////////////////

struct CustomSmartPointer {
  data: String,
}
impl Drop for CustomSmartPointer {
  fn drop(&mut self){
    println!("Dropping CustomerSmartPointer with data: {}", self.data);
  }
}

pub fn custom_smart_test(){
  let c = CustomSmartPointer {
    data: String::from("Prima Cosa"),
  };
  let d = CustomSmartPointer {
    data: String::from("Seconda cosa"),
  };
  let e = CustomSmartPointer {
    data: String::from("Tersa cosa"),
  };
  drop(e);
  println!("CustomSmartPointer created.");
} 
```
---


```rs
// concur/src/lib.rs
use std::thread;
use std::time::Duration;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn concur_step_01(){
    let handle = thread::spawn(|| {
      for _i in 1..10 {
        println!("Spawned thread: {}", _i);
        thread::sleep(Duration::from_millis(1));
      }
    });
    handle.join().unwrap();
}
pub fn concur_step_02(){
    for _i in 1..5 {
        println!("Principal thread: {}", _i);
        thread::sleep(Duration::from_millis(1));
    }
}
pub fn concur_step_03(){
    let handle = thread::spawn(|| {
      for _i in 1..10 {
        println!("Spawned thread: {}", _i);
        thread::sleep(Duration::from_millis(1));
      }
    });
    for _i in 1..5 {
        println!("Principal thread: {}", _i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

```

```rs
// concur/src/main.rs
use concur::{add, concur_step_01, concur_step_02, concur_step_03};
fn main(){
  println!(" {}", add(2,3));
  println!("-- ");
  concur_step_01();
  println!("-- ");
  concur_step_02();
  println!("-- ");
  concur_step_03();
  println!("-- ");
}
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/concur]
└─$ cargo run 
   Compiling concur v0.1.0 (/home/kali/projects/weekly67/devrust/concur)
    Finished dev [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/concur`
 5
-- 
Spawned thread: 1
Spawned thread: 2
Spawned thread: 3
Spawned thread: 4
Spawned thread: 5
Spawned thread: 6
Spawned thread: 7
Spawned thread: 8
Spawned thread: 9
-- 
Principal thread: 1
Principal thread: 2
Principal thread: 3
Principal thread: 4
-- 
Principal thread: 1
Spawned thread: 1
Principal thread: 2
Spawned thread: 2
Principal thread: 3
Spawned thread: 3
Principal thread: 4
Spawned thread: 4
Spawned thread: 5
Spawned thread: 6
Spawned thread: 7
Spawned thread: 8
Spawned thread: 9
-- 

```

---



```rs
use std::sync::mpsc;
pub fn concur_step_05(){
  let (tx, rx) = mpsc::channel(); // tx Transmiter, rx Receiver
  thread::spawn(move || {
    let val = String::from("Message! Hi");
    tx.send(val).unwrap();
  });
  let received = rx.recv().unwrap();
  println!("Got: {}", received);
}
```


---


```rs
use std::sync::Arc;
pub fn concur_step_10(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn( move || {
            let mut num = counter.lock().unwrap();
            *num +=1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result! {}", *counter.lock().unwrap());
}
```


---

```rs
pub trait Draw {
    fn draw(&self);
}
pub struct Screen<T: Draw>{
  pub components: Vec<T>,
}
impl<T> Screen<T>
where T: Draw, {
  pub fn run(&self){
    for components in self.components.iter() {
        components.draw();
    }
  }
}

```

---


```rs
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```
---

