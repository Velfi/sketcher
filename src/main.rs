mod conversion;
mod generator;

use generator::sketcher::image_proc_sketcher;
use generator::{DssimChecker, Generator, ImageProcSketcher, Options};
use humantime::format_duration;
use std::time::{Duration, Instant};
use tracing::info;

use crate::generator::sketcher;

const MAX_ITERATIONS: u64 = 30_000;
const DESIRED_SIMILARITY: f64 = 0.3;
#[cfg(not(debug_assertions))]
/// Each iteration takes about 0.45s on my machine in release mode
const ESTIMATE_MULTIPLIER: f64 = 0.045;
#[cfg(debug_assertions)]
/// Each iteration takes about 1.23s on my machine in debug mode
const ESTIMATE_MULTIPLIER: f64 = 1.229;

const INPUT_PATH: &str = "./zelda.jpg";
const OUTPUT_PATH: &str = "./zelda_sketch.jpg";

fn main() {
    tracing_subscriber::fmt::init();
    let run_timer = Instant::now();
    let runtime_duration = Duration::from_secs_f64(MAX_ITERATIONS as f64 * ESTIMATE_MULTIPLIER);
    let estimated_runtime = format_duration(runtime_duration);
    let finish_time = chrono::offset::Local::now()
        .checked_add_signed(chrono::Duration::from_std(runtime_duration).unwrap())
        .unwrap();

    let original = image::open(INPUT_PATH)
        .expect("input path exists")
        .into_rgba8();
    info!("original image loaded, now creating a generator");
    let sketcher = ImageProcSketcher::new(
        original.width(),
        original.height(),
        image_proc_sketcher::Options {
            background: sketcher::Background::Black,
            color_mode: sketcher::ColorMode::white(),
            style: image_proc_sketcher::Style::triangles(),
        },
    );
    let checker = DssimChecker::new(original);
    let mut generator = Generator::new(
        sketcher,
        checker,
        Options {
            max_iters: Some(MAX_ITERATIONS),
            desired_similarity: Some(DESIRED_SIMILARITY),
        },
    );

    info!("now generating a sketch with a maximum of {MAX_ITERATIONS} iterations");
    info!("estimated runtime: {estimated_runtime} (will finish at {finish_time})");

    loop {
        if let Some(image) = generator.poll_next() {
            info!("saving sketch to '{OUTPUT_PATH}'...");
            image
                .save(OUTPUT_PATH)
                .expect("saving image was successful");
            break;
        }
    }

    info!("completed sketch in {:?}", run_timer.elapsed());
}
