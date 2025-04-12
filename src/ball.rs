use std::ops::AddAssign;

#[derive(Clone)] // Derive the Clone trait for Point
pub struct Point {
    pub x: f32,
    pub y: f32,
}

// Implement AddAssign for Point to support the `+=` operator
impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

pub struct Rect {
    pub top_left: Point,
    pub size: Point,
}

pub struct Ball {
    pub rect: Rect,
    pub speed: Point,
}

pub struct Pad {
    pub rect: Rect,
}

pub struct Pong {
    pub field: Rect,
    pub ball: Ball,
    pub left_pad: Pad,
    pub right_pad: Pad,
}

impl Pong {
    pub fn new(left: f32, top: f32, width: f32, height: f32) -> Self {
        let pad_margin: f32 = 10.0;
        let center = Point { x: width / 2.0, y: height / 2.0 };
        let right = left + width;
        let pad_size = Point { x: 10.0, y: 50.0 };
        let ball_size = Point { x: 10.0, y: 10.0 };

        Pong {
            field: Rect {
                top_left: Point { x: left, y: top },
                size: Point { x: width, y: height },
            },
            ball: Ball {
                rect: Rect {
                    top_left: Point { x: center.x - ball_size.x / 2.0, y: center.y - ball_size.y / 2.0},
                    size: ball_size,
                },
                speed: Point { x: 10.0, y: 10.0 },
            },
            left_pad: Pad {
                rect: Rect {
                    top_left: Point { x: left + pad_margin, y: center.y  - pad_size.y / 2.0}, // Use center.y directly
                    size: pad_size.clone(),
                },
            },
            right_pad: Pad {
                rect: Rect {
                    top_left: Point { x: right - pad_margin - pad_size.x, y: center.y - pad_size.y / 2.0}, // Use center.y directly
                    size: pad_size.clone(),
                },
            },
        }
    }
}

impl Ball {
    pub fn update_position(&mut self) {
        self.rect.top_left += self.speed.clone(); // Clone speed to avoid moving it
    }
}