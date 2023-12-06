// concur/src/main.rs
use concur::{
  add, 
  concur_step_01, concur_step_02, concur_step_03,
  concur_step_04, concur_step_05, concur_step_06,
  concur_step_07, concur_step_08, concur_step_09,
  concur_step_10,
};
fn main(){
  println!(" {}", add(2,3));
  println!("-- ");
  concur_step_01();
  println!("-- ");
  concur_step_02();
  println!("-- ");
  concur_step_03();
  println!("-- ");
  concur_step_04();
  println!("-- ");
  concur_step_05();
  println!("-- ");
  concur_step_06();
  println!("-- ");
  concur_step_07();
  println!("-- ");
  concur_step_08();
  println!("-- ");
  concur_step_09();
  println!("-- ");
  concur_step_10();
  println!("-- ");
}