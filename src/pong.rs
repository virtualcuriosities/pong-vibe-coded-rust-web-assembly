use web_sys::{HtmlElement, MouseEvent};

use crate::{ai::EnemyStrategy, ball::Ball, collisions::CollisionResults, pad::Pad, page::Page, point::Point, rect::Rect, styling::Styling};


pub struct Pong {
    pub field_div: HtmlElement,
    pub ball_div: HtmlElement,
    pub left_pad_div: HtmlElement,
    pub right_pad_div: HtmlElement,
    pub left_score_div: HtmlElement,
    pub right_score_div: HtmlElement,
    pub speed_counter_div: HtmlElement,
    pub logic: PongLogic,
}
pub struct PongLogic {
    pub field: Rect,
    pub ball: Ball,
    pub left_pad: Pad,
    pub right_pad: Pad,
    pub left_score: u32,
    pub right_score: u32,
    pub speed_counter: u32,
}

pub struct PongTick<'a> {
    logic: &'a mut PongLogic,
    delta: f32,
    rubberbanded_ai_speed: f32,
    speed_boost: f32,
    normalized_ball_speed: Point,
    scored: i32,
}

impl Pong {
    pub fn new(page: &Page, left: f32, top: f32, width: f32, height: f32) -> Result<Self, wasm_bindgen::JsValue> {
        let pad_size = Point { x: 10.0, y: 50.0 };
        let ball_size = Point { x: 10.0, y: 10.0 };

        // Create divs
        let field_div = page.element("div", "field")?;
        page.body.append_child(&field_div)?;

        let ball_div = page.element_with_parent("div", "ball", &field_div)?;
        let left_pad_div = page.element_with_parent("div", "pad", &field_div)?;
        let right_pad_div = page.element_with_parent("div", "pad", &field_div)?;
        let left_score_div = page.element_with_parent("div", "score left", &field_div)?;
        let right_score_div = page.element_with_parent("div", "score right", &field_div)?;
        let speed_counter_div = page.element_with_parent("div", "speed_counter", &field_div)?;

        let mut pong = Pong {
            logic: PongLogic::new(
                Rect::with_xywh(left, top, width, height),
                Ball::new(Rect::with_size(ball_size), Point::ZERO),
                Pad {
                    rect: Rect::with_size(pad_size),
                },
                Pad {
                    rect: Rect::with_size(pad_size),
                },
            ),
            field_div,
            ball_div,
            left_pad_div,
            right_pad_div,
            left_score_div,
            right_score_div,
            speed_counter_div,
        };

        pong.logic.setup_round_start();

        Ok(pong)
    }

    pub fn refresh(&self) {
        self.refresh_ball();
        self.refresh_left_pad();
        self.refresh_right_pad();
        self.refresh_left_score();
        self.refresh_right_score();
        self.refresh_speed_counter();
        self.refresh_field();
    }

    pub fn refresh_field(&self) {
        let _ = Styling::new(&self.field_div)
            .set_rect(&self.logic.field);
    }
    
    pub fn refresh_ball(&self)  {
        let _ = Styling::new(&self.ball_div)
            .set_rect(&self.logic.ball.rect);
    }

    pub fn refresh_left_pad(&self) {
        let _ = Styling::new(&self.left_pad_div)
            .set_rect(&self.logic.left_pad.rect);
    }

    pub fn refresh_right_pad(&self) {
        let _ = Styling::new(&self.right_pad_div)
            .set_rect(&self.logic.right_pad.rect);
    }

    pub fn refresh_left_score(&self) {
        self.left_score_div.set_inner_text(&self.logic.left_score.to_string());
    }

    pub fn refresh_right_score(&self) {
        self.right_score_div.set_inner_text(&self.logic.right_score.to_string());
    }

    pub fn refresh_speed_counter(&self) {
        self.speed_counter_div.set_inner_text(&self.logic.speed_counter.to_string());
    }

    pub fn screen_point_to_field(&self, point: &Point) -> Point {
        Point::new(
            point.x - self.field_div.offset_left() as f32,
            point.y - self.field_div.offset_top() as f32,
        )
    }

    pub fn mouse_point_to_field(&self, event: &MouseEvent) -> Point {
        let mouse_x = event.client_x();
        let mouse_y = event.client_y();
        self.screen_point_to_field(&Point::new(mouse_x as f32, mouse_y as f32))
    }

    pub fn handle_mouse_move(&mut self, event: &MouseEvent) {
        let cursor = self.mouse_point_to_field(&event);
        self.logic.handle_mouse_move(cursor);
    }
}


