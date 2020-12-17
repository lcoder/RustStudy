
fn main() {
  let x = 5_i32;

  match x {
    ref r => println!("Got a reference to {}", r),  // r的类型是&i32
  }
}
