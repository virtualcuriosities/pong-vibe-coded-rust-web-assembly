use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{self, HtmlElement};

pub struct Page {
    pub window: web_sys::Window,
    pub document: web_sys::Document,
    pub body: web_sys::HtmlElement,
}

impl Page {
    pub fn new(window: web_sys::Window) -> Option<Self> {
        let document = window.document()?;
        let body = document.body()?;
        Some(Page { window, document, body })
    }

    pub fn from_global() -> Option<Self> {
        // Get the window and document objects
        let window = web_sys::window()?;
        Page::new(window)
    }

    pub fn element(&self, tag: &str, cls: &str) -> Result<HtmlElement, wasm_bindgen::JsValue> {
        // Create a new element with the specified tag and class
        let element = self.document.create_element(tag)?;
        element.set_class_name(cls);

        // Append the new element to the body
        self.body.append_child(&element)?;

        // cast
        let html = element.dyn_into::<HtmlElement>()?;

        // Return the created element
        Ok(html)
    }

    pub fn element_with_parent(&self, tag: &str, cls: &str, parent: &HtmlElement) -> Result<HtmlElement, wasm_bindgen::JsValue> {
        let element = self.element(tag, cls)?;
        parent.append_child(&element)?;
        Ok(element)
    }

    pub fn on_event<T: wasm_bindgen::convert::FromWasmAbi + 'static>(&self, event: &str, callback: Box<dyn Fn(T) -> ()>) {
        let window = self.window.clone();

        let js_closure = Closure::wrap(Box::new(move |event: T| {
            callback(event);
        }) as Box<dyn Fn(T) >);
        
        window.add_event_listener_with_callback(
            event,
            js_closure.as_ref().unchecked_ref()
        ).expect("should add event listener");
        
        
        js_closure.forget();
    }
    
    pub fn on_interval(&self, interval: i32, callback: Box<dyn Fn() -> bool>) {
        let window = self.window.clone();
        
        let reg = Rc::new(RefCell::new(None as Option<RegisteredInterval>));

        let js_closure = {
            let window = self.window.clone();
            let reg = reg.clone();

            Closure::wrap(Box::new(move || {
                let should_continue = callback();
                if !should_continue {
                    window.clear_interval_with_handle(reg.borrow().as_ref().unwrap().interval_handle);
                }
            }) as Box<dyn Fn()>)
        };
        
        let handle = window.set_interval_with_callback_and_timeout_and_arguments_0(
            js_closure.as_ref().unchecked_ref(),
            interval,
        ).unwrap();

        *reg.borrow_mut() = Some(RegisteredInterval {
            interval_handle: handle,
            closure: js_closure
        });
    }

    pub fn on_request_animation_frame(&self, callback: Box<dyn Fn() -> bool>) {
        let request_animation_frame = Rc::new(RefCell::new(None as Option<Box<dyn Fn()> >));
        let inner_ref = request_animation_frame.clone();

        let js_closure = Closure::wrap(Box::new(move || {
            let should_continue = callback();
            if should_continue {
                inner_ref.borrow().as_ref().unwrap()();
            }
        }) as Box<dyn Fn()>);

        *request_animation_frame.borrow_mut() = {
            let window = self.window.clone();

            Some(Box::new(move || {
                window.request_animation_frame(
                    js_closure.as_ref().unchecked_ref(),
                ).unwrap();
            }))
        };

        request_animation_frame.borrow().as_ref().unwrap()();
    }
}

struct RegisteredInterval {
    interval_handle: i32,
    closure: Closure<dyn Fn() -> ()>
}