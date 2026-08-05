use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{borrow::Cow, time::Duration};

pub fn new_progress<S>(mp: &MultiProgress, msg: S) -> ProgressBar
where
    S: Into<Cow<'static, str>>,
{
    let pb = mp.add(ProgressBar::new_spinner());

    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    pb.set_message(msg.into());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}
