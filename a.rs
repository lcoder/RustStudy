
trait Shape {
  fn area(&self) -> f64;
}

trait Round {
  fn get_radius(&self) -> f64;
}

struct Circle {
  radius: f64,
}

impl Round for Circle {
  fn get_radius(&self) -> f64 {
    self.radius
  }
}

impl Shape for Round {
  fn area(&self) -> f64 {
    std::f64::consts::PI * self.get_radius() * self.get_radius()
  }
}

fn main() {
  // let c = Circle { radius: 2f64 };
  // c.area();
  let b = Box::new(Circle { radius: 4f64 }) as Box<Round>;
  b.area();
}

