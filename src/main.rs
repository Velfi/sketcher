mod conversion;
mod generator;

use generator::sketcher::line::Style;
use generator::{DssimChecker, Generator, LineSketcher, Options};
use humantime::format_duration;
use std::time::{Duration, Instant};
use tracing::info;

const MAX_ITERATIONS: u64 = 30_000;
const DESIRED_SIMILARITY: f64 = 0.3;
#[cfg(not(debug_assertions))]
/// Each iteration takes about 0.45s on my machine in release mode
const ESTIMATE_MULTIPLIER: f64 = 0.045;
#[cfg(debug_assertions)]
/// Each iteration takes about 1.23s on my machine in debug mode
const ESTIMATE_MULTIPLIER: f64 = 1.229;

fn main() {
    tracing_subscriber::fmt::init();
    let run_timer = Instant::now();
    let runtime_duration = Duration::from_secs_f64(MAX_ITERATIONS as f64 * ESTIMATE_MULTIPLIER);
    let estimated_runtime = format_duration(runtime_duration);
    let finish_time = chrono::offset::Local::now()
        .checked_add_signed(chrono::Duration::from_std(runtime_duration).unwrap())
        .unwrap();

    let original = image::open("./zelda.png")
        .expect("zelda png exists")
        .into_rgba8();
    info!("original image loaded, now creating a generator");
    let sketcher = LineSketcher::new(original.width(), original.height(), Style::CrossHatch);
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
            info!("saving sketch to './zelda_sketch.png'...");
            image
                .save("./zelda_sketch.png")
                .expect("saving image was successful");
            break;
        }
    }

    info!("completed sketch in {:?}", run_timer.elapsed());
}
