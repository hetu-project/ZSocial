use {
    lazy_static::lazy_static,
    std::sync::{Arc, RwLock},
};

// global chrono_log config
lazy_static! {
    static ref CLOGGER: Arc<RwLock<env_logger::Logger>> =
        Arc::new(RwLock::new(env_logger::Logger::from_default_env()));
}

struct ChronoLogger {}

impl log::Log for ChronoLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        CLOGGER.read().unwrap().enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        CLOGGER.read().unwrap().log(record);
    }

    fn flush(&self) {}
}



/// * `logfile_path`: path to store log file
/// * `filter`: log level
///
/// # Examples
/// init_chrono_logger_with_path("./xx/chrono.log", "debug")
pub fn init_chrono_logger_with_path(logfile_path: &str, filter: &str) {
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(logfile_path)
        .expect("Failed to open log file");
    let logger = env_logger::Builder::from_env(env_logger::Env::new().default_filter_or(filter))
        .format_timestamp_nanos()
        .target(env_logger::Target::Pipe(Box::new(file)))
        .build();
    replace_logger(logger);
}

fn replace_logger(logger: env_logger::Logger) {
    log::set_max_level(logger.filter());
    *CLOGGER.write().unwrap() = logger;
    let _ = log::set_boxed_logger(Box::new(ChronoLogger {}));
}

pub fn init_chrono_logger() {
    init_chrono_with_filter("error");
}

pub fn init_chrono_with_filter(filter: &str) {
    let logger = env_logger::Builder::from_env(env_logger::Env::new().default_filter_or(filter))
        .format_timestamp_nanos()
        .build();
    replace_logger(logger);
}