use std::fs;
use crate::flag;

#[derive(Debug)]
pub enum Error {
    JsonError
}

pub fn load_config() -> Result<Vec<flag::Flag>, Error> {
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

    parse_config(flags_json_str)
}

fn parse_config(contents: String) -> Result<Vec<flag::Flag>, Error> {
    let flags_json = json::parse(&contents).expect("Error in parsing flags.json file");

    // Parse JSON config and store the flags in a vector
    let mut flags: Vec<flag::Flag> = Vec::new();

    for (name, data) in flags_json.entries() {
        let mut stripes: Vec<flag::Stripe> = Vec::new();

        for i in 0..data.len() {
            let color = data[i]["color"].to_string();
            let height: u8 = data[i]["height"]
                .as_u8()
                .expect("height in flags.json is invalid");

            let stripe = flag::Stripe::new(&color, height);
            stripes.push(stripe);
        }

        let flag = flag::Flag::new(name.to_string(), stripes);
        flags.push(flag);
    }

    Ok(flags)
}
