// This file contains the Rust code that will be compiled to WebAssembly. It exports functions that can be called from JavaScript.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use web_sys::{window, Element, MouseEvent};
use std::rc::Rc;
use std::cell::RefCell;
use std::u128::MIN;

mod ball;

use ball::{Ball, Pad, Pong, Point};

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
        const DELTA: f32 = 1.0 / FRAME_RATE_IN_MS as f32;
        let ball_speed = pong.ball.speed.clone();
        pong.ball.rect.top_left += Point {
            x: ball_speed.x * DELTA,
            y: ball_speed.y * DELTA
        };

        // compute the right pad enemy AI
        const AI_SPEED: f32 = 500.0;

        let pad_center = pong.right_pad.rect.top_left.y + pong.right_pad.rect.size.y / 2.0;
        let ball_center = pong.ball.rect.top_left.y + pong.ball.rect.size.y / 2.0;
        
        if pad_center < ball_center {
            pong.right_pad.rect.top_left.y += f32::min(AI_SPEED * DELTA, ball_center - pad_center);
        } else {
            pong.right_pad.rect.top_left.y -= f32::min(AI_SPEED * DELTA, pad_center - ball_center);
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
        if pong.ball.speed.y < 0.0 {
            if pong.ball.rect.top_left.y < 0.0{
                pong.ball.rect.top_left.y = 0.0;
                pong.ball.speed.y *= -1.0;
            }
        }

        if pong.ball.speed.x < 0.0 {
            if pong.ball.rect.top_left.x < 0.0 {
                pong.ball.rect.top_left.x = 0.0;
                pong.ball.speed.x *= -1.0;
            }

            if pong.ball.rect.top_left.y >= pong.left_pad.rect.top_left.y && pong.ball.rect.top_left.y + pong.ball.rect.size.y <= pong.left_pad.rect.top_left.y + pong.left_pad.rect.size.y {
                if pong.ball.rect.top_left.x + pong.ball.rect.size.x >= pong.left_pad.rect.top_left.x && pong.ball.rect.top_left.x <= pong.left_pad.rect.top_left.x + pong.left_pad.rect.size.x {
                    pong.ball.speed.x *= -1.0;
                }
            }
        }

        if pong.ball.speed.y > 0.0 {
            if pong.ball.rect.top_left.y + pong.ball.rect.size.y > pong.field.size.y {
                pong.ball.rect.top_left.y = pong.field.size.y - pong.ball.rect.size.y;
                pong.ball.speed.y *= -1.0;
            }
        }
        
        if pong.ball.speed.x > 0.0 {
            if pong.ball.rect.top_left.x + pong.ball.rect.size.x > pong.field.size.x {
                pong.ball.rect.top_left.x = pong.field.size.x - pong.ball.rect.size.x;
                pong.ball.speed.x *= -1.0;
            }

            if pong.ball.rect.top_left.y >= pong.right_pad.rect.top_left.y && pong.ball.rect.top_left.y + pong.ball.rect.size.y <= pong.right_pad.rect.top_left.y + pong.right_pad.rect.size.y {
                if pong.ball.rect.top_left.x + pong.ball.rect.size.x >= pong.right_pad.rect.top_left.x && pong.ball.rect.top_left.x <= pong.right_pad.rect.top_left.x + pong.right_pad.rect.size.x {
                    pong.ball.speed.x *= -1.0;
                }
            }
        }

        set_position_and_size(&ball_div, 
            pong.ball.rect.top_left.x,
            pong.ball.rect.top_left.y,
            pong.ball.rect.size.x,
            pong.ball.rect.size.y,
        );

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