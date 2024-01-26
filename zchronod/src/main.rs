mod cli;

use std::{env, error};
use clap::{crate_description, crate_name, ArgMatches};
use crate::cli::set_clap;
use tokio::runtime::Runtime;


fn main() -> Result<(), Box<dyn error::Error>> {
    // env::set_var("RUST_LOG", "debug");

    chrono_logger::init_chrono_logger();
    let matches = set_clap(
        crate_name!(),
        crate_description!(),
    ).get_matches();

    unsafe { process_cmd(&matches)?; }

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // wait
        tokio::signal::ctrl_c().await.unwrap();
    });
    Ok(())
}

unsafe fn process_cmd(matches: &ArgMatches<'_>) -> Result<(), Box<dyn error::Error>> {
    // to add handle cmd
    if let Some(f) = matches.value_of("log_path") {
        chrono_logger::init_chrono_logger_with_path(f, &find_env("RUST_LOG"))
    } else {
        chrono_logger::init_chrono_with_filter("info")
    }
    let config = matches.value_of("config").unwrap_or("./chronod.yaml");
    process::init_chrono_node(config);
    Ok(())
}

fn find_env(env: &str) -> String {
    if let Ok(path_value) = env::var(env) {
        path_value
    } else {
        String::new()
    }
}