// Copyright 2021 redcar Project Authors. Licensed under Apache-2.0.

use daemon::config::Config;
use daemon::daemon::Daemon;
use daemon::os;

fn main() {
    Daemon::new(Config::new())
        .map(|mut d| {
            d.run();
            os::wait_for_signal(|| d.exit())
        })
        .map_err(|err| panic!("daemon error {:?}", err));
}
