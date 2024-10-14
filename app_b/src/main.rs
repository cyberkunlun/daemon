use std::{thread::sleep, time::Duration};

use dj_log::log_init;

fn main() {
    log_init(log::Level::Info);
    log::info!("app b start");
    loop {
        log::info!("app b running");
        sleep(Duration::from_secs(1));
    }
}
