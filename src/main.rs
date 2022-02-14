use clap::{App, Arg};
use std::io;
extern crate clap;
mod config;
mod flag;
use crate::flag::{Flag, Width};

fn main() -> Result<(), io::Error> {
    let app = App::new("prideful")
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
                .required_unless("list"),
        );

    let matches = app.get_matches();

    let flags = match matches.value_of("config") {
        Some(path) => config::load_config_from_path(path).unwrap(),
        None => config::load_config().unwrap(),
    };

    if matches.is_present("list") {
        println!("List of flags:");
        print_flags(&flags);
        return Ok(());
    }

    let compact = matches.is_present("compact");

    let flag_width: Width = match matches.value_of("width") {
        Some(value) => match value.parse() {
            Ok(number) => Width::Custom(number),
            Err(..) => {
                println!("Error: you must specify the width argument a numeric value.");
                return Ok(());
            }
        },
        None => Width::Full,
    };

    let flag_name: String = matches.value_of("flag").unwrap().to_string();

    if let Some(index) = flags.iter().position(|flag| flag.name == flag_name) {
        flags[index].display(flag_width, compact);
    } else {
        println!("Error: Unknown flag name `{}`", flag_name);
        println!("List of available flags:");
        print_flags(&flags);
        return Ok(());
    }

    Ok(())
}

fn print_flags(flags: &Vec<Flag>) {
    for flag in flags {
        let mini_flag = flag.show_mini();
        println!("  {: <9} {}", flag.name, mini_flag);
    }
}
