
struct T(i32);

impl T {
  fn func(this: &Self) {
    println!("value {}", this.0);
  }
}

fn main() {
  let x = T(42);

  T::func(&x);
}

