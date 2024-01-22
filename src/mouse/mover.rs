use enigo::*;

// Circular movement constants
// Center coordinates
const X_C: i32 = 500;
const Y_C: i32 = 500;

// Radius and step angle
const RADIUS: i32 = 100;
const DELTA_TH: f64 = 0.09; // in radians


pub struct RoundMover {
  mouse: Enigo,
  step: i32,
}

impl RoundMover {
  pub fn new() -> Self {
    Self {mouse: Enigo::new(), step: 0}
  }

  pub fn round_move(&mut self) {
    let (x, y) = self.next().unwrap();
    self.mouse.mouse_move_to(x, y);
  }

  fn angle(&self) -> f64 {
    (self.step as f64) * DELTA_TH
  }

  fn pos(&self) -> (i32, i32) {
    (
     X_C + ((RADIUS as f64 * self.angle().cos()) as i32),
     Y_C + ((RADIUS as f64 * self.angle().sin()) as i32),
    )
  }

  fn step_incr(&mut self) {
    let step = match self.step + 1 {
      a if a < 70 => a,
      _ => 0,
    };
    self.step = step;
  }
}

impl Iterator for RoundMover {

  type Item = (i32, i32);

  fn next(&mut self) -> Option<Self::Item> {
    self.step_incr();
    Some(self.pos())
  }
}

unsafe impl Send for RoundMover {}
