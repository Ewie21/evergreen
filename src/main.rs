use std::ops::Deref;

use wasm_bindgen::{self, JsValue, JsCast};
use web_sys::{self, Element, Window, window, console, Document, Event, Node, Text, HtmlElement};

fn main() {
    mount (
        El::new("div")
            .child (
                El::new("button")
                    .on("click", |_| console::log_1(&JsValue::from_str("+1")))
                    .attr("id", "minus1")
                    .text("-1")
            )
            .text("Value")
            .child (
                El::new("button")
                    .on("click", |_| console::log_1(&JsValue::from_str("-1")))
                    .attr("id", "plus1")
                    .text("+1")
        )
    );
}

fn mount(root: El) {
    // let runtime = create_runtime();
    // _ = create_scope(runtime);
    let window: Window = window().unwrap();
    let document: Document = window.document().unwrap();
    let body: HtmlElement = document.body().unwrap();

    // let root = f(cx)
    body.append_child(&root).unwrap();
}

#[derive(Debug, Clone)]
pub struct El(Element);

impl Deref for El {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl El {
    pub fn new(tag_name: &str) -> Self {
        let window: Window = window().unwrap();
        let document = window.document().unwrap();
        let el: Element = document.create_element(tag_name).unwrap();

        Self(el)
    }

    pub fn on(self, event_name: &str, cb: impl FnMut(Event) + 'static) -> Self {
        use wasm_bindgen::prelude::Closure;
        let cb: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(cb) as Box<dyn FnMut(Event)>);
        self.0.add_event_listener_with_callback(
            event_name, 
            cb.as_ref().unchecked_ref()
        ).unwrap();
        cb.forget();
        
        self
    }

    pub fn attr(self, atter_name: &str, value: &str) -> Self {
        self.0.set_attribute(atter_name, value).unwrap();

        self
    }

    pub fn text(self, data: &str) -> Self {
        let window: Window = window().unwrap();
        let document: Document = window.document().unwrap();
        let node: Text = document.create_text_node(data);
        self.0.append_child(&node).unwrap();

        self
    }

    pub fn child(self, child: El) -> Self {
        self.0.append_child(&child).unwrap();

        self
    }
}