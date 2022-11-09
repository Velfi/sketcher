use dssim_core::{Val, SsimMap};
use image::DynamicImage;
use rgb::RGB8;

use crate::conversion::ConversionWrapper;

pub mod ssim;

pub fn calculate_similarity_of_images(
    image_a: &DynamicImage,
    image_b: &DynamicImage,
) -> (Val, Vec<SsimMap>) {
    let dssim = dssim_core::new();
    let bitmap_a: Vec<RGB8> =
        ConversionWrapper::new(image_a.as_rgb8().expect("image can be represented as rgb")).into();
    let bitmap_b: Vec<RGB8> =
        ConversionWrapper::new(image_b.as_rgb8().expect("image can be represented as rgb")).into();
    let image_a = dssim
        .create_image_rgb(
            &bitmap_a,
            image_a.width() as usize,
            image_a.height() as usize,
        )
        .expect("dssim image can be created from rgb8 slice");
    let image_b = dssim
        .create_image_rgb(
            &bitmap_b,
            image_b.width() as usize,
            image_b.height() as usize,
        )
        .expect("dssim image can be created from rgb8 slice");

    dssim.compare(&image_a, image_b)
}
