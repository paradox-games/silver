//!  Silver3d web render engine:
//! mouse module with Mouse struct
//! and implementations

#[derive(Default)]
pub struct Mouse {
    pressed: bool,
    x: u16,
    y: u16,
}

impl Mouse {
    pub fn is_pressed(&self) -> bool {
        return self.pressed;
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.pressed = pressed;
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x as u16;
        self.y = y as u16;
    }

    pub fn position(&self) -> (u16, u16) {
        return (self.x, self.y);
    }
}