use std::ops::Deref;
use crate::{
    base::sig::Sig,
    utils::console::*
};
use wasm_bindgen::{self, JsCast};
use web_sys::{self, Element, Window, window, console, Document, Event, Text, HtmlElement, Node};
use leptos_reactive::{self, create_signal, create_runtime, create_scope, create_effect, SignalUpdate, Scope, SignalGet, WriteSignal};

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

    pub fn wrap(element: &Element) -> Self {
        let mut el = El::new((*element).tag_name().as_str());
        el.0 = element.clone();

        el
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

        create_effect(cx, 
            move |_| {
                let value: String = f();
                node.set_data(&value);
            }
        );

        self
    }

    pub fn dyn_child(self, cx: Scope, f: impl Fn() -> El + 'static) -> Self {
        let el = El::div("", "").child(El::div("child", ""));
        self.0.append_child(&el).unwrap();

        create_effect(cx, 
            move |_| {
                let value: String = f().0.outer_html();
                log(value); // This value is being properly changed
                let node_data = &Node::from(f().0);
                let old_node = &Node::from(el.get_child_index(0).0);
                // The issue is that it's looking at the address not the value 
                //log(El::from(*old_node).outer_html());
                let res = el.0.replace_child(old_node, node_data); // el is not being updated 
                match res {
                    Err(jserr) => { // Err varient is always returning
                        console::log_1(&jserr);
                    },
                    Ok(node) => {
                        log(node.node_value().unwrap().as_str());
                    }, 
                } // Err is returned
                log(format!("el: {}", el.0.outer_html().as_str()).as_str());
            }
        );
        
        self
    }
    
    /// Panics
    pub fn get_child_index(&self, index: u32) -> El {
        assert!(self.0.child_element_count() >= index);
        let children = self.0.children();
        let child: Element = children.get_with_index(index).unwrap();
        log(child.outer_html());
        El::wrap(&child)
    }
    
    /// Panics
    pub fn get_child_name(&self, name: &str) -> El {
        El::wrap(&self.0.children().named_item(name).unwrap())
    }

    fn div(id: &str, class: &str) -> El {
        El::new("div")
            .attr("id", id)
            .attr("class", class)

}

    fn button(label: &str, cb: impl FnMut(Event) + 'static) -> El{
        El::new("button")
            .on("click", cb)
            .text(label)
}
}

impl From<Node> for El {
    fn from(value: Node) -> El {
        let parent = value.parent_element().unwrap();
        let wrapped = El::wrap(&parent);
        wrapped.get_child_name(value.node_name().as_str()) // If this doesn't work, try this:
        // for i in 0..parent.child_element_count() {
        //     let value_as_el = wrapped.get_child_index(i);
        //     if wrapped.get_child_index(i).0.is_equal_node(Some(&value)) {
        //         return value_as_el;
        //     }   
        // }
        // log("Failure");
        // div("","")
    }
}
