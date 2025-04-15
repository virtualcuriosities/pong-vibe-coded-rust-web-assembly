// This file contains the Rust code that will be compiled to WebAssembly. It exports functions that can be called from JavaScript.

use pong::Pong;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use std::rc::Rc;
use std::cell::RefCell;

mod ai;
mod ball;
mod collisions;
mod pad;
mod page;
mod point;
mod pong;
mod rect;
mod styling;

use page::Page;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn start_pong() {
    let page = Page::from_global().expect("Failed to get window, document, or body element.");
    
    let pong = Pong::new(&page, 0.0, 0.0, 640.0, 480.0).expect("Failed to create Pong");
    pong.refresh();

    let pong_rc = Rc::new(RefCell::new(pong));
    {
        // Add mousemove event listener
        let pong_rc = pong_rc.clone();
        page.on_event("mousemove", Box::new(move |event: MouseEvent| {
            let mut pong = pong_rc.borrow_mut();
            pong.handle_mouse_move(&event);
        }));
    };

    {
        const FRAME_RATE_IN_MS: i32 = 50;
        const DELTA: f32 = 1.0 / FRAME_RATE_IN_MS as f32;

        let pong_rc = pong_rc.clone();
        page.on_interval(FRAME_RATE_IN_MS, Box::new(move || {
            let mut pong = pong_rc.borrow_mut();
            pong.logic.handle_game_tick(DELTA);

            true
        }));
    }

    page.on_request_animation_frame({
        let pong_clone = pong_rc.clone();

        Box::new(move || {
            let pong = pong_clone.borrow();
            pong.refresh();
            
            true
        })
    });
}