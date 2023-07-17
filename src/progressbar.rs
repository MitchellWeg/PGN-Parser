use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::{cmp::min, fmt::Write};

pub struct CustomProgressBar {
    progress_bar: ProgressBar,
    total_size: u64,
}

impl CustomProgressBar {
    pub fn new(total_size: u64) -> CustomProgressBar {
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
            .progress_chars("#>-"));

        CustomProgressBar {
            progress_bar: pb,
            total_size,
        }
    }
    pub fn update(&self, offset: u64) {
        let new = min(offset, self.total_size);
        self.progress_bar.set_position(new);
    }
}