impl PongLogic {
    pub fn new(field: Rect, ball: Ball, left_pad: Pad, right_pad: Pad) -> Self {
        PongLogic {
            field,
            ball,
            left_pad,
            right_pad,
            left_score: 0,
            right_score: 0,
            speed_counter: 0,
        }
    }

    pub fn handle_game_tick(&mut self, delta: f32) {
        let mut tick = PongTick::new(self, delta);
        tick.process();
    }

    pub fn setup_round_start(&mut self) {
        let pad_margin: f32 = 10.0;

        self.ball.rect.top_left = self.field.size / 2.0 - self.ball.rect.size / 2.0;
        self.ball.speed = Point { x: 300.0, y: -300.0 };
        
        self.left_pad.rect.set_x(self.field.x() + pad_margin);
        self.left_pad.rect.top_left.y = self.field.size.y / 2.0 - self.left_pad.rect.size.y / 2.0;
        
        self.right_pad.rect.set_right(self.field.right() - pad_margin);
        self.right_pad.rect.top_left.y = self.field.size.y / 2.0 - self.right_pad.rect.size.y / 2.0;

        self.speed_counter = 0;
    }
    
    fn handle_mouse_move(&mut self, cursor: Point) {
        self.left_pad.rect.top_left.y = cursor.y - self.left_pad.rect.height() / 2.0;
        self.left_pad.rect.keep_inside(self.field);
    }

    pub fn player_skill_diff(&self, player_score: u32, enemy_score: u32) -> i32 {
        player_score as i32 - enemy_score as i32
    }

    pub fn rubberbanded_ai_speed(&self, player_skill: i32) -> f32 {
        const AI_SPEED: f32 = 400.0;

        if player_skill > 0 {
            AI_SPEED * (1.0 + 0.25 * player_skill as f32)
        } else if player_skill < 0 {
            AI_SPEED / (1.0 + 0.25 * player_skill as f32 * -1.0)
        } else {
            AI_SPEED
        }
    }

    pub fn speed_boost(&self, player_skill: i32) -> f32 {
        let result = 1.0 + self.speed_counter as f32 / 5.0;

        if player_skill > 0 {
            result + (player_skill as f32 / 4.0).min(1.0)
        } else {
            result
        }
    }
}


impl<'a> PongTick<'a> {
    pub fn new(logic: &'a mut PongLogic, delta: f32) -> Self {
        let player_skill = logic.player_skill_diff(logic.left_score, logic.right_score);
        let rubberbanded_ai_speed = logic.rubberbanded_ai_speed(player_skill);
        let speed_boost = logic.speed_boost(player_skill);
        let normalized = logic.ball.speed.normalized();

        PongTick { logic, delta, rubberbanded_ai_speed, speed_boost, normalized_ball_speed: normalized, scored: 0 }
    }

    pub fn compute_enemy_ai(&mut self, alpha: f32) {
        let max_movement = self.rubberbanded_ai_speed * self.delta * alpha;
        let strategy = EnemyStrategy::new(&self.logic.right_pad, &self.logic.ball, max_movement);

        // apply strategy
        self.logic.right_pad.rect.top_left.y += strategy.movement;
        
        // clamp the right pad position to the field
        self.logic.right_pad.rect.keep_inside(self.logic.field);
    }

    pub fn compute_physics(&mut self) {
        let results = CollisionResults::new(
            &self.logic.ball,
            &self.normalized_ball_speed,
            &self.logic.field,
            &self.logic.left_pad,
            &self.logic.right_pad
        );

        self.logic.ball.rect.top_left = results.ball_position;
        self.normalized_ball_speed = results.ball_speed;

        if results.pad_hits > 0 {
            self.logic.speed_counter += results.pad_hits;
        }

        if results.scored != 0 {
            self.scored += results.scored;
        }
    }
    
    pub fn process(&mut self) {
        let speed = self.logic.ball.speed.length();

        let mut i = 0;
        while (i as f32) < speed {
            let mut iteration = self.normalized_ball_speed;
            let mut iteration_ratio = 1.0;
            if (i as f32 + 1.0) > speed  {
                iteration_ratio = speed - i as f32;
                iteration *= iteration_ratio;
            }

            self.logic.ball.rect.top_left += iteration * self.delta * self.speed_boost;
            
            // compute the right pad enemy AI
            self.compute_enemy_ai(iteration_ratio / speed);

            // perform collision check
            self.compute_physics();

            if self.scored != 0 {
                break;
            }
            i += 1;
        }

        self.logic.ball.speed = self.logic.ball.speed.abs() * self.normalized_ball_speed.signum();
        
        if self.scored != 0 {
            if self.scored > 0 {
                self.logic.left_score += 1;
            } else {
                self.logic.right_score += 1;
            }

            self.logic.setup_round_start();
        }
    }
}