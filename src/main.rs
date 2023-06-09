use std::ops::Deref;
use crate::utils::console::*;
use wasm_bindgen::{self, JsValue, JsCast, JsError};
use web_sys::{self, Element, Window, window, Document, Event, Text, HtmlElement, Node};
use leptos_reactive::{self, create_signal, create_runtime, create_scope, create_effect, SignalUpdate, Scope, SignalGet, WriteSignal, ReadSignal};

pub mod utils;

fn main() {
    log("test");

    mount (|cx: Scope| {
        let (count, set_count) = create_signal(cx, 11);
        let counter: El = 
             div("", "")
                 .child (
                     button("-1", minus(set_count))
                 )
                 .text(" Value: ")
                 .dyn_text(cx, move || count.get().to_string())
                 .text(" ")
                 .child (
                     // button("+1", move |_| set_count.update(|n| *n += 1))
                     button("+1", plus(set_count))    
                 );

        // Everything that is mutable should be wrapped in Sig
        let mut display: Sig<El> = 
            Sig::new(cx, 
                div("", "")
                    .text("DISPLAYED")
            );

        let show = move |_| Sig::update(&mut display.write, div("", "").text("DISPLAYED"));
        let hide = move |_| Sig::update(&mut display.write, div("", "").text("UNDISPLAYED"));

        let show_button: El = button("Show", show);
        let hide_button: El = button("Hide", hide);
        
        let mut ret: Sig<El> = Sig::new(cx, div("", ""));
        Sig::update(&mut ret.write,
            div("", "")
                .child(hide_button)
                .child(show_button)
                .dyn_child(cx, move || display.get())

            );
        ret.get()
        //counter
    });
}

fn plus(n: WriteSignal<i32>) -> impl FnMut(Event) + 'static {

    move |_| n.update(|num: &mut i32| *num += 1)
}

fn minus(n: WriteSignal<i32>) -> impl FnMut(Event) + 'static {

    move |_| n.update(|num: &mut i32| *num -= 1)
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

    pub fn wrap(element: Element) -> Self {
        let mut el = El::new(element.tag_name().as_str());
        el.0 = element;

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
        let mut el: El = div("", "").child(div("child", ""));
        self.0.append_child(&el).unwrap();

        create_effect(cx, 
            move |_| {
                let value: String = f().0.outer_html(); 
                log(value); // This value is being properly changed
                let node_data: &Node = &(Node::from(f().0));

                let old_node: &Node = &Node::from(el.get_child_index(1).0); // I assume the issue is here  
                let res = el.0.replace_child(old_node, node_data); // el is not being updated
                match res {
                    Err(jserr) => {},
                    Ok(node) => {
                        log(node.node_value().unwrap().as_str());
                    }, // Ok is always returned
                }
                log(format!("el: {}", el.0.outer_html().as_str()).as_str());
            }
        );
        
        self
    }
    
    // Panics
    pub fn get_child_index(&self, index: u32) -> El {
        assert!(self.0.child_element_count() >= index);
        
        log((*self).0.children().item(index).unwrap().outer_html().as_str());
        El::wrap((*self).0.children().item(index).unwrap())
    }

    pub fn get_child_name(&self, name: &str) -> El {
        El::wrap((*self).0.children().named_item(name).unwrap())
    }
}
#[derive(Clone)]
pub struct Sig<T> where T: Clone + 'static {
    pub read: ReadSignal<T>,
    pub write: WriteSignal<T>

}

impl<T> Sig<T> where T: Clone + 'static {

    pub fn new(cx: Scope, value: T) -> Sig<T> {
        let signals = create_signal(cx, value);

        Sig {read: signals.0, write: signals.1}
    }

    pub fn get(&self) -> T {
        self.read.get()
    }

    pub fn update(write: &mut WriteSignal<T>, value: T) {
        write.update(|val: &mut T| *val = value);
    }
}

fn to_jsvalue(str: &str) -> JsValue {
    JsValue::from_str(str)
}

// Omg if this shit wasn't private!
// #[track_caller]
// pub fn create_mut_effect<T>(cx: Scope, f: impl Fn(Option<T>) -> T + 'static)
// where
//     T: 'static,
// {
//     cfg_if! {
//         if #[cfg(not(feature = "ssr"))] {
//             let e = cx.runtime.create_effect(f);
//             //eprintln!("created effect {e:?}");
//             cx.with_scope_property(|prop| prop.push(ScopeProperty::Effect(e)))
//         } else {
//             // clear warnings
//             _ = cx;
//             _ = f;
//         }
//     }
// }
