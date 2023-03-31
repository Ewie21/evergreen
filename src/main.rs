use std::{ops::Deref, fmt::{self, Pointer}};

use wasm_bindgen::{self, JsValue, JsCast};
use web_sys::{self, Element, Window, window, console, Document, Event, Node, Text, HtmlElement};
use leptos_reactive::{self, create_signal, create_runtime, create_scope, create_effect, SignalUpdate, Scope, SignalGet};

fn main() {
    mount ( |cx: Scope| {
        // TODO: Make it easier to create and edit variables.
        let (count, set_count) = create_signal(cx, 0);
        div("", "")
            .child (
                button("-1", move |_| set_count.update(|n| *n -= 1))
            )
            .text(" Value: ")
            .dyn_text(cx, move || count.get().to_string())
            .text(" ")
            .child (
                button("+1", move |_| set_count.update(|n| *n += 1))    
            )
    }   
    );
}

fn div( id: &str, class: &str) -> El {
    El::new("div")
        .attr("id", id)
        .attr("class", class)

}

fn button(label: &str, cb: impl FnMut(Event) + 'static) -> El{
    El::new("button")
        .on("click", cb)
        .text(label)
}

fn mount(f: impl FnOnce(Scope) -> El + 'static) {
    let runtime = create_runtime();
    _ = create_scope(runtime, |cx| {
        let window: Window = window().unwrap();
        let document: Document = window.document().unwrap();
        let body: HtmlElement = document.body().unwrap();

        let root: El = f(cx); 

        body.append_child(&root).unwrap();

    });

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

    pub fn dyn_text(self, cx: Scope, f: impl Fn() -> String + 'static) -> Self {
        let window: Window = window().unwrap();
        let document: Document = window.document().unwrap();
        let node: Text = document.create_text_node("");

        self.0.append_child(&node).unwrap();

        create_effect(cx, move |_| {
            let value: String = f();
            node.set_data(&value);
        });

        self
    }
}