use crate::flag;
use std::fs;
use std::io;
extern crate directories;
extern crate yaml_rust;
#[cfg(windows)]
use directories::BaseDirs;

mod default {
    pub const DEFAULT_CONFIG: &str = r##"flags:
  ## Flags are formatted with their name,
  ## followed by some stripes.
  ## Each stripe has a format of `- [ <color hex code>, <height> ]`.
  classic:
    - [ "#E50000", 2 ]
    - [ "#FF8D00", 2 ]
    - [ "#FFEE00", 2 ]
    - [ "#028121", 2 ]
    - [ "#004CFF", 2 ]
    - [ "#770088", 2 ]

  pastel:
    - [ "#F59679", 2 ]
    - [ "#F9CE7B", 2 ]
    - [ "#FCF37C", 2 ]
    - [ "#9FFD7C", 2 ]
    - [ "#7BA1FB", 2 ]
    - [ "#AC7CFA", 2 ]

  les:
    - [ "#D62800", 2 ]
    - [ "#FF9B56", 2 ]
    - [ "#FFFFFF", 2 ]
    - [ "#D462A6", 2 ]
    - [ "#A40062", 2 ]

  gay:
    - [ "#078D70", 2 ]
    - [ "#98E8C1", 2 ]
    - [ "#FFFFFF", 2 ]
    - [ "#7BADE2", 2 ]
    - [ "#3D1A78", 2 ]

  bi:
    - [ "#D60270", 4 ]
    - [ "#9B4F96", 3 ]
    - [ "#0038A8", 4 ]

  trans:
    - [ "#5BCFFB", 2 ]
    - [ "#F5ABB9", 2 ]
    - [ "#FFFFFF", 2 ]
    - [ "#F5ABB9", 2 ]
    - [ "#5BCFFB", 2 ]

  enby:
    - [ "#FCF431", 2 ]
    - [ "#FCFCFC", 2 ]
    - [ "#9D59D2", 2 ]
    - [ "#282828", 2 ]
    
  agen:
    - [ "#000000", 1 ]
    - [ "#BABABA", 1 ]
    - [ "#FFFFFF", 1 ]
    - [ "#BAF484", 1 ]
    - [ "#FFFFFF", 1 ]
    - [ "#BABABA", 1 ]
    - [ "#000000", 1 ]

  ace:
    - [ "#000000", 2 ]
    - [ "#A4A4A4", 2 ]
    - [ "#FFFFFF", 2 ]
    - [ "#810081", 2 ]

  aro:
    - [ "#3BA740", 2 ]
    - [ "#A8D47A", 2 ]
    - [ "#FFFFFF", 2 ]
    - [ "#ABABAB", 2 ]
    - [ "#000000", 2 ]

  pan:
    - [ "#FF1C8D", 4 ]
    - [ "#FFD700", 4 ]
    - [ "#1AB3FF", 4 ]

  unlabeled:
    - [ "#DCFDE0", 2 ]
    - [ "#FDFEF8", 2 ]
    - [ "#D9EFFA", 2 ]
    - [ "#FEE0BE", 2 ]

  queer:
    - [ "#B57FDD", 4 ]
    - [ "#FFFFFF", 4 ]
    - [ "#49821E", 4 ]

default_flag: classic"##;
}

#[derive(Debug)]
pub enum Error {
    FileNotFound,
    MakeDir,
    Io(io::Error),
    Yaml(yaml_rust::ScanError),
    ParseError(ParseError),
}

