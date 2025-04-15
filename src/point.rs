use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const ZERO: Point = Point { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Point::new(0.0, 0.0)
        } else {
            Point::new(self.x / len, self.y / len)
        }
    }

    pub fn abs(&self) -> Self {
        Point::new(self.x.abs(), self.y.abs())
    }

    pub fn signum(&self) -> Self {
        Point::new(self.x.signum(), self.y.signum())
    }

    pub fn almost_equal(&self, other: &Self) -> bool {
        self.almost_equal_with_margin(other, 1e-6)
    }

    pub fn almost_equal_with_margin(&self, other: &Self, margin: f32) -> bool {
        (self.x - other.x).abs() < margin && (self.y - other.y).abs() < margin
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}, {}]",
            self.x,
            self.y
        )
    }
}

pub fn assert_point_xy(point: &Point, x: f32, y: f32) {
    assert_eq!(point.x, x);
    assert_eq!(point.y, y);
}

macro_rules! impl_point_ops {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for Point {
            type Output = Point;
            fn $method(self, rhs: Point) -> Self::Output {
                Point::new(self.x $op rhs.x, self.y $op rhs.y)
            }
        }

        impl $trait<f32> for Point {
            type Output = Point;
            fn $method(self, rhs: f32) -> Self::Output {
                Point::new(self.x $op rhs, self.y $op rhs)
            }
        }
    }
}

macro_rules! impl_point_ops_assign {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for Point {
            fn $method(&mut self, rhs: Point) {
                self.x $op rhs.x;
                self.y $op rhs.y;
            }
        }

        impl $trait<f32> for Point {
            fn $method(&mut self, rhs: f32) {
                self.x $op rhs;
                self.y $op rhs;
            }
        }
    };
}

impl_point_ops!(Add, add, +);
impl_point_ops!(Sub, sub, -);
impl_point_ops!(Mul, mul, *);
impl_point_ops!(Div, div, /);
impl_point_ops_assign!(AddAssign, add_assign, +=);
impl_point_ops_assign!(SubAssign, sub_assign, -=);
impl_point_ops_assign!(MulAssign, mul_assign, *=);
impl_point_ops_assign!(DivAssign, div_assign, /=);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let point = Point::new(3.0, 4.0);
        assert_point_xy(&point, 3.0, 4.0);
    }

    #[test]
    fn test_equal() {
        assert_eq!(Point::new(3.0, 4.0), Point::new(3.0, 4.0));
    }
    #[test]
    fn test_almost_equal() {
        let margin = 1e-6;
        let p = Point::new(3.0, 4.0); 
        let p2 = p + Point::new(margin / 2.0, -margin / 2.0);
        assert!(p.almost_equal(&p2));
    }

    #[test]
    fn test_length() {
        let point = Point::new(3.0, 4.0);
        let expects = f32::sqrt(3.0 * 3.0 + 4.0 * 4.0);
        assert_eq!(point.length(), expects);
    }

    #[test]
    fn test_normalized() {
        let point = Point::new(3.0, 4.0);
        let length = point.length();
        let expected = Point::new(point.x / length, point.y / length);
        let normalized = point.normalized();
        assert!(normalized.almost_equal(&expected));
        
        let zero_point = Point::new(0.0, 0.0);

        assert_point_xy(&zero_point.normalized(), 0.0, 0.0);
    }

    #[test]
    fn test_abs() {
        let point = Point::new(-3.0, -4.0);
        assert_point_xy(&point.abs(), 3.0, 4.0);
    }

    #[test]
    fn test_signum() {
        let point = Point::new(-3.0, 4.0);
        assert_point_xy(&point.signum(), -1.0, 1.0);

        let other = Point::new(3.0, -4.0);
        assert_point_xy(&other.signum(), 1.0, -1.0);
    }

    #[test]
    fn test_zero_constant() {
        assert_point_xy(&Point::ZERO, 0.0, 0.0);
    }

    macro_rules! impl_point_ops_test {
        ($trait:ident, $test:ident, $op:tt, $assign_op:tt) => {
            #[test]
            fn $test() {
                let s = Point::new(1.0, 2.0);
                // point OP point
                {
                    let m = Point::new(3.0, 4.0);
    
                    let point = s $op m;
                    assert_point_xy(&point, s.x $op m.x, s.y $op m.y);
                }
    
                // point OP scalar
                {
                    let m = 2.0;
                    let point = s $op m;
                    assert_point_xy(&point, s.x $op m, s.y $op m);
                }

                // point ASSIGN_OP point
                {
                    let m = Point::new(-0.3, -0.4);
    
                    let mut point = s;
                    point $assign_op m;

                    assert_point_xy(&point, s.x $op m.x, s.y $op m.y);
                }

                // point ASSIGN_OP scalar
                {
                    let m = -0.2;

                    let mut point = s;
                    point $assign_op m;
                    
                    assert_point_xy(&point, s.x $op m, s.y $op m);
                }
            }
        };
    }

    impl_point_ops_test!(Add, test_add, +, +=);
    impl_point_ops_test!(Sub, test_sub, -, -=);
    impl_point_ops_test!(Mul, test_mul, *, *=);
    impl_point_ops_test!(Div, test_div, /, /=);

}
