
fn main() {
  let x = (1, 2, 3);
  // let (a, _, _) = x;

  let (a, ..) = x;

  // let (a, .., b) = x;

  println!("{}", a);
}

