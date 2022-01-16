use std::io::Write;

use image::{imageops::FilterType, GenericImageView};
use image::io::Reader as ImageReader;
use termcolor::{BufferWriter, Color, ColorSpec, WriteColor};

mod args;
mod lib_console;
mod resizes;

fn main() {
    let arg = args::new_app().get_matches();
    println!("File path: {:?}", arg.value_of("FILE"));
    let fp = arg
        .value_of("FILE")
        .unwrap()
        .parse::<std::path::PathBuf>()
        .expect("Can`t convert to path");
    let img = ImageReader::open(fp)
        .expect("Can`t open file to read")
        .decode()
        .expect("Can`t decode image. Image type not supported");

    let ft = match arg.value_of("filter").unwrap_or("triangle").to_lowercase().trim() {
        "nearest" => FilterType::Nearest,
        "triangle" => FilterType::Triangle,
        "catmullrom" => FilterType::CatmullRom,
        "gaussian" => FilterType::Gaussian,
        "lanczos3" => FilterType::Lanczos3,
        e => panic!("Unkown filter type: {}", e),
    };

    let sf = arg
        .value_of("scale_font")
        .unwrap_or("0.75")
        .parse::<f32>()
        .expect("scale_font must be float");

    let rt = match arg
        .value_of("resize")
        .unwrap_or("resize")
        .to_lowercase()
        .trim()
    {
        "resize" => resizes::resize,
        "exact" => resizes::resize_exact,
        "to_fill" => resizes::resize_to_fill,
        e => panic!("Unkown resize type: {}", e),
    };

    let mut buffwr = BufferWriter::stdout(termcolor::ColorChoice::Always);
    let mut buff = buffwr.buffer();
    // let cs = buff.

    let csize = lib_console::get_terminal_size().expect("Run not in console?");
    if cfg!(debug) {
        println!("Terminal size h/w: {}/{}\n", &csize.rows, &csize.cols);
    };
    let rimg = rt(
        &img,
        csize.cols as u32,
        img.height() as u32, //(csize.rows as f32 * sf).floor()
        ft,
    )
    .into_rgb8();
    println!("New image size h/w: {}/{}", rimg.height(), rimg.width());
    for y in 0..(rimg.height() - 1) {
        for x in 0..(rimg.width() - 1) {
            let p = rimg.get_pixel(x, y).0;
            buff.set_color(
                ColorSpec::new()
                    .set_fg(Some(Color::White))
                    .set_bg(Some(Color::Rgb(p[0], p[1], p[2]))),
            )
            .expect("Can`t set color");
            write!(&mut buff, " ").expect("Can`t write to buffer");
        }
        buff.set_color(&ColorSpec::new()).expect("Can`t set color");
        writeln!(&mut buff, "").expect("Can`t write to buffer");
    }
    buffwr.print(&buff).expect("Can`t print buffer to terminal");
}
