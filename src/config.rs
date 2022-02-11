use std::fs;
use std::path;
use std::io::Write;
use crate::flag;

mod default {
    pub const DEFAULT_CONFIG: &str = r##"
{

  "abro": [
    {
      "color": "#75ca92",
      "height": 2
    },
    {
      "color": "#b2e4c5",
      "height": 2
    },
    {
      "color": "#FFFFFF",
      "height": 2
    },
    {
      "color": "#e695b5",
      "height": 2
    },
    {
      "color": "#da446c",
      "height": 2
    }
  ],

  "pastel": [
    {
      "color": "#F59679",
      "height": 2
    },
    {
      "color": "#F9CE7B",
      "height": 2
    },
    {
      "color": "#FCF37C",
      "height": 2
    },
    {
      "color": "#9FFD7C",
      "height": 2
    },
    {
      "color": "#7BA1FB",
      "height": 2
    },
    {
      "color": "#AC7CFA",
      "height": 2
    }
  ],

  "agen": [
    {
      "color": "#000000",
      "height": 1
    },
    {
      "color": "#BABABA",
      "height": 1
    },
    {
      "color": "#FFFFFF",
      "height": 1
    },
    {
      "color": "#BAF484",
      "height": 1
    },
    {
      "color": "#FFFFFF",
      "height": 1
    },
    {
      "color": "#BABABA",
      "height": 1
    },
    {
      "color": "#000000",
      "height": 1
    }
  ],

  "aro": [
    {
      "color": "#3BA740",
      "height": 2
    },
    {
      "color": "#A8D47A",
      "height": 2
    },
    {
      "color": "#FFFFFF",
      "height": 2
    },
    {
      "color": "#ABABAB",
      "height": 2
    },
    {
      "color": "#000000",
      "height": 2
    }
  ],

  "aroace": [
    {
      "color": "#e28b00",
      "height": 2
    },
    {
      "color": "#eaca00",
      "height": 2
    },
    {
      "color": "#ffffff",
      "height": 2
    },
    {
      "color": "#63addc",
      "height": 2
    },
    {
      "color": "#213555",
      "height": 2
    }
  ],

  "ace": [
    {
      "color": "#000000",
      "height": 2
    },
    {
      "color": "#A4A4A4",
      "height": 2
    },
    {
      "color": "#FFFFFF",
      "height": 2
    },
    {
      "color": "#810081",
      "height": 2
    }
  ],

  "bi": [
    {
      "color": "#D60270",
      "height": 4
    },
    {
      "color": "#9B4F96",
      "height": 3
    },
    {
      "color": "#0038A8",
      "height": 4
    }
  ],

  "bigen": [
    {
      "color": "#C479A2",
      "height": 2
    },
    {
      "color": "#EDA5CD",
      "height": 2
    },
    {
      "color": "#D5C7E8",
      "height": 2
    },
    {
      "color": "#FFFFFF",
      "height": 2
    },
    {
      "color": "#D5C7E8",
      "height": 2
    },
    {
      "color": "#9AC7E8",
      "height": 2
    },
    {
      "color": "#6D82D1",
      "height": 2
    }
  ],

  "classic": [
    {
      "color": "#E50000",
      "height": 2
    },
    {
      "color": "#FF8D00",
      "height": 2
    },
    {
      "color": "#FFEE00",
      "height": 2
    },
    {
      "color": "#028121",
      "height": 2
    },
    {
      "color": "#004CFF",
      "height": 2
    },
    {
      "color": "#770088",
      "height": 2
    }
  ],

  "demiboy": [
    {
      "color": "#7f7f7f",
      "height": 2
    },
    {
      "color": "#c4c4c4",
      "height": 2
    },
    {
      "color": "#9fd9eb",
      "height": 2
    },
    {
      "color": "#FFFFFF",
      "height": 2
    },
    {
      "color": "#9fd9eb",
      "height": 2
    },
    {
      "color": "#c4c4c4",
      "height": 2
    },
    {
      "color": "#7f7f7f",
      "height": 2
    }
  ],

  "demigirl": [
    {
      "color": "#7f7f7f",
      "height": 2
    },
    {
      "color": "#c4c4c4",
      "height": 2
    },
    {
      "color": "#fcadc9",
      "height": 2
    },
    {
      "color": "#FFFFFF",
      "height": 2
    },
    {
      "color": "#fcadc9",
      "height": 2
    },
    {
      "color": "#c4c4c4",
      "height": 2
    },
    {
      "color": "#7f7f7f",
      "height": 2
    }
  ],

  "enby": [
    {
      "color": "#FCF431",
      "height": 2
    },
    {
      "color": "#FCFCFC",
      "height": 2
    },
    {
      "color": "#9D59D2",
      "height": 2
    },
    {
      "color": "#282828",
      "height": 2
    }
  ],

  "fluid": [
    {
      "color": "#FE76A2",
      "height": 2
    },
    {
      "color": "#FFFFFF",
      "height": 2
    },
    {
      "color": "#BF12D7",
      "height": 2
    },
    {
      "color": "#000000",
      "height": 2
    },
    {
      "color": "#303CBE",
      "height": 2
    }
  ],

  "gay": [
    {
      "color": "#078D70",
      "height": 2
    },
    {
      "color": "#98E8C1",
      "height": 2
    },
    {
      "color": "#FFFFFF",
      "height": 2
    },
    {
      "color": "#7BADE2",
      "height": 2
    },
    {
      "color": "#3D1A78",
      "height": 2
    }
  ],

  "les": [
    {
      "color": "#D62800",
      "height": 2
    },
    {
      "color": "#FF9B56",
      "height": 2
    },
    {
      "color": "#FFFFFF",
      "height": 2
    },
    {
      "color": "#D462A6",
      "height": 2
    },
    {
      "color": "#A40062",
      "height": 2
    }
  ],

  "neutrois": [
    {
      "color": "#FFFFFF",
      "height": 4
    },
    {
      "color": "#2f9d1d",
      "height": 4
    },
    {
      "color": "#000000",
      "height": 4
    }
  ],

  "pan": [
    {
      "color": "#FF1C8D",
      "height": 4
    },
    {
      "color": "#FFD700",
      "height": 4
    },
    {
      "color": "#1AB3FF",
      "height": 4
    }
  ],

  "polysex": [
    {
      "color": "#f200b8",
      "height": 4
    },
    {
      "color": "#2dd76b",
      "height": 4
    },
    {
      "color": "#398ef5",
      "height": 4
    }
  ],

  "queer": [
    {
      "color": "#B57FDD",
      "height": 4
    },
    {
      "color": "#FFFFFF",
      "height": 4
    },
    {
      "color": "#49821E",
      "height": 4
    }
  ],

  "trans": [
    {
      "color": "#5BCFFB",
      "height": 2
    },
    {
      "color": "#F5ABB9",
      "height": 2
    },
    {
      "color": "#FFFFFF",
      "height": 2
    },
    {
      "color": "#F5ABB9",
      "height": 2
    },
    {
      "color": "#5BCFFB",
      "height": 2
    }
  ],

  "trigen": [
    {
      "color": "#ff95c4",
      "height": 2
    },
    {
      "color": "#9581fe",
      "height": 2
    },
    {
      "color": "#67d965",
      "height": 2
    },
    {
      "color": "#9581fe",
      "height": 2
    },
    {
      "color": "#ff95c4",
      "height": 2
    }
  ]

}
        "##;
}

#[derive(Debug)]
pub enum Error {
    FileNotFound,
    JsonError
}

pub fn load_config() -> Result<Vec<flag::Flag>, Error> {
    // Detect the flags.json file in ~/.config/prideful
    // If no file or no directory, make them
    // Open JSON file
    let xdg_dir = xdg::BaseDirectories::with_prefix("prideful").unwrap();

    let flags_json_path = match xdg_dir.find_config_file("flags.json") {
        Some(path) => path,
        None => {
            let path = xdg_dir.place_config_file("flags.json")
                              .expect("Cannot create config directory");

            fs::write(&path, default::DEFAULT_CONFIG)
                .expect("Could not write default config");

            path
        }
    };


    let flags_json_str: String =
        String::from_utf8_lossy(&fs::read(flags_json_path).expect("Could not read flags.json"))
            .to_string();

    parse_config(flags_json_str)
}

pub fn load_config_from_path(path: &str) -> Result<Vec<flag::Flag>, Error> {
    let flags_json_str: String =
        String::from_utf8_lossy(&fs::read(path).expect("Could not read file"))
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
