use crate::point::{assert_point_xy, Point};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)] // Derive the Clone trait for Point
pub struct Rect {
    pub top_left: Point,
    pub size: Point,
}

impl Rect {
    pub fn new(top_left: Point, size: Point) -> Self {
        Rect {
            top_left,
            size,
        }
    }

    pub fn with_xywh(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rect::new(
            Point::new(x, y),
            Point::new(width, height)
        )
    }
    
    pub fn with_size(size: Point) -> Rect {
        Rect::new(
            Point::new(0.0, 0.0),
            size
        )
    }

    pub fn x(&self) -> f32 {
        self.top_left.x
    }

    pub fn y(&self) -> f32 {
        self.top_left.y
    }

    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }

    pub fn set_x(&mut self, x: f32) {
        self.top_left.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.top_left.y = y;
    }

    pub fn set_width(&mut self, width: f32) {
        self.size.x = width;
    }

    pub fn set_height(&mut self, height: f32) {
        self.size.y = height;
    }

    pub fn right(&self) -> f32 {
        self.x() + self.width()
    }

    pub fn bottom(&self) -> f32 {
        self.y() + self.height()
    }

    pub fn set_right(&mut self, right: f32) {
        self.set_x(right - self.width());
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        self.set_y(bottom - self.height());
    }

    pub fn bottom_right(&self) -> Point {
        Point::new(
            self.right(),
            self.bottom()
        )
    }

    pub fn set_bottom_right(&mut self, bottom_right: Point) {
        self.set_right(bottom_right.x);
        self.set_bottom(bottom_right.y);
    }

    pub fn center(&self) -> Point {
        Point::new(
            self.x() + self.width() / 2.0,
            self.y() + self.height() / 2.0
        )
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        self.x() < other.right() &&
        self.right() > other.x() &&
        self.y() < other.bottom() &&
        self.bottom() > other.y()
    }

    pub fn keep_inside(&mut self, container: Rect) {
        if self.y() < container.y() {
            self.set_y(container.y());
        }

        if self.x() < container.x() {
            self.set_x(container.x());
        }

        if self.right() > container.right() {
            self.set_right(container.right());
        }

        if self.bottom() > container.bottom() {
            self.set_bottom(container.bottom());
        }
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}]",
            self.x(),
            self.y(),
            self.width(),
            self.height()
        )
    }
}

pub fn assert_rect_xywh(rect: &Rect, x: f32, y: f32, w: f32, h: f32) {
    assert_point_xy(&rect.top_left, x, y);
    assert_point_xy(&rect.size, w, h);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rect = Rect::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
        assert_rect_xywh(&rect, 1.0, 2.0, 3.0, 4.0);
    }

    #[test]
    fn test_with_xywh() {
        let rect = Rect::with_xywh(1.0, 2.0, 3.0, 4.0);
        assert_rect_xywh(&rect, 1.0, 2.0, 3.0, 4.0);
    }

    #[test]
    fn test_with_size() {
        let rect = Rect::with_size(Point::new(3.0, 4.0));
        assert_rect_xywh(&rect, 0.0, 0.0, 3.0, 4.0);
    }

    #[test]
    fn test_setters() {
        let mut rect = Rect::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
        rect.set_x(5.0);
        rect.set_y(6.0);
        rect.set_width(7.0);
        rect.set_height(8.0);
        assert_rect_xywh(&rect, 5.0, 6.0, 7.0, 8.0);
    }

    #[test]
    fn test_right_and_bottom() {
        let rect = Rect::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
        assert_point_xy(&rect.bottom_right(), 1.0 + 3.0, 2.0 + 4.0);
    }

    #[test]
    fn test_set_right_and_bottom() {
        let mut rect = Rect::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
        let expected = Point::new(10.0, 20.0);
        rect.set_bottom_right(expected);
        assert!(rect.bottom_right().almost_equal(&expected));
    }

    #[test]
    fn test_center() {
        let rect = Rect::new(Point::new(1.0, 2.0), Point::new(4.0, 6.0));
        let center = rect.center();
        let expected = Point::new(1.0 + 4.0 / 2.0, 2.0 + 6.0 / 2.0);
        assert!(center.almost_equal(&expected));
    }

    #[test]
    fn test_intersects_with() {
        let tl = Rect::with_xywh(0.0, 0.0, 1.0, 1.0);
        let tr = Rect::with_xywh(2.0, 0.0, 1.0, 1.0);
        let bl = Rect::with_xywh(0.0, 2.0, 1.0, 1.0);
        let br = Rect::with_xywh(2.0, 2.0, 1.0, 1.0);

        let hc = Rect::with_xywh(0.0, 1.0, 3.0, 1.0);
        let vc = Rect::with_xywh(1.0, 0.0, 1.0, 3.0);

        let rects = [tl, tr, bl, br];
        for i in 1..rects.len() {
            for j in 0..i {
                assert!(
                    !rects[i].intersects_with(&rects[j]),
                    "Rectangles should not intersect: {} and {}",
                    rects[i], rects[j]
                );
            }

            assert!(
                !rects[i].intersects_with(&hc),
                "Rectangles should not intersect: {} and {}",
                rects[i], hc
            );

            assert!(
                !rects[i].intersects_with(&vc),
                "Rectangles should not intersect: {} and {}",
                rects[i], vc
            );
        }

        assert!(hc.intersects_with(&vc), "Should intersect");

        let ht = Rect::with_xywh(0.0, 0.0, 3.0, 1.0);
        let hb = Rect::with_xywh(0.0, 2.0, 3.0, 1.0);
        let vl = Rect::with_xywh(0.0, 0.0, 1.0, 3.0);
        let vr = Rect::with_xywh(2.0, 0.0, 1.0, 3.0);

        let intersections = [
            [ht, tl, tr, vl, vc, vr],
            [hc, vl, vc, vr, hc, hc], // padded with hc so it compiles
            [hb, bl, br, vl, vc, vr],

            [vl, tl, bl, ht, hc, hb],
            [vc, ht, hc, hb, vc, vc],
            [vr, tr, br, ht, hc, hb],
        ];

        for i in 0..intersections.len() {
            let intersector = intersections[i][0];
            for j in 1..intersections[i].len() {
                assert!(
                    intersector.intersects_with(&intersections[i][j]),
                    
                "Rectangles should intersect: {} and {}",
                intersector, intersections[i][j]
            );
            }
        }
    }

    #[test]
    fn test_keep_inside() {
        let container = Rect::with_xywh(0.0, 0.0, 10.0, 10.0);
        let mut rect = Rect::with_xywh(-5.0, -5.0, 4.0, 4.0);

        rect.keep_inside(container);
        assert!(rect.top_left.almost_equal(&Point::new(0.0, 0.0)));

        rect.set_x(8.0);
        rect.set_y(8.0);
        rect.keep_inside(container);
        assert!(
            rect.bottom_right().almost_equal(&Point::new(10.0, 10.0)),
            "Expected bottom right to align.. {}",
            rect.bottom_right()
        );
    }
}