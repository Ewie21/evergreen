use std::string::ToString;
use wasm_bindgen::{self, JsValue};
use web_sys::{self, console};

pub fn log<T: ToString>(value: T) {
    let js_value: &JsValue = &JsValue::from_str(value.to_string().as_str());
    console::log_1(js_value);
}