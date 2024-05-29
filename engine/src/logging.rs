use std::fmt::Debug;

pub struct Logger;
pub(crate) static LOGGER: Logger = Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        // find the crate name, and if calling from a module, use just the name instead of the full module path for checking with the whitelist
        let caller_module_path = &_metadata.target();

        let module_break = match caller_module_path.find(':') {
            Some(some) => some,
            None => caller_module_path.len(),
        };

        let crate_name = caller_module_path.get(0..module_break).unwrap();

        if crate_name == "wgpu_core" {
            return false;
        }
        // dbg!(crate_name);

        ["engine", "game", "editor", "components"].contains(&crate_name)
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        match record.level() {
            log::Level::Error => println!(
                "! Error: [{}]: {}",
                record.file().unwrap().to_owned() + ":" + &record.line().unwrap().to_string(),
                record.args()
            ),
            log::Level::Warn => println!(
                "~ Warn : [{}]: {}",
                record.file().unwrap().to_owned() + ":" + &record.line().unwrap().to_string(),
                record.args()
            ),
            log::Level::Info => println!(
                "- Info : [{}]: {}",
                record.file().unwrap().to_owned() + ":" + &record.line().unwrap().to_string(),
                record.args()
            ),
            log::Level::Debug => println!(
                "? Debug: [{}]: {}",
                record.file().unwrap().to_owned() + ":" + &record.line().unwrap().to_string(),
                record.args()
            ),
            log::Level::Trace => println!(
                "  Trace: [{}]: {}",
                record.file().unwrap().to_owned() + ":" + &record.line().unwrap().to_string(),
                record.args()
            ),
        }
    }

    fn flush(&self) {
        todo!()
    }
}

pub trait LogData {
    fn log(&self) -> ();
}

impl<T> LogData for T
where
    T: Debug,
{
    fn log(&self) {
        dbg!(self);
    }
}
