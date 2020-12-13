#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]

struct Foo {
  data: i32,
}

fn main() {
  let v1 = Foo { data: 0 };
  let v2 = v1;
  println!("{:?}", v2);
}

