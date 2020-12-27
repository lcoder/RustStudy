
fn main() {
  type T1 = [i32;2];
  type T2 = (i32, i32);

  struct T3(i32, i32);

  struct T4 {
    field1: i32,
    field2: i32,
  }

  // 它们的基数都是Cardinality(i32) *  Cardinality(i32)

  struct R {
    var1: bool,
    var2: bool,
  }
  // 类型的基数，可以类比为代数中的求积运算
  Cardinality(R) = Cardinality(var1) * Cardinality(var2) = 2 * 2 = 4;

  struct Prod {
    field1: i32,
    field2: (),
  }
  // unit类型，并没有对结构体带来什么作用，没有额外信息
  Cardinality(Prod) = Cardinality(field1) * 1 = Cardinality(i32)

  // Rust enum类型，相当于代数中的求和
  enum Direction {
    North,
    East,
    South,
    West,
  }
  // 它的取值有4中可能性，基数就是4
  Cardinality(Direction) = Cardinality(North) + Cardinality(East) + Cardinality(South) + Cardinality(West) = 4;


  // 取代内置的bool类型
  enum Bool { True, False } // 表达能力和内置bool类型无差别

  
}
