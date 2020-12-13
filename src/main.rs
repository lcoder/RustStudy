use std::fmt::Debug;

fn my_print<T>(x: T) where T: Debug {
  println!("The value is {:?}", x);
}

fn main() {
  my_print("China");
  my_print(41_i32);
  my_print(true);
  my_print(['a', 'b', 'c']);
}

