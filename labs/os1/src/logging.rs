use log::{Log};

struct SimpleLogger;


impl Log for SimpleLogger{
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    // 主要就是这里用于输出相关的设置
    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return ;
        }
        let color = match record.level() {
            log::Level::Error => 31,
            log::Level::Warn => 93,
            log::Level::Info => 34,
            log::Level::Debug => 32,
            log::Level::Trace => 90,
        };
        println!(
            // 控制台颜色控制
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        );
    }

    fn flush(&self) {
        todo!()
    }
}


// 设置一个静态的logger,然后配置
pub fn init(){
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    // options_env 可以获取环境变量中的值，然后通过运行是设定临时的环境变量来达到控制日志输出级别
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => log::LevelFilter::Error,
        Some("WARN") => log::LevelFilter::Warn,
        Some("INFO") => log::LevelFilter::Info,
        Some("DEBUG") => log::LevelFilter::Debug,
        Some("TRACE") => log::LevelFilter::Trace,
        _ => log::LevelFilter::Off,
    });
}