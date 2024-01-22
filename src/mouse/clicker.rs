use enigo::*;

enum VPos {
  Top,
  Bottom
}

pub struct OffsetClicker {
  mouse: Enigo,
  position: VPos,
  x: i32,
  y: i32,
  v_offset: i32,
}

impl OffsetClicker {
  pub fn new(x: i32, y: i32, v_offset: i32) -> Self {
    Self {mouse: Enigo::new(), position: VPos::Top, x, y, v_offset}
  }

  pub fn click(&mut self) {
    self.mouse.mouse_move_to(self.x, self.y);
    self.mouse.mouse_click(MouseButton::Left);
  }

  pub fn move_and_click(&mut self) {
    let (offset, pos) = self.next_pos();
    self.y = self.y + offset;
    self.position = pos;

    self.click();
  }

  fn next_pos(&self) -> (i32, VPos) {
    match self.position {
      VPos::Top => (self.v_offset, VPos::Bottom),
      VPos::Bottom => (-self.v_offset, VPos::Top)
    }
  }
}

unsafe impl Send for OffsetClicker {}
