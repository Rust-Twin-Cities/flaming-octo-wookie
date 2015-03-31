extern crate rand;
use rand::{thread_rng, Rng};

pub struct Robot {
    // ...
}

impl Robot {
    pub fn new() -> Robot {
        // ...
    }

    pub fn name<'a>(&'a self) -> &'a str {
        // ...
    }

    pub fn reset_name(&mut self) {
        // ...
    }

}
