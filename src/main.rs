#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod term;
use std::process;

fn setup_logging(verbosity: u64) {
    let mut base_config = fern::Dispatch::new();

    base_config = match verbosity {
        0 => base_config.level(log::LevelFilter::Warn),
        1 => base_config.level(log::LevelFilter::Info),
        2 => base_config.level(log::LevelFilter::Debug),
        _3_or_more => base_config.level(log::LevelFilter::Trace),
    };

    // Separate file config so we can include year, month and day in file logs
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {} [{}] {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.target(),
                record.level(),
                message
            ));
        })
        .chain(fern::log_file("mini.log").unwrap());

    base_config.chain(file_config).apply().unwrap();
}

fn main() -> std::io::Result<()> {
    setup_logging(3);

    log::debug!("Starting program with pid: {}", process::id());
    let mut editor = editor::Editor::default();
    editor.run()
}
