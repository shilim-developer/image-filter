mod filters;

use filters::ImageFilter;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use tsify::Tsify;
use wasm_bindgen::prelude::*;
use web_sys::ImageData;

#[wasm_bindgen(getter_with_clone)]
pub struct AdjustParams {
    pub image_data: ImageData,
    pub brightness: Option<i32>,
    pub constrast_radio: Option<f32>,
    pub blur: Option<f32>,
}

#[wasm_bindgen]
impl AdjustParams {
    #[wasm_bindgen(constructor)]
    pub fn new(
        image_data: ImageData,
        brightness: Option<i32>,
        constrast_radio: Option<f32>,
        blur: Option<f32>,
    ) -> Self {
        Self {
            image_data,
            brightness,
            constrast_radio,
            blur,
        }
    }
}

// #[wasm_bindgen]
// impl AdjustParams {
//     #[wasm_bindgen(getter)]
//     pub fn image_data(&self) -> ImageData {
//         self.image_data.clone()
//     }
//     #[wasm_bindgen(setter)]
//     pub fn set_image_data(&mut self, image_data: ImageData) {
//         self.image_data = image_data;
//     }
// }

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn adjust(params: AdjustParams) -> ImageData {
    let mut image_data = ImageFilter::new(params.image_data);
    if params.brightness.is_some() {
        image_data.brighten(params.brightness.unwrap());
    }
    if params.blur.is_some() {
        image_data.blur(params.blur.unwrap());
    }
    if params.constrast_radio.is_some() {
        image_data.contrast(params.constrast_radio.unwrap());
    }
    image_data.to_image_data()
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn canvas_to_gray(image_data: ImageData) -> ImageData {
    ImageFilter::new(image_data).to_gray2().to_image_data()
}

#[wasm_bindgen]
pub fn brighten(image_data: ImageData, value: i32) -> ImageData {
    ImageFilter::new(image_data).brighten(value).to_image_data()
}

#[wasm_bindgen]
pub fn brighten2(image_data: ImageData, value: i32) -> Vec<u8> {
    ImageFilter::new(image_data).brighten(value).to_vec()
}

#[wasm_bindgen]
pub fn contrast(image_data: ImageData, value: f32) -> ImageData {
    ImageFilter::new(image_data).contrast(value).to_image_data()
}
