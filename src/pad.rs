use crate::rect::Rect;

pub struct Pad {
    pub rect: Rect,
}

impl Pad {
    pub fn new(rect: Rect) -> Self {
        Pad { rect }
    }
}