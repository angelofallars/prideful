use crate::flag;
use std::fs;
use std::io;
extern crate yaml_rust;

mod default {
    pub const DEFAULT_CONFIG: &str = r##"
        "##;
}

#[derive(Debug)]
pub enum Error {
    FileNotFound,
    Io(io::Error),
    Yaml(yaml_rust::ScanError),
    ParseError(ParseError)
}

#[derive(Debug)]
pub enum ParseError {
    FieldNotFound(String),
    InvalidCollectionType,
    InvalidColor { flag_name: String, color: String }
}

impl From<std::io::Error> for Error {
    fn from(val: std::io::Error) -> Self {
        if val.kind() == std::io::ErrorKind::NotFound {
            Error::FileNotFound
        } else {
            Error::Io(val)
        }
    }
}

impl From<yaml_rust::ScanError> for Error {
    fn from(val: yaml_rust::ScanError) -> Self {
        Error::Yaml(val)
    }
}

pub fn load_config() -> Result<Vec<flag::Flag>, Error> {
    let xdg_dir = xdg::BaseDirectories::with_prefix("prideful").unwrap();

    let flags_json_path = match xdg_dir.find_config_file("flags.yml") {
        Some(path) => path,
        None => {
            // If no file found, place the default config
            let path = xdg_dir.place_config_file("flags.yml")?;

            fs::write(&path, default::DEFAULT_CONFIG)?;

            path
        }
    };

    let flags_json_str: String = String::from_utf8_lossy(&fs::read(flags_json_path)?).to_string();

    parse_config(flags_json_str)
}

pub fn load_config_from_path(path: &str) -> Result<Vec<flag::Flag>, Error> {
    let flags_json_str: String = String::from_utf8_lossy(&fs::read(path)?).to_string();

    parse_config(flags_json_str)
}

fn parse_config(contents: String) -> Result<Vec<flag::Flag>, Error> {
    // Parse the YAML file
    let yaml_file = &yaml_rust::YamlLoader::load_from_str(&contents)?[0];

    let yaml_flags = &yaml_file["flags"];

    if yaml_flags.is_badvalue() {
        return Err(Error::ParseError(ParseError::FieldNotFound("flags".to_string())));
    }

    let yaml_hash = match yaml_flags.as_hash() {
        Some(hash) => hash,
        None => return Err(Error::ParseError(ParseError::InvalidCollectionType))
    };

    let mut flags: Vec<flag::Flag> = Vec::new();

    // Iterate through the flags list
    for flag in yaml_hash {
        let name = flag.0.as_str().unwrap().to_string();
        let raw_stripes = flag.1.as_vec().unwrap();
        let mut stripes: Vec<flag::Stripe> = Vec::new();

        for raw_stripe in raw_stripes {
            let raw_stripe = raw_stripe.as_vec().unwrap();

            let color = raw_stripe[0].as_str().unwrap().to_string();
            let height: u8 = raw_stripe[1].as_i64().unwrap().try_into().unwrap();

            let stripe_parse = flag::Stripe::from(&color, height);

            if let Err(..) = stripe_parse {
                return Err(Error::ParseError(ParseError::InvalidColor {
                    flag_name: name,
                    color
                }));
            }

            let stripe = stripe_parse.unwrap();

            stripes.push(stripe);
        }

        let flag = flag::Flag::new(name, stripes);
        flags.push(flag);
    }

    Ok(flags)
}
