#![feature(const_fn)]

const fn cube(num: usize) -> usize {
  num * num * num
}

fn main() {
  const DIM: usize = cube(3);

  println!("DIM={}", DIM);
}

