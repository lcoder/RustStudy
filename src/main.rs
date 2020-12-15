

fn main() {
  let greeting: &str = "Hello";

  let substr: &str = &greeting[2..];

  println!("{}", substr);

  println!("Size of pointer: {}", std::mem::size_of::<*const ()>());
  println!("Size of &str : {}", std::mem::size_of::<&str>());
}

