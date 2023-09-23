use deck_analyzer::combinations::combination_n;

use indicatif::{ProgressBar, ProgressIterator};
use itertools::Itertools;
// use std::time::Duration;

fn main() {
    let combination = (0..50).combinations(5);

    let pb = ProgressBar::new(combination_n(50, 5).unwrap());

    for _ in combination.progress_with(pb) {
        // std::thread::sleep(Duration::from_(1));
    }
}
