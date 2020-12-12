
trait Shape {
  fn area(self: Box<Self>) -> f64;
}

struct Circle {
  radius: f64,
}

impl Shape for Circle {
  fn area(self: Box<Self>) -> f64 {
    std::f64::consts::PI * self.radius * self.radius
  }
}


fn main() {
  // let c = Circle { radius: 2f64 };
  // c.area();
  let b = Box::new(Circle { radius: 4f64 });
  b.area();
}

