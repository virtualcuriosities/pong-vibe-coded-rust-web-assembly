use web_sys::HtmlElement;

use crate::{point::Point, rect::Rect};

pub struct Styling<'a> {
    target: &'a HtmlElement
}

impl<'a> Styling<'a> {
    pub fn new(target: &'a HtmlElement) -> Self {
        Self { target }
    }

    pub fn set_px(&self, property: &str, value: f32) -> Result<(), wasm_bindgen::JsValue> {
        let style = self.target.style();
        style.set_property(property, &to_px(value))?;
        Ok(())
    }

    pub fn set_top_left(&self, top_left: &Point) -> Result<(), wasm_bindgen::JsValue> {
        self.set_px("left", top_left.x)?;
        self.set_px("top", top_left.y)?;
        Ok(())
    }

    pub fn set_size(&self, size: &Point) -> Result<(), wasm_bindgen::JsValue> {
        self.set_px("width", size.x)?;
        self.set_px("height", size.y)?;
        Ok(())
    }

    pub fn set_rect(&self, rect: &Rect) -> Result<(), wasm_bindgen::JsValue> {
        self.set_top_left(&rect.top_left)?;
        self.set_size(&rect.size)?;
        Ok(())
    }
}

pub fn to_px(value: f32) -> String {
    format!("{}px", value)
}