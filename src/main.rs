
enum OptionalInt {
  Value(i32),
  Missing,
}

fn main() {
  let x = OptionalInt::Value(6);

  match x {
    OptionalInt::Value(i) if i > 5 => {
      println!("bigger than five!");
    }
    OptionalInt::Value(..) => {
      println!("got an int");
    }
    OptionalInt::Missing => {
      println!("No such luck")
    }
  }
}
