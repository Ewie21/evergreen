use std::ops::Deref;
use crate::{
    base::el::El,
    base::sig::Sig,
    utils::console::*
};
use wasm_bindgen::{self, JsCast};
use web_sys::{self, Element, Window, window, console, Document, Event, Text, HtmlElement, Node};
use leptos_reactive::{self, create_signal, create_runtime, create_scope, create_effect, SignalUpdate, Scope, SignalGet, WriteSignal};

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

