pub mod checker;
pub mod sketcher;

use std::cmp::Ordering;

pub use checker::dssim::DssimChecker;
pub use sketcher::image_proc_sketcher::ImageProcSketcher;

use checker::Checker;
use sketcher::Sketcher;
use tracing::{info, trace};

pub struct Options {
    pub max_iters: Option<u64>,
    pub desired_similarity: Option<f64>,
}

impl Options {
    pub fn are_valid(&self) -> bool {
        self.max_iters.is_some() || self.desired_similarity.is_some()
    }
}

pub struct Generator<S, C> {
    sketcher: S,
    checker: C,
    iter_count: u64,
    successful_iterations: u64,
    options: Options,
    last_similarity: f64,
}

impl<I, S, C> Generator<S, C>
where
    I: Clone,
    S: Sketcher<Image = I>,
    for<'b> C: Checker<Original = I, Other<'b> = &'b I, Similarity = f64>,
{
    pub fn new(sketcher: S, checker: C, options: Options) -> Self {
        if !options.are_valid() {
            panic!("generator will never stop running with the given options");
        }

        Self {
            sketcher,
            checker,
            iter_count: 0,
            successful_iterations: 0,
            // The first similarity check will always succeed, that's OK
            last_similarity: std::f64::MAX,
            options,
        }
    }

    pub fn poll_next(&mut self) -> Option<I> {
        trace!("iteration {}", self.iter_count);
        if self.is_similar_enough() || self.is_time_to_quit() {
            let final_similarity = self.checker.similarity(self.sketcher.image());
            let desired_similarity = self.options.desired_similarity.unwrap_or(0.0);
            if final_similarity <= desired_similarity {
                info!("finished early with final similarity of {final_similarity} (desired was {desired_similarity})",
            );
            }
            info!(
                "after {} iterations (#{} successful), it's time to quit",
                self.iter_count, self.successful_iterations
            );
            info!("stopping generator and returning the image");
            return Some(self.sketcher.image().clone());
        }

        self.sketcher.next();
        let similarity = self.checker.similarity(self.sketcher.image());

        match similarity.partial_cmp(&self.last_similarity) {
            // if the similarity score is lower, that means we're getting closer to the original
            Some(Ordering::Less) | Some(Ordering::Equal) => {
                self.last_similarity = similarity;
                self.successful_iterations += 1;
                trace!(
                    "next iteration was more similar (success #{}), continuing...",
                    self.successful_iterations
                );
            }
            // if the similarity score is higher, that means we're getting further from the original
            Some(Ordering::Greater) => {
                trace!("next iteration was less similar, undoing...");
                self.sketcher.undo();
            }
            None => {
                unreachable!("similarity is must not be NaN");
            }
        }

        self.iter_count += 1;
        None
    }

    fn is_time_to_quit(&self) -> bool {
        self.options
            .max_iters
            .map_or(false, |max_iters| self.iter_count == max_iters)
    }

    fn is_similar_enough(&self) -> bool {
        self.options
            .desired_similarity
            .map_or(false, |desired_similarity| {
                let similarity = self.checker.similarity(self.sketcher.image());
                // The lower the similarity score,
                // the closer the scketched image is to the original image
                similarity <= desired_similarity
            })
    }
}
