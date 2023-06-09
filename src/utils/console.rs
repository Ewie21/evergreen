use std::string::ToString;
use wasm_bindgen::{self, JsValue, JsCast};
use web_sys::{self, Element, Window, window, console, Document, Event, Text, HtmlElement, Node};
use leptos_reactive::{self, create_signal, create_runtime, create_scope, create_effect, SignalUpdate, Scope, SignalGet, WriteSignal, ReadSignal};

pub fn log<T: ToString>(value: T) {
    let js_value: &JsValue = &JsValue::from_str(value.to_string().as_str());
    console::log_1(js_value);
}
