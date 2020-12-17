
fn main() {
  let x = 'X';

  match x {
    'a' ..= 'z' => println!("lowercase"),
    'A' ..= 'Z' => println!("uppercase"),
    _ => println!("sth else"),
  }


}
