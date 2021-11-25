// Copyright 2021 redcar Project Authors. Licensed under Apache-2.0.

use std::path::PathBuf;
use structopt::StructOpt;

use utils::Error;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "redcar")]
pub struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// admin_level to consider
    #[structopt(short, long)]
    level: Vec<String>,

    /// Set home path
    #[structopt(short, long, default_value = "/usr/local/redcar")]
    home: String,

    /// Set host
    #[structopt(short, long, default_value = "127.0.0.1:8519")]
    addr: String,

    /// Set host
    #[structopt(short, long, default_value = "4")]
    thread_number: usize,

    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    // the long option will be translated by default to kebab case,
    // i.e. `--nb-cars`.
    /// Number of cars
    #[structopt(short = "c", long)]
    nb_cars: Option<i32>,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Config {
    opt: Opt,
}

impl Config {
    pub fn new() -> Config {
        Config {
            opt: Opt::from_args(),
        }
    }

    pub fn get_host(&self) -> String {
        self.opt.addr.clone()
    }

    pub fn get_home(&self) -> String {
        self.opt.home.clone()
    }

    pub fn get_thread_number(&self) -> usize {
        self.opt.thread_number
    }

    /// test
    pub fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}
