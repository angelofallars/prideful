use clap::{App, Arg};
use std::io;
extern crate clap;
mod config;
mod flag;
use crate::flag::{Flag, Width};
use config::Error;

fn main() -> Result<(), io::Error> {
    let matches = App::new("prideful")
        .version("0.1")
        .about("A configurable TUI Pride flag generator.")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true)
                .help("Width of the flag in terms of terminal blocks."),
        )
        .arg(
            Arg::with_name("compact")
                .long("compact")
                .help("Print a smaller version of the flag."),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .help("Path of the configuration file to use."),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("List available flags."),
        )
        .arg(
            Arg::with_name("flag")
                .takes_value(true)
        )
        .get_matches();

    let config_parse_result: Result<(String, Vec<Flag>), Error> = match matches.value_of("config") {
        Some(path) => config::load_config_from_path(path),
        None => config::load_config(),
    };

    if config_parse_result.is_err() {
        let error = config_parse_result.unwrap_err();

        match error {
            config::Error::FileNotFound => {
                println!("ERROR: config file not found.");
            }
            config::Error::MakeDir => {
                println!("ERROR: Could not create default config directory.");
            }
            config::Error::Io(io_err) => {
                println!("I/O error while reading the config file: {}", io_err);
            }
            config::Error::Yaml(yaml_err) => {
                println!("YAML error while reading the config file: {}", yaml_err);
            }
            config::Error::ParseError(parse_err) => {
                println!(
                    "Parse error while reading the config file: {:#?}",
                    parse_err
                );
            }
        }

        std::process::exit(1);
    }

    let config_data = config_parse_result.unwrap();
    let mut flag_name = config_data.0;
    let flags = config_data.1;

    if matches.is_present("list") {
        println!("List of flags:");
        list_flags(&flags);
        return Ok(());
    }

    let compact = matches.is_present("compact");

    let flag_width: Width = match matches.value_of("width") {
        Some(value) => match value.parse() {
            Ok(number) => Width::Custom(number),
            Err(..) => {
                println!("Error: you must specify the width argument a numeric value.");
                std::process::exit(1);
            }
        },
        None => Width::Full,
    };

    if matches.is_present("flag") {
        flag_name = matches.value_of("flag").unwrap().to_string();
    }

    if let Some(index) = flags.iter().position(|flag| flag.name == flag_name) {
        flags[index].display(flag_width, compact);
    } else {
        println!("Error: Unknown flag name `{}`", flag_name);
        println!("List of available flags:");
        list_flags(&flags);
        std::process::exit(1);
    }

    Ok(())
}

fn list_flags(flags: &Vec<Flag>) {
    for flag in flags {
        let mini_flag = flag.show_mini();
        println!("  {: <9} {}", flag.name, mini_flag);
    }
}