use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement,
};

use crate::utils::{
    debounce, device_pixel_ratio, get_canvas_size,
    get_ctx, get_window, lazy_round,
};

const STROKE_SIZE: f64 = 4.0;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub dpr: f64,
    pub el: HtmlCanvasElement,
    pub ctx: CanvasRenderingContext2d,
    pub width: f64,
    pub height: f64,
    pub bgcolor: String,
    pub color: String,
    pub frame: i32,
}

impl Canvas {
    pub fn new(
        el: HtmlCanvasElement,
        bgcolor: String,
        color: String,
    ) -> Self {
        web_sys::console::log_1(
            &("[canvas] ++++ new()".into()),
        );
        let ctx = get_ctx(&el).unwrap();
        let dpr: f64 = device_pixel_ratio();

        ctx.scale(dpr, dpr).unwrap_or(());

        Canvas {
            dpr,
            el,
            ctx,
            width: 100.0,
            height: 100.0,
            bgcolor,
            color,
            frame: 0,
        }
    }

    pub fn register_listeners(&mut self) {
        web_sys::console::log_1(
            &("[canvas] ++++ regsiter_listeners()"
                .into()),
        );
        let self_rc =
            Rc::new(RefCell::new(self.clone()));

        let mut debounced_update_size = debounce(
            move || {
                let mut canvas = self_rc.borrow_mut();
                canvas.update_size();
            },
            Duration::from_millis(500),
        );

        let callback =
            Closure::wrap(Box::new(move || {
                debounced_update_size();
            })
                as Box<dyn FnMut()>);

        get_window()
            .expect("No window")
            .set_onresize(Some(
                callback.as_ref().unchecked_ref(),
            ));

        callback.forget(); // prevent closure being dropped soon
    }

    // Called when browser size changes.
    pub fn update_size(&mut self) {
        let (w, h): (f64, f64) =
            get_canvas_size(&self.el);

        let width: f64 = w * self.dpr;
        let height: f64 = h * self.dpr;

        web_sys::console::log_1(
            &("[canvas] Updating canvas size".into()),
        );

        web_sys::console::log_1(
            &(format!(
                "[canvas] {} x {}",
                lazy_round(width),
                lazy_round(height)
            )
            .into()),
        );

        self.el.set_width(width as u32);
        self.el.set_height(height as u32);

        self.width = lazy_round(width);
        self.height = lazy_round(height);
    }

    pub fn update(&mut self) {
        self.frame += 1;
    }

    pub fn draw(&mut self) {
        self.ctx.set_fill_style(
            &self.bgcolor.as_str().into(),
        );

        self.ctx.fill_rect(
            0_f64,
            0_f64,
            self.width,
            self.height,
        );

        self.ctx.set_stroke_style(
            &self.color.as_str().into(),
        );

        self.ctx.set_line_width(STROKE_SIZE);

        self.ctx.save();
        self.ctx.begin_path();
        self.ctx.move_to(0_f64, 0_f64);
        self.ctx.line_to(200_f64, 200_f64);
        self.ctx.stroke();
        self.ctx.restore();
    }
}
