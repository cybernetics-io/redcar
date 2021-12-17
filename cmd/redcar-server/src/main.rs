// Copyright 2021 Redcar Project Authors. Licensed under Apache-2.0.

use redcar::config::Config;
use redcar::daemon::Daemon;
use redcar::os;

fn main() {
    Daemon::new(Config::new())
        .map(|mut d| {
            d.run();
            os::wait_for_signal(|| d.exit())
        })
        .map_err(|err| panic!("daemon error {:?}", err));
}
