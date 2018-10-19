use piston::input::{Button, Key};

#[derive(Copy, Clone)]
pub struct Input {
    key_right: Key,
    key_left: Key,
    pub right: bool,
    pub left: bool,
}

impl Input {
    pub fn new(key_right: Key, key_left: Key) -> Input {
        Input {
            key_right,
            key_left,
            right: false,
            left: false,
        }
    }

    pub fn press(&mut self, button: Button) {
        match button {
            Button::Keyboard(key) if key == self.key_right => self.right = true,
            Button::Keyboard(key) if key == self.key_left => self.left = true,
            _ => {}
        }
    }

    pub fn release(&mut self, button: Button) {
        match button {
            Button::Keyboard(key) if key == self.key_right => self.right = false,
            Button::Keyboard(key) if key == self.key_left => self.left = false,
            _ => {}
        }
    }
}