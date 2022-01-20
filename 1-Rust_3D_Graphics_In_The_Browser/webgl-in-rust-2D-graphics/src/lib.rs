use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

mod common_funcs;
mod gl_setup;
mod programs;
mod shaders;

/*
Wee Allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
*/

#[wasm_bindgen]
pub fn start() {
    console::log_1(&JsValue::from_str("start...."));
}

#[wasm_bindgen]
pub struct PrinceClient {
    gl: WebGlRenderingContext,
    program_color_2d: programs::Color2D,
}

#[wasm_bindgen]
impl PrinceClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();
        Self {
            program_color_2d: programs::Color2D::new(&gl),
            gl,
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        // console::log_1(&JsValue::from_str("update...."));
        Ok(())
    }
    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        self.program_color_2d.render(
            &self.gl, //webgl context
            100.,     //bottom
            200.,     //top
            100.,     //left
            200.,     //right
            400.,     //canvas_height
            400.,     //canvas_width
        );
    }
}
