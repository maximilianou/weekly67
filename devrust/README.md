
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

---


- cargo generate 

> cargo-generate is a developer tool to help you get up and running quickly 
> with a new Rust project by leveraging a > pre-existing git repository as a template.

https://github.com/cargo-generate/cargo-generate


```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/frontend-1]
└─$ cargo install cargo-generate
```

```sh

   Compiling lock_api v0.4.11
error: failed to run custom build command for `openssl-sys v0.9.97`

Caused by:
  process didn't exit successfully: `/tmp/cargo-installKHrU63/release/build/openssl-sys-030156b7c8d3444f/build-script-main` (exit status: 101)

  run pkg_config fail: Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS="1" "pkg-config" "--libs" "--cflags" "openssl"`
  The pkg-config command could not be found.

  Most likely, you need to install a pkg-config package for your OS.
  Try `apt install pkg-config`, or `yum install pkg-config`,
  or `pkg install pkg-config`, or `apk add pkgconfig` depending on your distribution.

```

```sh

┌──(kali㉿kali)-[~/projects/weekly67/devrust/frontend-1]
└─$ sudo apt install pkg-config
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/frontend-1]
└─$ cargo install cargo-generate
```

```sh
  run pkg_config fail: `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS="1" "pkg-config" "--libs" "--cflags" "openssl"` did not exit successfully: exit status: 1
  error: could not find system library 'openssl' required by the 'openssl-sys' crate

  --- stderr
  Package openssl was not found in the pkg-config search path.
  Perhaps you should add the directory containing `openssl.pc'


  Could not find directory of OpenSSL installation, and this `-sys` crate cannot
  proceed without this knowledge. If OpenSSL is installed and this crate had
  trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
  compilation process.

  Make sure you also have the development packages of openssl installed.
  For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.

```

```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/frontend-1]
└─$ apt search libssl-dev
Sorting... Done
Full Text Search... Done
libssl-dev/kali-rolling 3.0.11-1 amd64
  Secure Sockets Layer toolkit - development files


┌──(kali㉿kali)-[~/projects/weekly67/devrust/frontend-1]
└─$ apt search libssl-dev     
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/frontend-1]
└─$ cargo install cargo-generate


   Compiling cargo-generate v0.19.0
    Finished release [optimized] target(s) in 10m 46s
  Installing /home/kali/.cargo/bin/cargo-generate
   Installed package `cargo-generate v0.19.0` (executable `cargo-generate`)
```



https://yew.rs/docs/getting-started/build-a-sample-app


```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/frontend-1]
└─$ cargo generate --git https://github.com/yewstack/yew-trunk-minimal-template 
🤷   Project Name: webrust
🔧   Destination: /home/kali/projects/weekly67/devrust/frontend-1/webrust ...
🔧   project-name: webrust ...
🔧   Generating template ...
🔧   Moving generated files into: `/home/kali/projects/weekly67/devrust/frontend-1/webrust`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /home/kali/projects/weekly67/devrust/frontend-1/webrust
```



```sh
┌──(kali㉿kali)-[~/…/weekly67/devrust/frontend-1/webrust]
└─$ cargo run                

   Compiling trunk-template v0.1.0 (/home/kali/projects/weekly67/devrust/frontend-1/webrust)
    Finished dev [unoptimized + debuginfo] target(s) in 1m 45s
     Running `target/debug/trunk-template`
thread 'main' panicked at /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/js-sys-0.3.64/src/lib.rs:5847:9:
cannot call wasm-bindgen imported functions on non-wasm targets
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

```sh
┌──(kali㉿kali)-[~/…/weekly67/devrust/frontend-1/webrust]
└─$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/trunk-template`
thread 'main' panicked at /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/js-sys-0.3.64/src/lib.rs:5847:9:
cannot call wasm-bindgen imported functions on non-wasm targets
stack backtrace:
   0: std::panicking::begin_panic
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/std/src/panicking.rs:627:12
   1: js_sys::global::get_global_object::Global::get_self::__wbg_self_1ff1d729e9aae938
             at /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/js-sys-0.3.64/src/lib.rs:5847:9
   2: js_sys::global::get_global_object::Global::get_self
             at /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/js-sys-0.3.64/src/lib.rs:5847:9
   3: js_sys::global::get_global_object
             at /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/js-sys-0.3.64/src/lib.rs:5866:29
   4: js_sys::global::GLOBAL::__init
             at /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/js-sys-0.3.64/src/lib.rs:5825:43
   5: js_sys::global::GLOBAL::__getit::{{closure}}
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/std/src/sys/common/thread_local/fast_local.rs:99:25
   6: std::sys::common::thread_local::lazy::LazyKeyInner<T>::initialize
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/std/src/sys/common/thread_local/mod.rs:54:25
   7: std::sys::common::thread_local::fast_local::Key<T>::try_initialize
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/std/src/sys/common/thread_local/fast_local.rs:190:27
   8: std::sys::common::thread_local::fast_local::Key<T>::get
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/std/src/sys/common/thread_local/fast_local.rs:173:25
   9: js_sys::global::GLOBAL::__getit
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/std/src/sys/common/thread_local/fast_local.rs:91:21
  10: std::thread::local::LocalKey<T>::try_with
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/std/src/thread/local.rs:269:32
  11: std::thread::local::LocalKey<T>::with
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/std/src/thread/local.rs:246:9
  12: js_sys::global
             at /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/js-sys-0.3.64/src/lib.rs:5827:12
  13: web_sys::window
             at /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/web-sys-0.3.64/src/lib.rs:31:5
  14: gloo_utils::window
             at /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/gloo-utils-0.1.7/src/lib.rs:14:5
  15: gloo_utils::document
             at /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/gloo-utils-0.1.7/src/lib.rs:26:5
  16: yew::renderer::Renderer<COMP>::with_props
             at /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/yew-0.20.0/src/renderer.rs:76:13
  17: <yew::renderer::Renderer<COMP> as core::default::Default>::default
             at /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/yew-0.20.0/src/renderer.rs:49:9
  18: yew::renderer::Renderer<COMP>::new
             at /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/yew-0.20.0/src/renderer.rs:60:9
  19: trunk_template::main
             at ./src/main.rs:6:5
  20: core::ops::function::FnOnce::call_once
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```


