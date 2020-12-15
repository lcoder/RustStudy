use std::ascii::AsciiExt;

fn main() {
  fn capitalize(substr: &mut str) {
    substr.make_ascii_uppercase();
  }

  let mut s = String::from("Hello World");

  // 编译器自动做了类型转换 &mut String -> &mut str
  capitalize(&mut s);

  println!("{}", s);
}

