
enum Movement {
  Left,
  Right,
  Down,
  Up,
}

pub fn run () {
  let left = Movement::Left;
  let right = Movement::Right;
  let down = Movement::Down;
  let up = Movement::Up;
  move_avatar(left);
  move_avatar(right);
  move_avatar(down);
  move_avatar(up);
}
fn move_avatar(m:Movement) {
  match m {
      Movement::Left => println!("turn to left"),
      Movement::Right => println!("turn to right"),
      Movement::Down => println!("turn to down"),
      Movement::Up => println!("turn to up"),
  }
}