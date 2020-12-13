
trait Double {
  fn double(&self) -> Self;
}

impl Double for i32 {
  fn double(&self) -> i32 {
    *self * 2
  }
}


fn main() {
  let x:i32 = 10.double();
  println!("x = {}", x);
}

