use colored::*;
use std::io;
use tui::backend::TermionBackend;
use tui::Terminal;

#[derive(Debug)]
pub struct Flag {
    pub name: String,
    stripes: Vec<Stripe>,
}

impl Flag {
    pub fn new(name: String, stripes: Vec<Stripe>) -> Flag {
        Flag { name, stripes }
    }

    fn height(&self) -> u32 {
        let mut height = 0;

        for stripe in &self.stripes {
            height += stripe.height;
        }

        height.into()
    }

    pub fn display(&self, width: Width, compact: bool) {
        let terminal_width = get_terminal_width();
        let terminal_height = get_terminal_height();
        let flag_height = self.height();

        // Calculate flag width
        let flag_width: usize;
        if !compact {
            flag_width = match width {
                Width::Full => get_terminal_width(),
                Width::Custom(custom_width) => {
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

        if !compact && terminal_height > flag_height.try_into().unwrap() {
            multiplier = (terminal_height as f64 / flag_height as f64).floor();
        } else {
            multiplier = 1.0;
        }

        // Format the flag
        for stripe in &self.stripes {
            let stripe_height = (stripe.height as f64 * multiplier).floor() as i32;

            for _i in 0..stripe_height {
                let stripe = format!(
                    "{}",
                    " ".repeat(flag_width).on_truecolor(
                        stripe.color[0],
                        stripe.color[1],
                        stripe.color[2]
                    )
                );
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
    pub fn show_mini(&self) -> String {
        let mut flag: String = String::new();

        for stripe in &self.stripes {
            let stripe = format!(
                "{}",
                "▆".truecolor(stripe.color[0], stripe.color[1], stripe.color[2])
            );
            flag.push_str(&stripe);
        }

        flag
    }
}

// Individual stripe in a flag
#[derive(Debug)]
pub struct Stripe {
    color: [u8; 3],
    height: u8,
}

impl Stripe {
    pub fn new(color: &str, height: u8) -> Stripe {
        let red = u8::from_str_radix(&color[1..3], 16).unwrap();
        let green = u8::from_str_radix(&color[3..5], 16).unwrap();
        let blue = u8::from_str_radix(&color[5..7], 16).unwrap();

        Stripe {
            color: [red, green, blue],
            height,
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
pub enum Width {
    // Entire terminal
    Full,
    // Arbitrary width
    Custom(u32),
}
