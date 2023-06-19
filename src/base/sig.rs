use leptos_reactive::{self, create_signal, create_runtime, create_scope, create_effect, SignalUpdate, Scope, SignalGet, WriteSignal, ReadSignal};
use wasm_bindgen::{self, JsCast, JsValue};

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
