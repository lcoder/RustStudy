
fn main() {
  let v: [[i32; 2]; 3] = [[0,0],[0,1],[0,0]];

  for i in &v {
    println!("{:?}", i);
  }
}