https://github.com/rustwasm/wasm-bindgen/issues/2859#issuecomment-1100208359


> This is because you're trying to run your app in a non-wasm environment. Remember, cargo run is not used to run a 
> wasm app. Depending upon what your app is, you can use either wasm-pack or trunk to build and run it. Also see the
> wasm-bindgen book.



```sh
┌──(kali㉿kali)-[~/…/weekly67/devrust/frontend-1/webrust]
└─$ cargo install trunk    

   Compiling trunk v0.18.2
    Finished release [optimized] target(s) in 14m 55s
  Installing /home/kali/.cargo/bin/trunk
   Installed package `trunk v0.18.2` (executable `trunk`)
```


```sh
┌──(kali㉿kali)-[~/…/weekly67/devrust/frontend-1/webrust]
└─$ trunk serve

   Compiling cfg-if v1.0.0
error[E0463]: can't find crate for `core`
  |
  = note: the `wasm32-unknown-unknown` target may not be installed
  = help: consider downloading the target with `rustup target add wasm32-unknown-unknown`

error[E0463]: can't find crate for `compiler_builtins`

For more information about this error, try `rustc --explain E0463`.
error: could not compile `cfg-if` (lib) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
2023-12-20T10:13:04.790679Z ERROR ❌ error
error from build pipeline

Caused by:
    0: error from asset pipeline
    1: error during cargo build execution
    2: cargo call to executable 'cargo' with args: '["build", "--target=wasm32-unknown-unknown", "--manifest-path", "/home/kali/projects/weekly67/devrust/frontend-1/webrust/Cargo.toml"]' returned a bad status: exit status: 101
```

```sh
┌──(kali㉿kali)-[~/…/weekly67/devrust/frontend-1/webrust]
└─$ cargo add compiler_builtins
    Updating crates.io index
      Adding compiler_builtins v0.1.105 to dependencies.
             Features:
             + compiler-builtins
             - c
             - cc
             - core
             - mangled-names
             - mem
             - no-asm
             - public-test-deps
             - rustc-dep-of-std
             - weak-intrinsics
    Updating crates.io index
```


```sh
┌──(kali㉿kali)-[~/…/weekly67/devrust/frontend-1/webrust]
└─$ rustup update    

```

