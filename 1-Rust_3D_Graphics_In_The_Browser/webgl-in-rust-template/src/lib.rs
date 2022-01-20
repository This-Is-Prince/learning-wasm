use wasm_bindgen::prelude::*;
use web_sys::console;

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
pub struct PrinceClient {}

#[wasm_bindgen]
impl PrinceClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console::log_1(&JsValue::from_str("princeClient...."));

        Self {}
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        // console::log_1(&JsValue::from_str("update...."));
        Ok(())
    }
    pub fn render(&self) {
        // console::log_1(&JsValue::from_str("render...."));
    }
}
