#![allow(dead_code)]

use image::{DynamicImage, GenericImageView, SubImage};

const ALPHA_WEIGHT: f64 = 1.0;
const BETA_WEIGHT: f64 = 1.0;
const GAMMA_WEIGHT: f64 = 1.0;
// "For an image, it is typically calculated using a sliding Gaussian window of size 11x11 or a block window of size 8×8."
// https://en.wikipedia.org/wiki/Structural_similarity#Application_of_the_formula
const WINDOW_SIZE: u32 = 11;
// The dynamic range of the pixel-values (typically this is (2^bits per pixel) − 1;
const L: f64 = 255.0;
const K1: f64 = 0.01;
const K2: f64 = 0.03;
const C1: f64 = (K1 * L) * (K1 * L);
const C2: f64 = (K2 * L) * (K2 * L);

pub fn calculate_structural_similarity_of_images(
    image_a: &DynamicImage,
    image_b: &DynamicImage,
) -> f64 {
    assert_eq!(
        image_a.dimensions(),
        image_b.dimensions(),
        "the images must be the same size"
    );
    assert!(image_a.dimensions().0 >= WINDOW_SIZE, "the images must have a width ({}px) greater than or equal to the window size ({WINDOW_SIZE})", image_a.dimensions().0);
    assert!(image_a.dimensions().1 >= WINDOW_SIZE, "the images must have a height ({}px) greater than or equal to the window size ({WINDOW_SIZE})", image_a.dimensions().1);

    // When creating sub-images from image, we don't want to go beyond the dimensions of the image
    let max_window_x = image_a.dimensions().0 - WINDOW_SIZE;
    let max_window_y = image_a.dimensions().1 - WINDOW_SIZE;

    let mut ssim = None;

    for window_origin_y in 0..max_window_y {
        for window_origin_x in 0..max_window_x {
            let window_a = image_a.view(window_origin_x, window_origin_y, WINDOW_SIZE, WINDOW_SIZE);
            let window_b = image_b.view(window_origin_x, window_origin_y, WINDOW_SIZE, WINDOW_SIZE);

            let luminance_similarity = luminance_comparison(&window_a, &window_b);
            let contrast_similarity = contrast_comparison(&window_a, &window_b);
            let structure_similarity = structure_comparison(&window_a, &window_b);

            let window_ssim = luminance_similarity.powf(ALPHA_WEIGHT)
                * contrast_similarity.powf(BETA_WEIGHT)
                * structure_similarity.powf(GAMMA_WEIGHT);

            ssim = ssim.map_or(Some(window_ssim), |ssim| Some((ssim + window_ssim) / 2.0));
        }
    }

    ssim.expect("SSIM has been calculated")
}

fn luminance_comparison<I>(sub_image_a: &SubImage<I>, sub_image_b: &SubImage<I>) -> f64 {
    // let pixel_sample_mean_of_a = pixel_sample_mean(sub_image_a);
    // let pixel_sample_mean_of_b = pixel_sample_mean(sub_image_b);

    // (2.0 * pixel_sample_mean_of_a * pixel_sample_mean_of_b + C1)
    //     / (pixel_sample_mean_of_a.powi(2) + pixel_sample_mean_of_b.powi(2) + C1)
    todo!()
}

fn contrast_comparison<I>(sub_image_a: &SubImage<I>, sub_image_b: &SubImage<I>) -> f64 {
    let variance_of_a = variance(sub_image_a);
    let variance_of_b = variance(sub_image_b);

    (2.0 * variance_of_a * variance_of_b + C2)
        / (variance_of_a.powi(2) + variance_of_b.powi(2) + C2)
}

fn structure_comparison<I>(sub_image_a: &SubImage<I>, sub_image_b: &SubImage<I>) -> f64 {
    let covariance_of_ab = covariance(sub_image_a, sub_image_b);
    let variance_of_a = variance(sub_image_a);
    let variance_of_b = variance(sub_image_b);
    let c3 = C2 / 2.0;

    (covariance_of_ab + c3) / (variance_of_a * variance_of_b + c3)
}

fn pixel_sample_mean<I, P>(sub_image: &I) -> f64
where
    I: GenericImageView<Pixel = P>,
{
    todo!()
}

fn variance<I>(sub_image: &SubImage<I>) -> f64 {
    todo!()
}

fn covariance<I>(sub_image_a: &SubImage<I>, sub_image_b: &SubImage<I>) -> f64 {
    todo!()
}
