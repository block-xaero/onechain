use log::{debug, info, warn};
use simplelog::*;
use std::fs::File;
use std::io::Write;

#[cfg(feature = "logging")]
pub fn init_simple_logger() {
    use simplelog::*;
    use std::fs::File;

    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("library.log").unwrap()),
    ])
    .expect("Failed to initialize logger");
}