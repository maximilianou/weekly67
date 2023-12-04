use std::thread;

fn closure_1(){
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From Closure: {:?}", list);
    println!("Before calling closure : {:?}", list);
    only_borrows();
    println!("After calling closure  : {:?}", list);
    println!("---------------------------");
    let mut list_mut = vec![4, 5, 6];
    println!("Before defining closure: {:?}", list_mut);
    let mut borrows_mutably = || list_mut.push(7);
    borrows_mutably();
    println!("After calling closure  : {:?}", list_mut);
    println!("---------------------------");
    let list_thread  = vec![7, 8, 9];
    println!("Before defining closure: {:?}", list_thread);
  //  thread::spawn( move || println!("From thread: {:?}", list_thread)).join().unwrap();
  //  thread::spawn(  || println!("From thread: {:?}", list_thread)).join().unwrap();
    thread::spawn( move || println!("From thread: {:?}", list_thread)).join().unwrap();
  //  println!("After calling closure: {:?}", list_thread); // value borrowed here after move
  println!("---------------------------");
  
}

//impl<T> Option<T> {
//    pub fn unwrap_or_else<F>(self, f: F) -> T
//    where
//      F: FnOnce() -> T
//      {
//          match self {
//              Some(x) => x,
//              None => f(),
//          }
//      }
//}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn closure_2(){
  let mut list = [
    Rectangle { width: 10, height: 1 },
    Rectangle { width: 3, height: 5 },
    Rectangle { width: 7, height: 12 },
  ];

  list.sort_by_key(|r| r.width);
  println!("{:#?}",list);

}

fn main() {

//  closure_1();
  closure_2();

}

