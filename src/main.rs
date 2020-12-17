enum Direction {
  East,
  West,
  South,
  North,
}

fn direction_to_int(x: Direction) -> i32 {
  match x {
    Direction::East => 10,
    Direction::West => 20,
    Direction::South => 30,
    Direction::North => 40,
  }
}

fn main() {
  let x = Direction::West;
  let s = direction_to_int(x);
  println!("{}", s);
}
