use chrono::Local;
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

/// 日志记录器
pub struct Logger {
    /// 日志级别
    level: LevelFilter,
    /// 日志文件
    file: Option<Mutex<File>>,
}

impl Logger {
    /// 创建新的日志记录器
    pub fn new(level: LevelFilter, file: Option<PathBuf>) -> Result<Self, String> {
        let file = if let Some(path) = file {
            Some(Mutex::new(
                File::create(path).map_err(|e| format!("创建日志文件失败: {}", e))?,
            ))
        } else {
            None
        };

        Ok(Self { level, file })
    }

    /// 格式化日志记录
    fn format_log(&self, record: &Record) -> String {
        let level = record.level();
        let target = record.target();
        let args = record.args();
        let time = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        format!("[{}] {} {} - {}", time, level, target, args)
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let log = self.format_log(record);
            println!("{}", log);

            if let Some(file) = &self.file {
                if let Ok(mut file) = file.lock() {
                    let _ = writeln!(file, "{}", log);
                }
            }
        }
    }

    fn flush(&self) {
        if let Some(file) = &self.file {
            if let Ok(mut file) = file.lock() {
                let _ = file.flush();
            }
        }
    }
}

/// 初始化日志记录器
pub fn init(level: &str, file: Option<PathBuf>) -> Result<(), String> {
    let level = match level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    let logger = Logger::new(level, file)?;
    log::set_boxed_logger(Box::new(logger)).map_err(|e| format!("设置日志记录器失败: {}", e))?;
    log::set_max_level(level);
    Ok(())
}

/// 获取日志级别
pub fn get_level() -> Level {
    log::max_level().to_level().unwrap_or(Level::Info)
}

/// 设置日志级别
pub fn set_level(level: Level) {
    log::set_max_level(level.to_level_filter());
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_logger_new() {
        let logger = Logger::new(LevelFilter::Info, None).unwrap();
        assert_eq!(logger.level, LevelFilter::Info);
        assert!(logger.file.is_none());
    }

    #[test]
    fn test_logger_new_with_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.log");
        let logger = Logger::new(LevelFilter::Info, Some(path)).unwrap();
        assert_eq!(logger.level, LevelFilter::Info);
        assert!(logger.file.is_some());
    }

    #[test]
    fn test_logger_format_log() {
        let logger = Logger::new(LevelFilter::Info, None).unwrap();
        let record = log::Record::builder()
            .level(Level::Info)
            .target("test")
            .args(format_args!("test message"))
            .build();
        let log = logger.format_log(&record);
        assert!(log.contains("INFO"));
        assert!(log.contains("test"));
        assert!(log.contains("test message"));
    }

    #[test]
    fn test_init() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.log");
        assert!(init("info", Some(path)).is_ok());
        assert_eq!(get_level(), Level::Info);
    }

    #[test]
    fn test_set_level() {
        set_level(Level::Debug);
        assert_eq!(get_level(), Level::Debug);
    }
}
