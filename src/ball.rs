use crate::{point::Point, rect::Rect};

pub struct Ball {
    pub rect: Rect,
    pub speed: Point,
}

impl Ball {
    pub fn new(rect: Rect, speed: Point) -> Self {
        Ball { rect, speed }
    }
}