#[derive(Debug)]
pub enum ParseError {
    FieldNotFound(String),
    EmptyYamlFile,
    InvalidCollectionType,
    InvalidColor { flag_name: String, color: String },
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

pub fn load_config() -> Result<(String, Vec<flag::Flag>), Error> {
    let path = find_config_path();

    if path.is_none() {
        return Err(Error::MakeDir);
    }

    let path = path.unwrap();

    let flags_yaml_str: String = String::from_utf8_lossy(&fs::read(path)?).to_string();

    parse_config(flags_yaml_str)
}

#[cfg(not(windows))]
fn find_config_path() -> Option<std::path::PathBuf> {
    // Find XDG location first
    if let Ok(xdg_dir) = xdg::BaseDirectories::with_prefix("prideful") {
        // Find config file
        if let Some(xdg_config_file) = xdg_dir.find_config_file("prideful.yml") {
            return Some(xdg_config_file);
        } else {
            // Write the default config
            let path = xdg_dir.place_config_file("prideful.yml").unwrap();
            fs::write(&path, default::DEFAULT_CONFIG)
                .expect("Error: could not write default config to XDG directory.");
            return Some(path);
        }
    } else {
        // Then find $HOME directory
        if let Ok(home) = std::env::var("HOME") {
            let fallback_home_dir = std::path::PathBuf::from(&home).join(".config/prideful");
            let fallback_home_file = fallback_home_dir.join("prideful.yml");

            if fallback_home_file.exists() {
                return Some(fallback_home_file);
            } else {
                // Write default config
                fs::create_dir_all(fallback_home_dir)
                    .expect("Error: could not make default config directory.");
                fs::write(&fallback_home_file, default::DEFAULT_CONFIG).expect(
                    "Error: could not write default config to $HOME/.config/prideful/prideful.yml.",
                );
                return Some(fallback_home_file);
            }
        }
    }
    None
}

#[cfg(windows)]
fn find_config_path() -> Option<std::path::PathBuf> {
    if let Some(base_dirs) = BaseDirs::new() {
        let config_dir = base_dirs.config_dir().join("prideful/");
        let config_file_path = config_dir.join("prideful.yml");

        if config_file_path.exists() {
            return Some(config_file_path);
        } else {
            // Write default config
            fs::create_dir_all(config_dir)
                .expect("Error: could not make default config directory.");
            fs::write(&config_file_path, default::DEFAULT_CONFIG)
                .expect("Error: could not write default config.");
            return Some(config_file_path);
        }
    }

    None
}

pub fn load_config_from_path(path: &str) -> Result<(String, Vec<flag::Flag>), Error> {
    let flags_yaml_str: String = String::from_utf8_lossy(&fs::read(path)?).to_string();

    parse_config(flags_yaml_str)
}

fn parse_config(contents: String) -> Result<(String, Vec<flag::Flag>), Error> {
    // Parse the YAML file
    let yaml_file = &yaml_rust::YamlLoader::load_from_str(&contents)?;

    if yaml_file.len() < 1 {
        return Err(Error::ParseError(ParseError::EmptyYamlFile));
    }

    let yaml_file = &yaml_file[0];

    let yaml_flags = &yaml_file["flags"];
    let yaml_default_flag = &yaml_file["default_flag"];

    if yaml_flags.is_badvalue() {
        return Err(Error::ParseError(ParseError::FieldNotFound(
            "flags".to_string(),
        )));
    }

    if yaml_default_flag.is_badvalue() {
        return Err(Error::ParseError(ParseError::FieldNotFound(
            "default_flag".to_string(),
        )));
    }

    let yaml_flags_hash = match yaml_flags.as_hash() {
        Some(hash) => hash,
        None => return Err(Error::ParseError(ParseError::InvalidCollectionType)),
    };

    let mut flags: Vec<flag::Flag> = Vec::new();

    // Iterate through the flags list
    for flag in yaml_flags_hash {
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
                    color,
                }));
            }

            let stripe = stripe_parse.unwrap();

            stripes.push(stripe);
        }

        let flag = flag::Flag::new(name, stripes);
        flags.push(flag);
    }

    let yaml_default_flag = match yaml_default_flag.as_str() {
        Some(str) => str,
        None => return Err(Error::ParseError(ParseError::InvalidCollectionType)),
    };

    Ok((yaml_default_flag.to_string(), flags))
}
