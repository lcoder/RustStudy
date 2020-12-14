
fn main() {
  let mut xs: [i32; 5] = [1, 2, 3, 4, 5];
  let ys: [i32; 5] = [6, 7, 8, 9, 10];

  xs = ys;

  println!("xs = {:?}; ys = {:?}", xs, ys);
}

