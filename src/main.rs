
#![feature(core_intrinsics)]
fn print_type_name<T>(_arg: &T) {
  unsafe {
    println!("{}", std::intrinsics::type_name::<T>());
  }
}

fn main() {
  let ref x = 5_i32;

  print_type_name(&x);  // &i32
}
