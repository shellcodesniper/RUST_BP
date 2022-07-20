#[macro_use] extern crate log;
#[macro_use] extern crate dotenvhack;

extern crate dotenv;
extern crate chrono;
extern crate chrono_tz;

mod lib;
mod burnup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  burnup::init_logger();
  burnup::check_env();


  info!("[*] START PROGRAM (LEVEL: {})", dotenv!("RUST_ENV"));




  Ok(())
}

