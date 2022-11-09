mod checker;
mod sketcher;

pub use checker::dssim::DssimChecker;
pub use sketcher::line::LineSketcher;

use checker::Checker;
use sketcher::Sketcher;

pub struct Generator<I, S, C> {
    original: I,
    sketcher: S,
    checker: C,
}

impl<I, S, C, T> Generator<I, S, C>
where
    S: Sketcher<Image = I>,
    C: Checker<Image = I, Similarity = T>,
{
    pub fn new(original: I, sketcher: S, checker: C) -> Self
    where
        T: PartialEq + PartialOrd,
    {
        Self {
            original,
            sketcher,
            checker,
        }
    }

    pub fn poll_next(&mut self) -> Option<I> {
        if self.is_similar_enough() {
            return Some(self.sketcher.image());
        }

        self.sketcher.next();

        

        None
    }

    fn is_similar_enough(&self) -> bool {
        todo!()
    }
}
