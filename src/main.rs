

fn main() {
  let v = [10i32, 20, 30, 40, 50];

  let first = v.get(0);
  let tenth = v.get(10);

  println!("{:#?}, {:#?}", first, tenth);
}