```sh
┌──(kali㉿kali)-[~/…/weekly67/devrust/frontend-1/webrust]
└─$ cargo run                                                     
   Compiling proc-macro2 v1.0.66
   Compiling unicode-ident v1.0.11
   Compiling wasm-bindgen-shared v0.2.87
   Compiling log v0.4.19
   Compiling bumpalo v3.13.0
   Compiling wasm-bindgen v0.2.87
   Compiling cfg-if v1.0.0
   Compiling autocfg v1.1.0
   Compiling serde v1.0.171
   Compiling itoa v1.0.9
   Compiling futures-core v0.3.28
   Compiling serde_json v1.0.103
   Compiling ryu v1.0.15
   Compiling quote v1.0.31
   Compiling futures-channel v0.3.28
   Compiling thiserror v1.0.43
   Compiling syn v2.0.26
   Compiling pin-project-lite v0.2.10
   Compiling futures-sink v0.3.28
   Compiling version_check v0.9.4
   Compiling slab v0.4.8
   Compiling memchr v2.5.0
   Compiling futures-task v0.3.28
   Compiling futures-util v0.3.28
   Compiling syn v1.0.109
   Compiling proc-macro-error-attr v1.0.4
   Compiling tokio v1.29.1
   Compiling pin-utils v0.1.0
   Compiling libc v0.2.147
   Compiling futures-io v0.3.28
   Compiling percent-encoding v2.3.0
   Compiling rustversion v1.0.14
   Compiling form_urlencoded v1.2.0
   Compiling proc-macro-error v1.0.4
   Compiling indexmap v1.9.3
   Compiling prettyplease v0.1.25
   Compiling bytes v1.4.0
   Compiling fnv v1.0.7
   Compiling http v0.2.9
   Compiling wasm-bindgen-backend v0.2.87
   Compiling wasm-bindgen-macro-support v0.2.87
   Compiling serde_derive v1.0.171
   Compiling thiserror-impl v1.0.43
   Compiling wasm-bindgen-macro v0.2.87
   Compiling futures-macro v0.3.28
   Compiling js-sys v0.3.64
   Compiling pin-project-internal v1.1.2
   Compiling pin-project v1.1.2
   Compiling web-sys v0.3.64
   Compiling wasm-bindgen-futures v0.4.37
   Compiling anymap2 v0.13.0
   Compiling hashbrown v0.12.3
   Compiling futures v0.3.28
   Compiling pinned v0.1.0
   Compiling num_cpus v1.16.0
   Compiling gloo-timers v0.2.6
   Compiling tracing-attributes v0.1.26
   Compiling tokio-stream v0.1.14
   Compiling bincode v1.3.3
   Compiling serde-wasm-bindgen v0.5.0
   Compiling serde_urlencoded v0.7.1
   Compiling tracing-core v0.1.31
   Compiling boolinator v2.4.0
   Compiling compiler_builtins v0.1.105
   Compiling tracing v0.1.37
   Compiling yew-macro v0.20.0
   Compiling gloo-utils v0.1.7
   Compiling gloo-events v0.1.2
   Compiling gloo-console v0.2.3
   Compiling gloo-worker v0.2.1
   Compiling gloo-file v0.2.3
   Compiling gloo-history v0.1.4
   Compiling gloo-net v0.3.0
   Compiling gloo-storage v0.2.2
   Compiling gloo-render v0.1.1
   Compiling gloo-dialogs v0.1.1
   Compiling gloo v0.8.1
   Compiling prokio v0.1.0
   Compiling implicit-clone v0.3.5
   Compiling console_error_panic_hook v0.1.7
error[E0554]: `#![feature]` may not be used on the stable release channel
 --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:2:38
  |
2 | #![cfg_attr(not(feature = "no-asm"), feature(asm))]
  |                                      ^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
 --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:3:1
  |
3 | #![feature(abi_unadjusted)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
 --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:4:1
  |
4 | #![feature(asm_experimental_arch)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
 --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:5:38
  |
5 | #![cfg_attr(not(feature = "no-asm"), feature(global_asm))]
  |                                      ^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
 --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:6:1
  |
6 | #![feature(cfg_target_has_atomic)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
 --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:7:1
  |
7 | #![feature(compiler_builtins)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
 --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:8:1
  |
8 | #![feature(core_ffi_c)]
  | ^^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
 --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:9:1
  |
9 | #![feature(core_intrinsics)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
  --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:10:1
   |
10 | #![feature(inline_const)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
  --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:11:1
   |
11 | #![feature(lang_items)]
   | ^^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
  --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:12:1
   |
12 | #![feature(linkage)]
   | ^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
  --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:13:1
   |
13 | #![feature(naked_functions)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
  --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:14:1
   |
14 | #![feature(repr_simd)]
   | ^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
 --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:2:46
  |
2 | #![cfg_attr(not(feature = "no-asm"), feature(asm))]
  |                                              ^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
 --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:5:46
  |
5 | #![cfg_attr(not(feature = "no-asm"), feature(global_asm))]
  |                                              ^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
 --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:8:12
  |
8 | #![feature(core_ffi_c)]
  |            ^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
 --> /home/kali/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.105/src/lib.rs:9:12
  |
9 | #![feature(core_intrinsics)]
  |            ^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0554`.
error: could not compile `compiler_builtins` (lib) due to 17 previous errors
warning: build failed, waiting for other jobs to finish...
```


------------
------------
------------

https://rustwasm.github.io/docs/book/game-of-life/setup.html#wasm-pack

