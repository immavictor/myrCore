// use core::fmt;
// use crate::console;
use log::{self, Level, LevelFilter, Log, Metadata, Record};//引入了Rust的日志模块log，定义了日志级别Level、日志级别过滤器LevelFilter、日志记录器Log、元数据Metadata和日志记录Record。
pub fn init() {
    static LOGGER: Logger = Logger;//设置了一个静态的Logger实例
    log::set_logger(&LOGGER).unwrap();//尝试将LOGGER设置为全局日志记录器，并在设置成功时继续执行，如果设置失败，则程序会崩溃。
    log::set_max_level(match option_env!("LOG") {//option_env! 宏尝试获取名为 LOG 的环境变量的值，如果该环境变量存在，它将返回 Some(value)
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Off,//如果LOG环境变量没有设置这意味着没有任何日志会被记录
    });
}

struct Logger;
impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }//判断是否应该记录给定元数据的日志。
    
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        print!("\x1b[{}m", level_to_color_code(record.level()));
        println!("[{}] {}", record.level(), record.args());
        print!("\x1b[0m") //重置文本颜色到默认
    }

    fn flush(&self) {}//清空日志缓冲区
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack,数字是 ANSI 转义代码中的前景色代码与 \x1b[ 结合使用，可以改变控制台输出的文本颜色。
    }
}