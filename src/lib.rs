use std::usize;

pub struct Blinky {
  light: [[bool; 1000]; 1000],
}

impl Default for Blinky {
  fn default() -> Self {
    Self {
      light: [[false; 1000]; 1000],
    }
  }
}

impl Blinky {
  pub fn new() -> Self {
    Self {
      light: [[false; 1000]; 1000],
    }
  }

  pub fn turn_light_on(&mut self, row: usize, col: usize) {
    self.light[row][col] = true;
  }

  pub fn turn_light_off(&mut self, row: usize, col: usize) {
    self.light[row][col] = false;
  }

  pub fn is_light_on(&self, row: usize, col: usize) -> bool {
    self.light[row][col]
  }

  pub fn toggle_light(&mut self, row: usize, col: usize) {
    self.light[row][col] = !self.light[row][col];
  }

  pub fn get_number_of_lights_on(&self) -> usize {
    let mut sum = 0;
    for row in 0..1000 {
      for col in 0..1000 {
        if self.light[row][col] {
          sum += 1;
        }
      }
    }
    sum
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no_lights_on() {
    let blinky = Blinky::new();
    assert_eq!(0, blinky.get_number_of_lights_on());
  }

  #[test]
  fn able_to_check_if_any_light_is_on_or_off() {
    let blinky = Blinky::new();
    assert_eq!(false, blinky.is_light_on(284, 42));
  }

  #[test]
  fn able_to_turn_one_light_on() {
    let mut blinky = Blinky::new();
    blinky.turn_light_on(0, 0);
    assert_eq!(true, blinky.is_light_on(0, 0));
    assert_eq!(false, blinky.is_light_on(284, 0));
  }

  #[test]
  fn able_to_toggle_one_light_on() {
    let mut blinky = Blinky::new();
    let row = 284;
    let col = 42;
    blinky.toggle_light(row, col);
    assert_eq!(true, blinky.is_light_on(row, col));
    assert_eq!(false, blinky.is_light_on(0, 0));
  }

  #[test]
  fn able_to_toggle_one_light_off() {
    let mut blinky = Blinky::new();
    let row = 284;
    let col = 42;
    blinky.turn_light_on(row, col);
    blinky.toggle_light(row, col);
    assert_eq!(false, blinky.is_light_on(row, col));
  }

  #[test]
  fn able_to_turn_one_light_off() {
    let mut blinky = Blinky::new();
    let row = 284;
    let col = 42;
    blinky.turn_light_on(row, col);
    blinky.turn_light_off(row, col);
    assert_eq!(false, blinky.is_light_on(row, col));
  }

  #[test]
  fn able_to_count_one_light_on() {
    let mut blinky = Blinky::new();
    let row = 284;
    let col = 42;
    blinky.turn_light_on(row, col);
    assert_eq!(1, blinky.get_number_of_lights_on());
  }
}
