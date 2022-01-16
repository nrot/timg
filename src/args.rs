extern crate clap;
use clap::{App, arg};

pub fn new_app()-> App<'static>{
    App::new("").args(&[
        arg!(<FILE> "file to show"),
        arg!(--filter [filter] "filter type: Nearest,Triangle,CatmullRom,Gaussian,Lanczos3"),
        arg!(-s --"scale_font" [scale_font] "scale of font to correct image"),
        arg!(-r --resize [resize] "resize type: resize(default), exact, to_fill(croped)")
    ])
}