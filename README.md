# 🌈 `prideful` (in development)

<img src="https://img.shields.io/github/stars/angelofallars/prideful?style=flat-square&label=stars&color=E54545"> <img src="https://img.shields.io/github/forks/angelofallars/prideful?style=flat-square&label=forks&color=FFAF4D"> <img src="https://img.shields.io/github/issues-pr/angelofallars/prideful?&style=flat-square&color=FFF23E"> <img src="https://img.shields.io/github/issues/angelofallars/prideful?style=flat-square&color=26813C"> <img src="https://img.shields.io/github/last-commit/angelofallars/prideful?style=flat-square&color=4B81FF"> <img src="https://img.shields.io/github/license/angelofallars/prideful?style=flat-square&color=7C2888">

A CLI-based pride flag generator written in Rust.

![flags](https://user-images.githubusercontent.com/39676098/149643374-b69507cb-4617-43c8-9666-1ea5277c821e.png)

## How to run

Build the project using `cargo`. Install `cargo` by following the Rustup install instructions [here](https://www.rust-lang.org/learn/get-started).

```cargo install --git https://github.com/angelofallars/prideful```

## Usage

To display a flag, run `prideful` with the flag of your choice. For example, `prideful classic`. To get the list of command-line options, run `prideful --help`. Run `prideful --list` to list available flags.

List of default flags:
- `classic`
- `pastel`
- `les`
- `gay`
- `bi`
- `trans`
- `enby`
- `agen`
- `ace`
- `aro`
- `pan`
- `queer`

## Config

The config file is located in `$XDG_CONFIG_HOME/prideful/prideful.yml`, usually `~/.config/prideful/prideful.yml`.
Flags are defined using YAML syntax, which means that adding new flags should be very easy.

Example snippets from the default config:
```yaml
  # Left is the color code in hexadecimal notation, right is the height of the stripe.
  classic:
    - [ "#E50000", 2 ]
    - [ "#FF8D00", 2 ]
    - [ "#FFEE00", 2 ]
    - [ "#028121", 2 ]
    - [ "#004CFF", 2 ]
    - [ "#770088", 2 ]

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
```

## Contributors

<a href="https://github.com/angelofallars/prideful/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=angelofallars/prideful" />
</a>

## Supporting this project

This project is free and open-source and will always be that way.

Development takes time and resources. If you like this project, consider donating as even a small amount goes a long way.

<a href="https://www.buymeacoffee.com/angelofallaria" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-orange.png" alt="Buy Me A Coffee" width="140"></a>

## License

This program is written under the GPLv3 license.
