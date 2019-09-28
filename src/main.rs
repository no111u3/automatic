use std::path::PathBuf;
use std::process;

use clap::{App, Arg};

use automatic::run::Run;
use automatic::script::Script;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("script")
                .short("r")
                .long("run")
                .value_name("SCRIPT NAME")
                .help("Script to run")
                .takes_value(true),
        )
        .get_matches();

    if let Some(script_name) = matches.value_of("script") {
        let script = Script::new(PathBuf::from(script_name));

        let runner = script.parse().unwrap();

        let result = match runner.run().status() {
            Err(e) => {
                println!("Failed to run script with error: {}", e);
                process::exit(1);
            }
            Ok(ok) => ok,
        };

        if !result.success() {
            println!(
                "Unsucessful run with error code: {}",
                result.code().unwrap_or(0)
            );
        };
    } else {
        println!("No provided script path");
        process::exit(1);
    }
}
