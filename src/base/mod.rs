pub mod el;
pub mod sig;

use leptos_reactive::{self, create_signal, create_runtime, create_scope, create_effect, SignalUpdate, Scope, SignalGet, WriteSignal};
use web_sys::{self, Element, Window, window, console, Document, Event, Text, HtmlElement, Node};
use crate::base::el::El;

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
