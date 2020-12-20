
fn main() {
  struct T {
    item1: char,
    item2: bool,
  }

  fn test(T{item1: arg1, item2: arg2}: T) {
    println!("{} {}", arg1, arg2);
  }

  let x = T {
    item1: 'A',
    item2: false,
  };

  test(x);
}
