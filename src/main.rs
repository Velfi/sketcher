mod comparison;
mod conversion;
mod generator;

use generator::{DssimChecker, Generator, LineSketcher};
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();

    let original = image::open("./zelda.png").expect("zelda png exists");
    let sketcher = LineSketcher::new(original.width(), original.height());
    let checker = DssimChecker::new();
    let mut generator = Generator::new(original, sketcher, checker);

    loop {
        if let Some(image) = generator.poll_next() {
            info!("successfully generated a sketch, now saving image to 'output.bmp'...");
            image
                .save("./zelda_sketch.png")
                .expect("saving image was successful");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comparison::calculate_similarity_of_images;

    #[test]
    fn test_same_is_similar() {
        let image_a = image::open("./image_a.bmp").unwrap();
        let (similarity, _) = calculate_similarity_of_images(&image_a, &image_a);
        assert_eq!(
            0.0, similarity,
            "expected same image to have similarity of 0.0 but it was {}",
            similarity
        );
    }

    #[test]
    fn test_addition_is_similar() {
        let image_a = image::open("./image_a.bmp").unwrap();
        let image_b = image::open("./image_b.bmp").unwrap();
        let (similarity, _) = calculate_similarity_of_images(&image_a, &image_b);
        assert!(
            similarity < 0.1,
            "expected image with addition to have similarity of <0.1 but it was {}",
            similarity
        );
    }

    #[test]
    fn test_subtraction_is_similar() {
        let image_a = image::open("./image_a.bmp").unwrap();
        let image_c = image::open("./image_c.bmp").unwrap();
        let (similarity, _) = calculate_similarity_of_images(&image_a, &image_c);
        assert!(
            similarity < 0.1,
            "expected image with subtraction to have similarity of <0.1 but it was {}",
            similarity
        );
    }
}
