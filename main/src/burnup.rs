use chrono::prelude::*;

use log::LevelFilter;

use log4rs::{ append::file::FileAppender, encode::pattern::PatternEncoder, config::{Appender, Config, Root}, append::console::ConsoleAppender};

// pub fn to_string(s: &str) -> String {
//   String::from_utf8(s.as_bytes().to_vec())
//     .unwrap_or(String::from(""))
// }


pub fn init_logger() {
  /*
   * 로거 동기화를 위함
   */
  let log_value = dotenv!("LOG_LEVEL");
  let log_level = match log_value {
    "trace" => LevelFilter::Trace,
    "debug" => LevelFilter::Debug,
    "info" => LevelFilter::Info,
    "warn" => LevelFilter::Warn,
    "error" => LevelFilter::Error,
    _ => LevelFilter::Off,
  };

  let stdout = if cfg!(debug_assertions) {
    ConsoleAppender::builder()
      .encoder(Box::new(PatternEncoder::new("[{h({l})}]{d(%H:%M:%S)}> {m}{n}")))
      .build()
  } else {
    ConsoleAppender::builder()
      .encoder(Box::new(PatternEncoder::new("[{h({l})}]{d(%H:%M:%S)}> {m}{n}\n")))
      .build()
  };

	let logfile = FileAppender::builder()
		.encoder(Box::new(PatternEncoder::new("<{d}> LEVEL[{l}] {t}\n    - {m}{n}\n")))
		.build("logs/logs.log").unwrap();

	let config = Config::builder()
		.appender(Appender::builder().build("logfile", Box::new(logfile)))
		.appender(Appender::builder().build("stdout", Box::new(stdout)))
			.build(
				Root::builder()
					.appender("logfile")
					.appender("stdout")
						.build(log_level)
			).unwrap();

	log4rs::init_config(config).unwrap();

	if cfg!(debug_assertions) {
		debug!("DEBUG MODE ENABLED");
	} else {
		info!("PRODUCTION MODE ENABLED");
	}
}

pub fn check_env() -> bool {
  /*
   * 서버 환경 체크를 위한 루틴
   * 서버 시간대가 KST 인지 확인
   * 서버 시간이 동기화 되어있는지 확인
   */
  let utc_diff = chrono::Local.timestamp(0, 0).offset().fix().local_minus_utc();
  let utc_diff = utc_diff / 3600;
  if utc_diff != 9 {
    error!("[!] UTC DIFF IS NOT 9 HOUR [KST]");
    std::process::exit(1);
  }
  

  true
}
