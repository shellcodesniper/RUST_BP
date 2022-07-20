use chrono::prelude::*;

use log::LevelFilter;

use log4rs::{ append::file::FileAppender, encode::pattern::PatternEncoder, config::{Appender, Config, Root}, append::console::ConsoleAppender};

pub fn init_logger() {
  /*
   * 로거 동기화를 위함
   */
  let log_value = dotenv!("LOG_LEVEL");
  std::env::set_var("RUST_LOG", log_value);

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
						.build(LevelFilter::Debug)
			).unwrap();

	log4rs::init_config(config).unwrap();

	if cfg!(debug_assertions) {
		debug!("DEBUG MODE ENABLED");
	} else {
		info!("PRODUCTION MODE ENABLED");
	}
}

pub fn check_env() -> bool {
  // NOTE: 서버 환경 체크
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
