

fn main() {
  let v = [10i32, 20, 30, 40, 50];

  let index: usize = std::env::args().nth(1).map(
    |x| x.parse().unwrap_or(0)
  ).unwrap_or(0);

  println!("{:?}", v[index]);
}

