use clap::{AppSettings, Parser};
use client::client::Client;
use client::config::Config;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Parser)]
#[clap(version = "1.0", author = "Meng S. <meng19850812@gmail.com>")]
struct Opts {
    /// Some input. it's required to be used
    input: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    /// A host name for redcar server
    #[clap(short, long, default_value = "http://127.0.0.1:8519")]
    host: String,
    #[clap(subcommand)]
    sub_cmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(version = "1.3", author = "Meng S. <meng19850812@gmail.com>")]
    Put(Put),
    #[clap(version = "1.3", author = "Meng S. <meng19850812@gmail.com>")]
    Range(Range),
}

/// Put key-value data to redcar server, example: "foo bar"
#[derive(Parser)]
struct Put {
    #[clap(short)]
    debug: bool,
}

/// Get value by key from redcar server, example: "foo"
#[derive(Parser)]
struct Range {
    #[clap(short)]
    debug: bool,
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();

    let mut client = Client::new(&Config {
        host: string_to_static_str(opts.host),
    });

    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be ridiculous"),
    }

    match opts.sub_cmd {
        SubCommand::Put(s) => {
            let pair = opts.input.split(" ");
            let kv: Vec<&str> = pair.collect();
            client
                .put(Vec::from(kv[0].as_bytes()), Vec::from(kv[1].as_bytes()))
                .await;
            if s.debug {
                println!("OK");
            }
        }
        SubCommand::Range(s) => {
            let vv = client.range(opts.input.into_bytes()).await;
            for v in vv.iter() {
                println!("{:?}", String::from_utf8(v.to_vec()).unwrap())
            }
        }
    }
}
