use std::iter::Iterator;
use std::ops::Range;

fn main() {
  let r = 1..10;
  for i in r {
    print!("{:?}\t", i);
  } 
  let r = Range { start: 1, end: 10 };
  for i in r {
    print!("{:?}\t", i)
  }

  let r = (1i32..11).rev().map(|i| i * 10);

  for i in r {
    print!("{:?}\t", i);
  }
}

