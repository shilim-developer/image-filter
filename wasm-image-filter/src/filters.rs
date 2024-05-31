use image::RgbaImage;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageDecoder, Rgba};
use std::{result, str};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::js_sys::{ArrayBuffer, Object};
use web_sys::{js_sys::wasm_bindgen, ImageData};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u8(a: Vec<u8>);
}

pub struct ImageFilter {
    image_mut: DynamicImage,
}

impl ImageFilter {
    pub fn new(data: ImageData) -> ImageFilter {
        let width = data.width();
        let height = data.height();
        let raw_pixels = data.data().to_vec();
        let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
        let image = DynamicImage::ImageRgba8(img_buffer);
        ImageFilter { image_mut: image }
    }
    pub fn to_gray(&self) -> ImageFilter {
        let mut last_mut = self.image_mut.clone();
        let width = last_mut.width();
        let height = last_mut.height();
        for x in 0..width {
            for y in 0..height {
                let pixel = self.image_mut.get_pixel(x, y);
                let r = pixel[0] as f32;
                let g = pixel[1] as f32;
                let b = pixel[2] as f32;
                let a = pixel[3];
                let gray = (r * 0.3 + g * 0.59 + b * 0.11) as u8;
                let new_pixel = image::Rgba([gray, gray, gray, a]);
                last_mut.put_pixel(x, y, new_pixel)
            }
        }
        ImageFilter {
            image_mut: last_mut,
        }
    }
    pub fn to_gray2(&self) -> ImageFilter {
        let mut last_mut = self.image_mut.clone();
        let width = last_mut.width();
        let height = last_mut.height();
        for y in 0..height {
            for x in 0..width {
                let pixel = self.image_mut.get_pixel(x, y);
                let gray_value = (0.2126 * pixel[0] as f32
                    + 0.7152 * pixel[1] as f32
                    + 0.0722 * pixel[2] as f32) as u8;
                last_mut.put_pixel(x, y, Rgba([gray_value, gray_value, gray_value, pixel[3]]));
            }
        }
        ImageFilter {
            image_mut: last_mut,
        }
    }

    pub fn to_image_data(&self) -> ImageData {
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.image_mut.to_rgba8().to_vec().clone()),
            self.image_mut.width(),
            self.image_mut.height(),
        )
        .unwrap()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.image_mut.to_rgba8().into_vec()
    }

    pub fn dynamic_image_to_u8_array(dynamic_image: &DynamicImage) -> Vec<u8> {
        // Convert the DynamicImage to RGBA8 format
        let mut rgba_image = dynamic_image.clone().into_rgba8().into_vec();

        rgba_image
    }

    pub fn brighten(&mut self, value: i32) -> &ImageFilter {
        self.image_mut = DynamicImage::brighten(&self.image_mut, value);
        self
    }

    pub fn blur(&mut self, value: f32) -> &ImageFilter {
        self.image_mut = DynamicImage::blur(&self.image_mut, value);
        self
    }

    pub fn contrast(&mut self, value: f32) -> &ImageFilter {
        self.image_mut = DynamicImage::adjust_contrast(&self.image_mut, value);
        self
    }
}
