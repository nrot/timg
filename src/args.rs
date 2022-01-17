extern crate clap;
use clap::{arg, App, Arg};
use regex::Regex;

pub fn new_app() -> App<'static> {
    App::new("").args(&[
        arg!(<FILE> "File to show"),
        arg!(--filter [filter] "Filter type: Nearest,Triangle,CatmullRom,Gaussian,Lanczos3. Default Triangle."),
        arg!(-s --"scale_font" [scale_font] "Scale of font to correct image. Default value = 0.5"),
        arg!(--scale [scale] "Custom scale"),
        Arg::new("resolution")
        .short('r')
        .long("resolution")
        .value_name("resolution")
        .help("Set exact resolution. Ignore scale_font. Format 80x54 - hxw")
        .validator_regex(Regex::new(r"\d+x\d+").unwrap().to_owned(), "Format 80x54 - hxw."),
        arg!(-g --grayscale "Grayscale image")
   ])
}
