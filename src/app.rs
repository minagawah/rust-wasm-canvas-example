use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::canvas::Canvas;
use crate::utils::{
    get_canvas, request_animation_frame_future, timer,
};

const REFRESH_RATE: i32 = 60;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub bgcolor: String,
    pub color: String,
}

#[wasm_bindgen]
pub struct App {
    #[wasm_bindgen(skip)]
    pub canvas: Canvas,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(
        params: &JsValue,
    ) -> Result<App, JsValue> {
        web_sys::console::log_1(
            &("[app] ++++ new()".into()),
        );
        let config: Config =
            serde_wasm_bindgen::from_value(
                params.clone(),
            )
            .unwrap();

        let bgcolor: String = config.bgcolor.clone();
        let color: String = config.color;
        let element =
            get_canvas("#rust-wasm-canvas-example")
                .unwrap();
        let mut canvas =
            Canvas::new(element, bgcolor, color);

        canvas.register_listeners();
        canvas.update_size();

        Ok(App { canvas })
    }

    #[wasm_bindgen]
    pub async fn start(&mut self) {
        web_sys::console::log_1(
            &("[app] ++++ start()".into()),
        );
        loop {
            timer(REFRESH_RATE).await.unwrap();
            self.canvas.update();
            self.canvas.draw();
            request_animation_frame_future().await;
        }
    }
}
