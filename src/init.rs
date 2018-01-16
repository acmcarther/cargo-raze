use fern;
use chrono;
use log;
use std;

pub fn global_logger() {
  fern::Dispatch::new()
    .format(|out, message, record| {
        out.finish(format_args!("{} {} [{}] {}",
            record.level(),
            chrono::Local::now()
                .format("%m%d %H:%M:%S%.6f"),
            record.target(),
            message))
    })
    .level(log::LogLevelFilter::Debug)
    .chain(std::io::stdout())
    .apply().unwrap();
}
