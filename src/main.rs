use std::fmt::Debug;

trait Base {}

// trait Derived: Base {}

// 或者如下  给Derived这个trait加了个约束条件，即实现Derived trait的具体类型，也必须满足Base trait的约束
trait Derived where Self: Base {}

struct T;

impl Derived for T {}

impl Base for T {}

fn main() {
  
}

