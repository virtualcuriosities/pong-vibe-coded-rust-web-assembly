use crate::{ball::Ball, pad::Pad};


pub struct EnemyStrategy {
    pub movement: f32,
}

impl EnemyStrategy {
    pub fn new(pad: &Pad, ball: &Ball, max_movement: f32) -> Self {
        let pad_center = pad.rect.center().y;
        let ball_center = ball.rect.center().y;
        
        let dist = (ball_center - pad_center).abs();
        let min_dist = max_movement.min(dist);

        if pad_center < ball_center {
            EnemyStrategy {
                movement: min_dist,
            }
        } else {
            EnemyStrategy {
                movement: min_dist * -1.0,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{point::Point, rect::Rect};

    use super::*;

    #[test]
    pub fn test_enemy_strategy() {
        let max_movement = 5.0;
        
        let pad = Pad {
            rect: Rect::with_xywh(0.0, 100.0, 10.0, 50.0),
        };
        
        let mut ball = Ball {
            rect: Rect::with_xywh(0.0, 0.0, 10.0, 10.0),
            speed: Point::new(10.0, 10.0),
        };
        
        {
            let strategy = EnemyStrategy::new(&pad, &ball, max_movement);
            assert!(strategy.movement < 0.0);
        }

        ball.rect.set_y(200.0);
        {
            let strategy = EnemyStrategy::new(&pad, &ball, max_movement);
            assert!(strategy.movement > 0.0);
        }

        ball.rect.set_y(-ball.rect.height() / 2.0 + pad.rect.center().y);
        {
            let strategy = EnemyStrategy::new(&pad, &ball, max_movement);
            assert!(strategy.movement == 0.0);
        }
    }
}