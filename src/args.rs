use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use strum::Display;

#[derive(Debug, Display, Default, Copy, Clone, ValueEnum)]
#[strum(serialize_all = "kebab_case")]
pub enum Mode {
    #[default]
    White,
    Black,
    Bright,
    Dark,
}

#[derive(Debug, Display, Default, Copy, Clone, ValueEnum)]
#[strum(serialize_all = "kebab_case")]
pub enum Method {
    Luminance,
    Absolute,
    #[default]
    Hue,
    Hsl,
}

#[derive(Debug, Clone, Parser)]
#[clap(about, long_about, author, version)]
pub struct Args {
    #[clap(
        short,
        long,
        help = "The sorting mode to use. Useless if using pure mode",
        default_value_t = Mode::default()
    )]
    pub filtering: Mode,

    #[clap(short, long, help = "The sorting method to use", default_value_t = Method::default())]
    pub method: Method,

    #[clap(
        long,
        help = "Just sorts all the pixels, without any filtering. This will not return a result that even remotely resembles the original image",
        default_value_t = false
    )]
    pub pure: bool,

    #[clap(help = "The input image")]
    pub input: PathBuf,

    #[clap(short, long, help = "The output path")]
    pub out: Option<PathBuf>,
}
