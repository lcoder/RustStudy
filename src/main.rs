#[lang = "sized"]
#[rustc_on_unimplemented = "`{Self}` does not have a constant size known at compile-time"]
#[fundamental] // for Default,for example,which required taht `[T]: !Default` be evaluatable
pub trait Sized {
  // Empty.
}


fn main() {
  
}

