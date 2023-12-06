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
pub fn concur_step_04(){
    let v = vec![1, 2, 3];
    let handle = thread::spawn( move || {
        println!("Here's the vector: {:?}", v);
    });
    //drop(v); // value used here after move
    handle.join().unwrap();
}
use std::sync::mpsc; // multiple producer single consumer
pub fn concur_step_05(){
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    let val = String::from("Message! Hi");
    tx.send(val).unwrap();
  });
  let received = rx.recv().unwrap();
  println!("Got: {}", received);
}

pub fn concur_step_06(){
    let (tx, rx) = mpsc::channel();
    thread::spawn( move || {
        let val = String::from("Msg: hi");
        tx.send(val).unwrap();
        //println!("val is {}", val); // value borrowed here after move
    });
    let received = rx.recv().unwrap();
    println!("Got!: {}", received);
}

pub fn concur_step_07(){
  let (tx, rx) =  mpsc::channel();
  thread::spawn( move || {
    let vals = vec![
        String::from("Hey.."),
        String::from("from.."),
        String::from("the.."),
        String::from("thread.."),
    ];
    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
  });
  for received in rx {
    println!("Got!!: {}", received);
  }
}

pub fn concur_step_08(){
    let (tx, rx) =  mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn( move || {
        let vals = vec![
            String::from("inside..."),
            String::from("the..."),
            String::from("thread..."),
        ];
        for val in  vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn( move || {
        let vals = vec![
            String::from("more.."),
            String::from("and more.."),
            String::from("more messages.."),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got!:: {}", received);
    }
}

use std::sync::Mutex;
pub fn concur_step_09(){
    let m = Mutex::new(5);
    {
      let mut num = m.lock().unwrap();
      *num = 6;
    }
    println!("m = {:?}", m);
}

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
