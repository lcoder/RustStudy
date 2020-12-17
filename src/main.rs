
enum OptionalInt {
  Value(i32),
  Missing,
}

fn main() {
  let x = 10;
  match x {
    i if i > 5 => println!("bigger than five"),
    i if i <= 5 => println!("less to five"),
    // 虽然已经覆盖了所有情况，可惜还是会出现编译器错误，只能加入这么一条分支，单纯避免编译错误
    _ => unreachable!(),
  }

}
