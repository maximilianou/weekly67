use std::ops::Deref;
////////////////////////





////////////////////////
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
//////////////////////////

///////////////////////////
pub fn one_box(){
  let b = Box::new(5);
  println!("b = {}", b);
}

#[cfg(test)]
mod tests_one_box {

  

  #[test]
  fn one_box_all () {
    assert!(true)
  }

  #[test]
  fn dereference(){
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
  }
  #[test]
  fn dereference_box(){
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
  
  }

}

pub enum List {
  Cons(i32, Box<List>),
  Nil,
}
use crate::List::{Cons,Nil};

pub fn list_box() -> List {
  let listing = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  listing
}

//use std::rc::Rc;
//pub fn shared_list(){
//  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//  let b = Cons(33,  Rc::clone(&a));
//  let c = Cons(44,  Rc::clone(&a));
//}





//pub fn count_references(){
//  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//  println!("Count after creating a: {}", Rc::strong_count(&a));
//  let b = Cons(3, Rc::clone(&a));
//  println!("Count after creating b: {}", Rc::strong_count(&a));
//  {
//    let c = Cons(4, Rc::clone(&a));
//    println!("Count after creating c: {}", Rc::strong_count(&a));
//  }
// 
// println!("Count after c is out of scope: {}", Rc::strong_count(&a));
//}