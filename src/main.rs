use clap::{App, Arg};
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::io;
extern crate clap;
mod flag;
use crate::flag::{Flag, Width, Stripe};

fn print_usage(flags: &HashMap<String, Flag>) {
    let prideful_title = format!(
        "{}{}{}{}{}{}{}{}",
        "p".bold().red(),
        "r".bold().yellow(),
        "i".bold().bright_green(),
        "d".bold().green(),
        "e".bold().cyan(),
        "f".bold().blue(),
        "u".bold().magenta(),
        "l".bold().bright_magenta()
    );
    println!(
        "{} {} {} {}",
        "Usage:".bold(),
        prideful_title,
        "[flag]".green(),
        "[args]".blue()
    );

    println!();
    println!("{}", "Options:".bold());
    println!("  -h, --help           display this help message");
    println!("  -c, --compact        show a formatted flag with a nice aspect ratio");
    println!();
    println!("{}", "Dimensions:".bold());
    println!("  -w, --width NUMBER   set the flag width to the specified number");
    println!("  -s, --small          make the flag not take up the entire terminal height");

    println!();
    println!("{}", "Flags:".bold());

    // This long vector algorithm is necessary to sort the flags by name
    let mut flag_names: Vec<String> = Vec::new();
    for (flag_name, _flag) in flags {
        flag_names.push(flag_name.to_string());
    }

    flag_names.sort();

    for flag_name in flag_names {
        let mini_flag = flags.get(&flag_name).unwrap().show_mini();
        println!("  {: <9} {}", flag_name, mini_flag);
    }

    println!();
    println!("{}", "prideful BETA v0.1.0".bold());
    println!("Report bugs to https://github.com/angelofallars/prideful/issues")
}

fn main() -> Result<(), io::Error> {
    let app = App::new("prideful")
        .version("0.1")
        .author("Angelo Fallaria <ba.fallaria@gmail.com>")
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
                .short("c")
                .long("compact")
                .help("Print a smaller version of the flag."),
        )
        .arg(Arg::with_name("flag").takes_value(true).required(true));

    let matches = app.get_matches();

    // Detect the flags.json file in ~/.config/prideful
    // If no file or no directory, make them
    // Open JSON file
    let xdg_dir = xdg::BaseDirectories::with_prefix("prideful").unwrap();
    let flags_json_path = xdg_dir
        .find_config_file("flags.json")
        .expect("flags.json file not found");

    let flags_json_str: String =
        String::from_utf8_lossy(&fs::read(flags_json_path).expect("Could not read flags.json"))
            .to_string();

    let flags_json = json::parse(&flags_json_str).expect("Error in parsing flags.json file");

    // Parse JSON config and store the flags in a vector
    let mut flags: Vec<Flag> = Vec::new();

    for (name, data) in flags_json.entries() {
        let mut stripes: Vec<Stripe> = Vec::new();

        for i in 0..data.len() {
            let color = data[i]["color"].to_string();
            let height: u8 = data[i]["height"]
                .as_u8()
                .expect("height in flags.json is invalid");

            let stripe = Stripe::new(&color, height);
            stripes.push(stripe);
        }

        let flag = Flag::new(name.to_string(), stripes);
        flags.push(flag);
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
        return Ok(());
    }

    Ok(())
}
