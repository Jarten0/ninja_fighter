use log::Log;

pub(crate) static LOGGER: Logger = Logger {
    target_whitelist: Vec::new(),
};

pub struct Logger {
    target_whitelist: Vec<&'static str>,
}

impl Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        false
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        match record.level() {
            log::Level::Error => println!("Error: [{}]: {}", record.file().unwrap(), record.args()),
            log::Level::Warn => println!("Warn: [{}]: {}", record.file().unwrap(), record.args()),
            log::Level::Info => println!("Info: [{}]: {}", record.file().unwrap(), record.args()),
            log::Level::Debug => println!("Debug: [{}]: {}", record.file().unwrap(), record.args()),
            log::Level::Trace => println!("Trace: [{}]: {}", record.file().unwrap(), record.args()),
        }
    }

    fn flush(&self) {
        todo!()
    }
}
