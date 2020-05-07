#[derive(Copy, Clone)]
pub struct Cursor {
    pub x: u8,
    pub y: u8,
}

impl Cursor {
    pub fn new() -> Self {
        Self { x: 2, y: 2 }
    }
}
