pub mod app;
pub mod canvas;
// pub mod proxy;
pub mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub fn exit(message: &str) {
    let v = JsValue::from_str(message);
    web_sys::console::log_1(&("panic".into()));
    web_sys::console::exception_1(&v);
    std::process::abort();
}

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init()
        .expect("console_log::init failed");
    console_error_panic_hook::set_once();

    #[cfg(debug_assertions)]
    web_sys::console::log_1(&JsValue::from_str(
        "debug",
    ));

    #[cfg(not(debug_assertions))]
    web_sys::console::log_1(&JsValue::from_str(
        "release",
    ));
}
