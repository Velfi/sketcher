//! Currently this is all unused because I went with Kornel's [dssim_core](https://docs.rs/crate/dssim-core)
//! crate instead of rolling my own.

// pub mod ssim;

// use dssim_core::{Val, SsimMap};
// use image::DynamicImage;
// use rgb::RGB8;

// use crate::conversion::ConversionWrapper;

// pub fn calculate_similarity_of_images(
//     image_a: &DynamicImage,
//     image_b: &DynamicImage,
// ) -> (Val, Vec<SsimMap>) {
//     let dssim = dssim_core::new();
//     let bitmap_a: Vec<RGB8> =
//         ConversionWrapper::new(image_a.as_rgba8().expect("image can be represented as rgb")).into();
//     let bitmap_b: Vec<RGB8> =
//         ConversionWrapper::new(image_b.as_rgba8().expect("image can be represented as rgb")).into();
//     let image_a = dssim
//         .create_image_rgb(
//             &bitmap_a,
//             image_a.width() as usize,
//             image_a.height() as usize,
//         )
//         .expect("dssim image can be created from rgb8 slice");
//     let image_b = dssim
//         .create_image_rgb(
//             &bitmap_b,
//             image_b.width() as usize,
//             image_b.height() as usize,
//         )
//         .expect("dssim image can be created from rgb8 slice");

//     dssim.compare(&image_a, image_b)
// }

// #[cfg(test)]
// mod tests {
//     use super::calculate_similarity_of_images;

//     #[test]
//     fn test_same_is_similar() {
//         let image_a = image::open("./image_a.bmp").unwrap();
//         let (similarity, _) = calculate_similarity_of_images(&image_a, &image_a);
//         assert_eq!(
//             0.0, similarity,
//             "expected same image to have similarity of 0.0 but it was {}",
//             similarity
//         );
//     }

//     #[test]
//     fn test_addition_is_similar() {
//         let image_a = image::open("./image_a.bmp").unwrap();
//         let image_b = image::open("./image_b.bmp").unwrap();
//         let (similarity, _) = calculate_similarity_of_images(&image_a, &image_b);
//         assert!(
//             similarity < 0.1,
//             "expected image with addition to have similarity of <0.1 but it was {}",
//             similarity
//         );
//     }

//     #[test]
//     fn test_subtraction_is_similar() {
//         let image_a = image::open("./image_a.bmp").unwrap();
//         let image_c = image::open("./image_c.bmp").unwrap();
//         let (similarity, _) = calculate_similarity_of_images(&image_a, &image_c);
//         assert!(
//             similarity < 0.1,
//             "expected image with subtraction to have similarity of <0.1 but it was {}",
//             similarity
//         );
//     }
// }
