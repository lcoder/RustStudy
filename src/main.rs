
enum Direction {
  East,
  West,
  South,
  North,
}

fn print(x: Direction) {
  match x {
    Direction::East => { println!("East"); }
    Direction::West => { println!("West"); }
    Direction::South => { println!("South"); }
    _ => {
      println!("Other");
    }
  }
}


fn main() {
  let x = Direction::North;
  print(x);
}

