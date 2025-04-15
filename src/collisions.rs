use crate::{ball::Ball, pad::Pad, point::Point, rect::Rect};

pub struct CollisionResults {
    pub ball_position: Point,
    pub ball_speed: Point,
    pub pad_hits: u32,
    pub scored: i32,
}

impl CollisionResults {
    pub fn new(ball: &Ball, ball_speed: &Point, field: &Rect, left_pad: &Pad, right_pad: &Pad) -> Self {
        let mut ball_rect = ball.rect;
        let mut ball_speed = *ball_speed;
        let mut pad_hits = 0;
        let mut scored = 0;

        if ball_speed.y < 0.0 {
            if ball_rect.y() < field.y() {
                ball_rect.set_y(0.0);
                ball_speed.y *= -1.0;
            }
        }

        if ball_speed.x < 0.0 {
            if ball_rect.x() < 0.0 {
                ball_rect.set_x(0.0);
                ball_speed.x *= -1.0;
                scored = -1;
            }

            if ball_rect.intersects_with(&left_pad.rect) {
                ball_speed.x *= -1.0;
                pad_hits += 1;
            }
        }

        if ball_speed.y > 0.0 {
            if ball.rect.bottom() > field.bottom() {
                ball_rect.set_bottom(field.bottom());
                ball_speed.y *= -1.0;
            }
        }
        
        if ball_speed.x > 0.0 {
            if ball.rect.right() > field.right() {
                ball_rect.set_right(field.right());
                ball_speed.x *= -1.0;
                scored = 1;
            }

            if ball.rect.intersects_with(&right_pad.rect) {
                ball_speed.x *= -1.0;
                pad_hits += 1;
            }
        }

        CollisionResults { ball_speed, ball_position: ball_rect.top_left, pad_hits, scored }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_collision_results() {
        let field = Rect::with_xywh(0.0, 0.0, 640.0, 480.0);
        let left_pad = Pad::new(Rect::with_xywh(10.0, 0.0, 10.0, 50.0));
        let right_pad = Pad::new(Rect::with_xywh(620.0, 0.0, 10.0, 50.0));
        let mut ball = Ball::new(Rect::with_xywh(300.0, 200.0, 10.0, 10.0), Point::new(5.0, 5.0));
        let mut ball_speed = Point::new(5.0, 5.0);

        // test no collision
        {
            let collision_results = CollisionResults::new(&ball, &ball_speed, &field, &left_pad, &right_pad);
            assert_eq!(collision_results.pad_hits, 0);
            assert_eq!(collision_results.scored, 0);
            assert_eq!(collision_results.ball_position, ball.rect.top_left);
            assert_eq!(collision_results.ball_speed, ball_speed);
        }

        // left colllision
        {
            ball.rect.top_left = left_pad.rect.top_left;
            ball_speed.x = -5.0;
            let collision_results = CollisionResults::new(&ball, &ball_speed, &field, &left_pad, &right_pad);
            assert_eq!(collision_results.pad_hits, 1);
            assert_eq!(collision_results.scored, 0);
            assert_eq!(collision_results.ball_position, ball.rect.top_left);
            assert_eq!(collision_results.ball_speed, Point::new(ball_speed.x * -1.0, ball_speed.y));
        }

        // right colllision
        {
            ball.rect.top_left = right_pad.rect.top_left;
            ball_speed.x = 5.0;
            let collision_results = CollisionResults::new(&ball, &ball_speed, &field, &left_pad, &right_pad);
            assert_eq!(collision_results.pad_hits, 1);
            assert_eq!(collision_results.scored, 0);
            assert_eq!(collision_results.ball_position, ball.rect.top_left);
            assert_eq!(collision_results.ball_speed, Point::new(ball_speed.x * -1.0, ball_speed.y));
        }

        // left score
        {
            ball.rect.top_left = Point::new(field.x() - 1.0, field.y());
            ball_speed.x = -5.0;
            let collision_results = CollisionResults::new(&ball, &ball_speed, &field, &left_pad, &right_pad);
            assert_eq!(collision_results.pad_hits, 0);
            assert_eq!(collision_results.scored, -1);
            assert_eq!(collision_results.ball_position, field.top_left);
            assert_eq!(collision_results.ball_speed, Point::new(ball_speed.x * -1.0, ball_speed.y));
        }

        // right score
        {
            ball.rect.set_right(field.right() + 1.0);
            ball_speed.x =  5.0;
            let collision_results = CollisionResults::new(&ball, &ball_speed, &field, &left_pad, &right_pad);
            assert_eq!(collision_results.pad_hits, 0);
            assert_eq!(collision_results.scored, 1);
            assert_eq!(collision_results.ball_position, Point::new(field.right() - ball.rect.width(), field.y()));
            assert_eq!(collision_results.ball_speed, Point::new(ball_speed.x * -1.0, ball_speed.y));
        }

        // top collision
        {
            ball.rect.top_left = Point::new(field.x(), field.y() - 1.0);
            ball_speed = Point::new(0.0, -1.0);
            let collision_results = CollisionResults::new(&ball, &ball_speed, &field, &left_pad, &right_pad);
            assert_eq!(collision_results.pad_hits, 0);
            assert_eq!(collision_results.scored, 0);
            assert_eq!(collision_results.ball_position, field.top_left);
            assert_eq!(collision_results.ball_speed, ball_speed * -1.0);
        }

        // bottom collision
        {
            ball.rect.top_left = Point::new(field.x(), field.bottom() + 1.0);
            ball_speed = Point::new(0.0, 1.0);
            let collision_results = CollisionResults::new(&ball, &ball_speed, &field, &left_pad, &right_pad);
            assert_eq!(collision_results.pad_hits, 0);
            assert_eq!(collision_results.scored, 0);
            assert_eq!(collision_results.ball_position, Point::new(field.x(), field.bottom() - ball.rect.height()));
            assert_eq!(collision_results.ball_speed, ball_speed * -1.0);
        }
    }
}