
fn main() {
  let nan = std::f32::NAN;
  let x = 1.0f32;

  println!("{}", nan < x);
  println!("{}", nan > x);
  println!("{}", nan == x);
}

