
fn print_slice(arr: &[i32]) {
  println!("Length: {}", arr.len());

  for item in arr {
    print!("{}\t", item);
  }

  println!("");
}


fn main() {
  let arr: [i32; 5] = [1,2,3,4,5];
  print_slice(&arr[..]);   // full range

  let slice = &arr[2..];   // rangeFrom
  print_slice(slice);

  let slice2 = &arr[..2];  // RangeTo
  print_slice(slice2);
}

