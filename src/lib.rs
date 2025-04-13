// This file contains the Rust code that will be compiled to WebAssembly. It exports functions that can be called from JavaScript.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use web_sys::{window, Element, MouseEvent};
use std::rc::Rc;
use std::cell::RefCell;

mod ball;

use ball::{Pong, Point};

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn start_pong() {
    add_ball_div();
}

#[wasm_bindgen]
pub fn add_ball_div() {
    // Get the window and document objects
    let window = window().expect("should have a window");
    let document = window.document().expect("should have a document");
    let body = document.body().expect("should have a body");

    let pong = Rc::new(RefCell::new(Pong::new(0.0, 0.0, 640.0, 480.0)));

    // Create field div
    let field_div = document.create_element("div").expect("should create a div").dyn_into::<HtmlElement>().expect("should cast to HtmlElement");
    field_div.set_class_name("field");
    body.append_child(&field_div).expect("should append the div");
    set_position_and_size(
        &field_div,
        pong.borrow().field.top_left.x,
        pong.borrow().field.top_left.y,
        pong.borrow().field.size.x,
        pong.borrow().field.size.y,
    );

    // Create ball div
    let ball_div = document.create_element("div").expect("should create a div");
    ball_div.set_class_name("ball");
    field_div.append_child(&ball_div).expect("should append the div");
    set_position_and_size(
        &ball_div,
        pong.borrow().ball.rect.top_left.x,
        pong.borrow().ball.rect.top_left.y,
        pong.borrow().ball.rect.size.x,
        pong.borrow().ball.rect.size.y,
    );

    // Create left pad div
    let left_pad_div = document.create_element("div").expect("should create a div");
    left_pad_div.set_class_name("pad");
    field_div.append_child(&left_pad_div).expect("should append the div");
    set_position_and_size(
        &left_pad_div,
        pong.borrow().left_pad.rect.top_left.x,
        pong.borrow().left_pad.rect.top_left.y,
        pong.borrow().left_pad.rect.size.x,
        pong.borrow().left_pad.rect.size.y,
    );

    // Create right pad div
    let right_pad_div = document.create_element("div").expect("should create a div");
    right_pad_div.set_class_name("pad");
    field_div.append_child(&right_pad_div).expect("should append the div");
    set_position_and_size(
        &right_pad_div,
        pong.borrow().right_pad.rect.top_left.x,
        pong.borrow().right_pad.rect.top_left.y,
        pong.borrow().right_pad.rect.size.x,
        pong.borrow().right_pad.rect.size.y,
    );


    let left_score_div = document.create_element("div").expect("should create a div");
    left_score_div.set_class_name("score left");
    field_div.append_child(&left_score_div).expect("should append the div");
    left_score_div.set_text_content(Some("0"));

    let right_score_div = document.create_element("div").expect("should create a div");
    right_score_div.set_class_name("score right");
    field_div.append_child(&right_score_div).expect("should append the div");
    right_score_div.set_text_content(Some("0"));

    let speed_counter_div = document.create_element("div").expect("should create a div");
    speed_counter_div.set_class_name("speed_counter");
    field_div.append_child(&speed_counter_div).expect("should append the div");
    speed_counter_div.set_text_content(Some("0"));

    // Add mousemove event listener to update left pad position
    let pong_clone = Rc::clone(&pong);
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mouse_y = event.client_y() as f32 - field_div.offset_top() as f32;
        let mut pong = pong_clone.borrow_mut();
        pong.left_pad.rect.top_left.y = mouse_y - pong.left_pad.rect.size.y / 2.0;
        
        // clamp the left pad position to the field
        if pong.left_pad.rect.top_left.y  < pong.field.top_left.y{
            pong.left_pad.rect.top_left.y = pong.field.top_left.y;
        }

        if pong.left_pad.rect.top_left.y  > pong.field.top_left.y + pong.field.size.y - pong.left_pad.rect.size.y {
            pong.left_pad.rect.top_left.y = pong.field.top_left.y + pong.field.size.y - pong.left_pad.rect.size.y;
        }

        let left_pad = &pong.left_pad;
        set_position_and_size(
            &left_pad_div,
            left_pad.rect.top_left.x,
            left_pad.rect.top_left.y,
            left_pad.rect.size.x,
            left_pad.rect.size.y,
        );
    }) as Box<dyn FnMut(_)>);

    window
        .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
        .expect("should add mousemove event listener");
    closure.forget(); // Prevent the closure from being dropped

    const FRAME_RATE_IN_MS: i32 = 50;
    let pong_clone = Rc::clone(&pong);
    let closure = Closure::wrap(Box::new(move || {
        let mut pong = pong_clone.borrow_mut();
        let mut scored = 0;

        const DELTA: f32 = 1.0 / FRAME_RATE_IN_MS as f32;
        const AI_SPEED: f32 = 400.0;

        let player_skill = pong.right_score as i32 - pong.left_score as i32;
        let mut rubberbanded_ai_speed = AI_SPEED;
        if player_skill > 0 {
            rubberbanded_ai_speed *= 1.0 + 0.25 * player_skill as f32;
        } else if player_skill < 0 {
            rubberbanded_ai_speed /= 1.0 + 0.25 * player_skill as f32 * -1.0;
        }

        let mut speed_boost = 1.0 + pong.speed_counter as f32 / 5.0;
        if player_skill > 0 {
            speed_boost += (player_skill as f32 / 4.0).min(1.0);
        }
        let ball_speed = pong.ball.speed.clone();
        let speed = f32::sqrt(ball_speed.x * ball_speed.x + ball_speed.y * ball_speed.y);
        let mut normalized = Point {
            x: ball_speed.x / speed,
            y: ball_speed.y / speed
        };

        let mut i = 0;
        while (i as f32) < speed {
            let mut iteration = normalized.clone();
            let mut iteration_ratio = 1.0;
            if (i as f32 + 1.0) > speed  {
                iteration_ratio = speed - i as f32;
                iteration = Point { 
                    x: iteration.x * iteration_ratio,
                    y: iteration.y * iteration_ratio
                };
            }

            pong.ball.rect.top_left += Point {
                x: iteration.x * DELTA * speed_boost,
                y: iteration.y * DELTA * speed_boost
            };
            
            // compute the right pad enemy AI
            let pad_center = pong.right_pad.rect.top_left.y + pong.right_pad.rect.size.y / 2.0;
            let ball_center = pong.ball.rect.top_left.y + pong.ball.rect.size.y / 2.0;
            
            if pad_center < ball_center {
                pong.right_pad.rect.top_left.y += f32::min(rubberbanded_ai_speed * DELTA * (iteration_ratio / speed), ball_center - pad_center);
            } else {
                pong.right_pad.rect.top_left.y -= f32::min(rubberbanded_ai_speed * DELTA * (iteration_ratio / speed), pad_center - ball_center);
            }
            
            // clamp the right pad position to the field
            if pong.right_pad.rect.top_left.y < pong.field.top_left.y {
                pong.right_pad.rect.top_left.y = pong.field.top_left.y;
            }
            
            if pong.right_pad.rect.top_left.y > pong.field.top_left.y + pong.field.size.y - pong.right_pad.rect.size.y {
                pong.right_pad.rect.top_left.y = pong.field.top_left.y + pong.field.size.y - pong.right_pad.rect.size.y;
            }

            set_position_and_size(
                &right_pad_div,
                pong.right_pad.rect.top_left.x,
                pong.right_pad.rect.top_left.y,
                pong.right_pad.rect.size.x,
                pong.right_pad.rect.size.y,
            );

            // perform collision check
            if normalized.y < 0.0 {
                if pong.ball.rect.top_left.y < 0.0{
                    pong.ball.rect.top_left.y = 0.0;
                    normalized.y *= -1.0;
                }
            }

            if normalized.x < 0.0 {
                if pong.ball.rect.top_left.x < 0.0 {
                    pong.ball.rect.top_left.x = 0.0;
                    normalized.x *= -1.0;
                    scored = -1;
                }

                if pong.ball.rect.top_left.y >= pong.left_pad.rect.top_left.y && pong.ball.rect.top_left.y + pong.ball.rect.size.y <= pong.left_pad.rect.top_left.y + pong.left_pad.rect.size.y {
                    if pong.ball.rect.top_left.x + pong.ball.rect.size.x >= pong.left_pad.rect.top_left.x && pong.ball.rect.top_left.x <= pong.left_pad.rect.top_left.x + pong.left_pad.rect.size.x {
                        normalized.x *= -1.0;
                        pong.speed_counter += 1;
                    }
                }
            }

            if normalized.y > 0.0 {
                if pong.ball.rect.top_left.y + pong.ball.rect.size.y > pong.field.size.y {
                    pong.ball.rect.top_left.y = pong.field.size.y - pong.ball.rect.size.y;
                    normalized.y *= -1.0;
                }
            }
            
            if normalized.x > 0.0 {
                if pong.ball.rect.top_left.x + pong.ball.rect.size.x > pong.field.size.x {
                    pong.ball.rect.top_left.x = pong.field.size.x - pong.ball.rect.size.x;
                    normalized.x *= -1.0;
                    scored = 1;
                }

                if pong.ball.rect.top_left.y >= pong.right_pad.rect.top_left.y && pong.ball.rect.top_left.y + pong.ball.rect.size.y <= pong.right_pad.rect.top_left.y + pong.right_pad.rect.size.y {
                    if pong.ball.rect.top_left.x + pong.ball.rect.size.x >= pong.right_pad.rect.top_left.x && pong.ball.rect.top_left.x <= pong.right_pad.rect.top_left.x + pong.right_pad.rect.size.x {
                        normalized.x *= -1.0;
                        pong.speed_counter += 1;
                    }
                }

                if scored != 0 {
                    break;
                }
            }
            i += 1;
        }

        pong.ball.speed = Point {
            x: f32::abs(pong.ball.speed.x) * f32::signum(normalized.x),
            y: f32::abs(pong.ball.speed.y) * f32::signum(normalized.y)
        };

        set_position_and_size(&ball_div, 
            pong.ball.rect.top_left.x,
            pong.ball.rect.top_left.y,
            pong.ball.rect.size.x,
            pong.ball.rect.size.y,
        );
        
        if scored != 0 {
            if scored < 0 {
                pong.left_score += 1;
            } else {
                pong.right_score += 1;
            }
            
            // Reset the ball position
            pong.ball.rect.top_left.x = pong.field.size.x / 2.0 - pong.ball.rect.size.x / 2.0;
            pong.ball.rect.top_left.y = pong.field.size.y / 2.0 - pong.ball.rect.size.y / 2.0;
            pong.ball.speed = Point { x: 300.0, y: -300.0 };
            pong.left_pad.rect.top_left.y = pong.field.size.y / 2.0 - pong.left_pad.rect.size.y / 2.0;
            pong.right_pad.rect.top_left.y = pong.field.size.y / 2.0 - pong.right_pad.rect.size.y / 2.0;
            pong.speed_counter = 0;
        }

        left_score_div.set_text_content(Some(&pong.left_score.to_string()));
        right_score_div.set_text_content(Some(&pong.right_score.to_string()));
        speed_counter_div.set_text_content(Some(&pong.speed_counter.to_string()));
    }) as Box<dyn Fn()>);
    window
    .set_interval_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        FRAME_RATE_IN_MS,
    )
    .expect("should register timeout");
closure.forget(); // Prevent the closure from being dropped
}

pub fn set_position_and_size(element: &Element, x: f32, y: f32, width: f32, height: f32) {
    element
        .set_attribute(
            "style",
            &format!(
                "left: {}px; top: {}px; width: {}px; height: {}px;",
                x, y, width, height
            ),
        )
        .expect("should set style");
}