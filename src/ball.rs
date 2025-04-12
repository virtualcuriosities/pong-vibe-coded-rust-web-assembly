pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

pub struct Pad {
    pub x: f32,
    pub y: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32, dx: f32, dy: f32) -> Self {
        Ball { x, y, dx, dy }
    }

    pub fn update_position(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    pub fn reset_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

pub struct Pong {
    pub left: f32,
    pub top: f32,
    pub bottom: f32,
    pub right: f32,
}

impl Pong {
    pub fn new(left: f32, top: f32, bottom: f32, right: f32) -> Self {
        Pong { left, top, bottom, right }
    }
}