- install wasm
```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/wasmrust]
└─$ cargo install wasm-pack        
    Updating crates.io index
  Downloaded wasm-pack v0.12.1
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/wasmrust]
└─$ cargo install wasm-pack        
    Updating crates.io index
  Downloaded wasm-pack v0.12.1
```


```sh
┌──(kali㉿kali)-[~/projects/weekly67/devrust/wasmrust]
└─$ cargo generate --git https://github.com/rustwasm/wasm-pack-template         
🤷   Project Name: wasm-game-of-life
🔧   Destination: /home/kali/projects/weekly67/devrust/wasmrust/wasm-game-of-life ...
🔧   project-name: wasm-game-of-life ...
🔧   Generating template ...
🔧   Moving generated files into: `/home/kali/projects/weekly67/devrust/wasmrust/wasm-game-of-life`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /home/kali/projects/weekly67/devrust/wasmrust/wasm-game-of-life
```


```sh
┌──(kali㉿kali)-[~/…/weekly67/devrust/wasmrust/wasm-game-of-life]
└─$ wasm-pack build
[INFO]: 🎯  Checking for the Wasm target...
info: downloading component 'rust-std' for 'wasm32-unknown-unknown'
```

```sh
┌──(kali㉿kali)-[~/…/weekly67/devrust/wasmrust/wasm-game-of-life]
└─$ npm init wasm-app www      
Need to install the following packages:
create-wasm-app@0.1.0
Ok to proceed? (y) 
🦀 Rust + 🕸 Wasm = ❤
```


```sh
www> npm install
```

---

```sh
┌──(kali㉿kali)-[~/…/webstatic4/axum/examples/static-file-server]
└─$ cargo run 
    Finished dev [unoptimized + debuginfo] target(s) in 6.56s
     Running `/home/kali/projects/weekly67/devrust/webstatic4/axum/target/debug/example-static-file-server`
2024-01-31T12:18:00.804669Z DEBUG example_static_file_server: listening on 127.0.0.1:3001
2024-01-31T12:18:00.804838Z DEBUG example_static_file_server: listening on 127.0.0.1:3002
2024-01-31T12:18:00.804984Z DEBUG example_static_file_server: listening on 127.0.0.1:3003
2024-01-31T12:18:00.805124Z DEBUG example_static_file_server: listening on 127.0.0.1:3004
2024-01-31T12:18:00.805250Z DEBUG example_static_file_server: listening on 127.0.0.1:3005
2024-01-31T12:18:00.805380Z DEBUG example_static_file_server: listening on 127.0.0.1:3006
2024-01-31T12:18:00.805518Z DEBUG example_static_file_server: listening on 127.0.0.1:3307
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ curl http://localhost:3002/     
Hi from index.html
                          
```

```sh
┌──(kali㉿kali)-[~/…/webstatic4/axum/examples/testing]
└─$ cargo test
   Compiling example-testing v0.1.0 (/home/kali/projects/weekly67/devrust/webstatic4/axum/examples/testing)
    Finished test [unoptimized + debuginfo] target(s) in 25.68s
     Running unittests src/main.rs (/home/kali/projects/weekly67/devrust/webstatic4/axum/target/debug/deps/example_testing-b4e032331c1bc338)

running 6 tests
test tests::hello_world ... ok
test tests::json ... ok
test tests::not_found ... ok
test tests::multiple_request ... ok
test tests::with_into_make_service_with_connect_info ... ok
test tests::the_real_deal ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```


```sh

┌──(kali㉿kali)-[~/projects/weekly67/devrust/webstatic4]
└─$ cargo new yew-app   
     Created binary (application) `yew-app` package
                                                                                                                                           
┌──(kali㉿kali)-[~/projects/weekly67/devrust/webstatic4]
└─$ cd yew-app   

┌──(kali㉿kali)-[~/…/weekly67/devrust/webstatic4/yew-app]
└─$ cargo run  

```

```rs
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter +1;
            counter.set(value);
        }
    };
    html! {
        <div>
          <button {onclick} >{ "+1" }</button>
          <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

```

```html
<!doctype html>
<html>
    <head>
        <meta charset="utf-8" />
        <title>Yew App</title>
    </head>
    <body></body>
</html>
```


```sh
┌──(kali㉿kali)-[~/…/weekly67/devrust/webstatic4/yew-app]
└─$ trunk serve --open

```

```sh
touch Trunk.toml
```

```toml
[serve]
# The address to serve on LAN.
address = "127.0.0.1"
# The address to serve on WAN.
# address = "0.0.0.0"
# The port to serve on.
port = 8000
```

```sh
┌──(kali㉿kali)-[~/…/weekly67/devrust/webstatic4/yew-app]
└─$ trunk serve --open

```




---




---