
fn direction_to_int(x: i32) -> () {
  match x {
    -1 => println!("negative"),
    0 | 10 => println!("zero"),
    1 => println!("postive"),
    _ => println!("error"),
  }
}

fn main() {
  direction_to_int(10);
}
