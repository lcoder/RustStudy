#[derive(Debug)]

struct Point3d {
  x: i32,
  y: i32,
  z: i32,
}

fn default() -> Point3d {
  Point3d { x: 0, y: 0, z: 0 }
}

fn main() {
  let origin = Point3d { x: 5, ..default() };
  let point = Point3d { z: 1, x: 2, ..origin };
  println!("origin origin = {:?} point = {:?}", origin, point);
}

