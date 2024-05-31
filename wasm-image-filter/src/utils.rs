// // 去色
// pub fn desaturate_image(image: DynamicImage) -> DynamicImage {
//     let (width, height) = image.dimensions();
//     let mut new_image = ImageBuffer::new(width, height);

//     for y in 0..height {
//         for x in 0..width {
//             let pixel = image.get_pixel(x, y);
//             let gray_value = (0.2126 * pixel[0] as f32
//                 + 0.7152 * pixel[1] as f32
//                 + 0.0722 * pixel[2] as f32) as u8;
//             new_image.put_pixel(x, y, Rgba([gray_value, gray_value, gray_value, pixel[3]]));
//         }
//     }

//     DynamicImage::ImageRgba8(new_image)
// }
