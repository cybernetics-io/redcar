// Copyright 2021 Redcar Project Authors. Licensed under Apache-2.0.

use daemon::config::Config;
use daemon::daemon::Daemon;
use daemon::os;

pub fn load_config() -> Config {
    Config::default()
}

pub fn start_daemon(c: Config) {
    let dm = Daemon::new(c);
    match dm {
        Ok(mut d) => {
            d.run();
        }
        Err(err) => {
            panic!(err)
        }
    }
}
