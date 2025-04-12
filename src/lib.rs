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
pub fn greet(name: &str) {
    let greeting = format!("Hello, {}!", name);
    alert(&greeting);
    add_ball_div();
}

#[wasm_bindgen]
pub fn add_ball_div() {
    // Get the window and document objects
    let window = window().expect("should have a window");
    let document = window.document().expect("should have a document");

    // Create a new div element
    let div = document.create_element("div").expect("should create a div");
    div.set_class_name("ball");

    // Append the div to the body
    let body = document.body().expect("should have a body");
    body.append_child(&div).expect("should append the div");

    // Instantiate a Ball object
    let ball = Ball::new(0.0, 0.0, 1.0, 1.0);
    // Set the initial position of the div
    div.set_attribute("style", &format!("left: {}px; top: {}px;", ball.x, ball.y))
        .expect("should set style");
}