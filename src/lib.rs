// This file contains the Rust code that will be compiled to WebAssembly. It exports functions that can be called from JavaScript.

use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element};

mod ball;

use ball::{Ball, Pad, Pong};

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

    let pong = Pong::new(0.0, 0.0, 640.0, 480.0);
    let ball = pong.ball;
    let left_pad = pong.left_pad;
    let right_pad = pong.right_pad;
    let field = pong.field;

    // Create field div
    let field_div = document.create_element("div").expect("should create a div");
    field_div.set_class_name("field");
    body.append_child(&field_div).expect("should append the div");
    set_position_and_size(
        &field_div,
        field.top_left.x,
        field.top_left.y,
        field.size.x,
        field.size.y,
    );

    // Create ball div
    let ball_div = document.create_element("div").expect("should create a div");
    ball_div.set_class_name("ball");
    field_div.append_child(&ball_div).expect("should append the div");
    set_position_and_size(
        &ball_div,
        ball.rect.top_left.x,
        ball.rect.top_left.y,
        ball.rect.size.x,
        ball.rect.size.y,
    );

    // Create left pad div
    let left_pad_div = document.create_element("div").expect("should create a div");
    left_pad_div.set_class_name("pad");
    field_div.append_child(&left_pad_div).expect("should append the div");
    set_position_and_size(
        &left_pad_div,
        left_pad.rect.top_left.x,
        left_pad.rect.top_left.y,
        left_pad.rect.size.x,
        left_pad.rect.size.y,
    );

    // Create right pad div
    let right_pad_div = document.create_element("div").expect("should create a div");
    right_pad_div.set_class_name("pad");
    field_div.append_child(&right_pad_div).expect("should append the div");
    set_position_and_size(
        &right_pad_div,
        right_pad.rect.top_left.x,
        right_pad.rect.top_left.y,
        right_pad.rect.size.x,
        right_pad.rect.size.y,
    );
}

pub fn set_position_and_size(element: &Element, x: f32, y: f32, width: f32, height: f32) {
    element
        .set_attribute(
            "style",
            &format!(
                "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
                x, y, width, height
            ),
        )
        .expect("should set style");
}