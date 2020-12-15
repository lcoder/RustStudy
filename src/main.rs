

fn main() {
  use std::iter::Iterator;

  let v = &[10i32, 20, 30, 40, 50];

  // 同时需要index和内部元素的值，调用enumerate()
  for (index, value) in v.iter().enumerate() {
    println!("{} {}", index, value);
  }

  let item = v.iter().filter( |&x| *x % 2 == 0 ).nth(2);

  println!("{:?}", item);
}

