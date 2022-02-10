use std::fs;
use std::io;
use std::collections::HashMap;
use tui::Terminal;
use tui::backend::TermionBackend;
use colored::*;
use clap::{Arg, App};
extern crate clap;

#[derive(Debug)]
struct Flag {
    stripes: Vec<Stripe>,
}

impl Flag {
    fn new (stripes: Vec<Stripe>) -> Flag {
        Flag {
            stripes,
        }
    }

    fn height(&self) -> u32 {
        let mut height = 0;

        for stripe in &self.stripes {
            height += stripe.height;
        }

        height.into()
    }

    fn display(&self, width: Width, compact: bool) {
        let terminal_width = get_terminal_width();
        let terminal_height = get_terminal_height();
        let flag_height = self.height();

        // Calculate flag width
        let flag_width: usize;
        if !compact {
            flag_width = match width {
                Width::Full => get_terminal_width(),
                Width::Custom(custom_width) =>  {
                    let custom_width: usize = custom_width.try_into().unwrap();

                    // Make sure the set width doesn't exceed
                    // the terminal width
                    if terminal_width > custom_width {
                        custom_width
                    } else {
                        terminal_width
                    }
                }
            };
        } else {
            let compact_width = flag_height as usize * 4;

            if terminal_width > compact_width {
                flag_width = compact_width;
            } else {
                flag_width = terminal_width;
            }
        }

        let mut flag = String::new();

        // Calculate flag height
        let multiplier: f64;

        if !compact
        && terminal_height > flag_height.try_into().unwrap() {
            multiplier = (terminal_height as f64 / flag_height as f64)
                         .floor();
        } else {
            multiplier = 1.0;
        }

        // Format the flag
        for stripe in &self.stripes {
            let stripe_height = (stripe.height as f64 * multiplier)
                                .floor() as i32;

            for _i in 0..stripe_height {
                let stripe = format!("{}", " ".repeat(flag_width)
                                              .on_truecolor(stripe.color[0],
                                                            stripe.color[1],
                                                            stripe.color[2]));
                flag.push_str(&stripe);

                // Don't print a newline for full flags
                // so that it blends better when terminal is resized
                if width != Width::Full || compact {
                    flag.push('\n');
                }
            }
        }

        // Trim newline at the end for a cleaner output
        flag = flag.trim().to_string();

        println!("{}", flag);
    }

    // Show a mini horizontal flag for the help message
    fn show_mini(&self) -> String {
        let mut flag: String = String::new();

        for stripe in &self.stripes {
                let stripe = format!("{}", "▆".truecolor(stripe.color[0],
                                                         stripe.color[1],
                                                         stripe.color[2]));
                flag.push_str(&stripe);
        }

        flag
    }
}

// Individual stripe in a flag
#[derive(Debug)]
struct Stripe {
    color: [u8; 3],
    height: u8
}

impl Stripe {
    fn new(color: &str, height: u8) -> Stripe {
        let red = u8::from_str_radix(&color[1..3], 16).unwrap();
        let green = u8::from_str_radix(&color[3..5], 16).unwrap();
        let blue = u8::from_str_radix(&color[5..7], 16).unwrap();

        Stripe {
            color: [red, green, blue],
            height
        }
    }
}

fn get_terminal_width() -> usize {
    // Set up terminal stuff to find terminal width
    let stdout = io::stdout();
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();
    let terminal_size = terminal.size().unwrap();

    terminal_size.width.into()
}

fn get_terminal_height() -> usize {
    // Set up terminal stuff to find terminal height
    let stdout = io::stdout();
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();
    let terminal_size = terminal.size().unwrap();

    terminal_size.height.into()
}

#[derive(PartialEq)]
enum Width {
    // Entire terminal
    Full,
    // Arbitrary width
    Custom(u32)
}

fn print_usage(flags: &HashMap<String, Flag>) {
    let prideful_title = format!("{}{}{}{}{}{}{}{}",
                                 "p".bold().red(),
                                 "r".bold().yellow(),
                                 "i".bold().bright_green(),
                                 "d".bold().green(),
                                 "e".bold().cyan(),
                                 "f".bold().blue(),
                                 "u".bold().magenta(),
                                 "l".bold().bright_magenta());
    println!("{} {} {} {}",
             "Usage:".bold(),
             prideful_title,
             "[flag]".green(),
             "[args]".blue());

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

    // Detect the flags.json file in ~/.config/prideful
    // If no file or no directory, make them
    // Open JSON file
    let xdg_dir = xdg::BaseDirectories::with_prefix("prideful").unwrap();
    let flags_json_path = xdg_dir.find_config_file("flags.json")
                                 .expect("flags.json file not found");

    let flags_json_str: String = String::from_utf8_lossy(
                                 &fs::read(flags_json_path)
                                 .expect("Could not read flags.json"))
                                 .to_string();

    let flags_json = json::parse(&flags_json_str)
                          .expect("Error in parsing flags.json file");

    // Parse JSON config and store the flags in a hashmap
    let mut flags: HashMap<String, Flag> = HashMap::new();

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

        let flag = Flag::new(stripes);

        flags.insert(name.to_string(), flag);
    }

    // Parse CLI arguments

    let app = App::new("prideful")
                        .version("0.1")
                        .author("Angelo Fallaria <ba.fallaria@gmail.com>")
                        .about("A configurable TUI Pride flag generator.")
                        .arg(Arg::with_name("width")
                             .short("w")
                             .long("width")
                             .takes_value(true)
                             .help("Width of the flag in terms of terminal blocks."))
                        .arg(Arg::with_name("compact")
                             .short("c")
                             .long("compact")
                             .help("Print a smaller version of the flag."))
                        .arg(Arg::with_name("flag")
                             .takes_value(true)
                             .required(true));

    let matches = app.get_matches();

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

    let flag = flags.get(&flag_name).unwrap();
    flag.display(flag_width, compact);

    Ok(())
}
