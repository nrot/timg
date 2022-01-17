use std::io::Write;

use image::io::Reader as ImageReader;
use image::DynamicImage;
use image::{imageops::FilterType, GenericImageView};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod args;
mod lib_console;

struct WorkerArgs<'a> {
    img: &'a DynamicImage,
    csize: lib_console::Size,
    ft: FilterType,
    sf: f32,
    exact: bool,
    grayscale: bool,
    scale: f32,
}

fn worker(args: WorkerArgs) {
    let mut buffwr = BufferWriter::stdout(termcolor::ColorChoice::Always);
    let mut buff = buffwr.buffer();
    let c = args.img.height() as f32 / (args.img.width() as f32);
    let pmg = args.img.resize_exact(
        (args.csize.cols as f32 * args.scale).floor() as u32,
        ((if args.exact {
            args.csize.rows as f32
        } else {
            (args.csize.cols as f32 * c * args.sf).floor()
        }) * args.scale)
            .floor() as u32,
        args.ft,
    );
    let rimg = if args.grayscale {
        pmg.grayscale().to_rgb8()
    } else {
        pmg.to_rgb8()
    };
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

    let ft = match arg
        .value_of("filter")
        .unwrap_or("triangle")
        .to_lowercase()
        .trim()
    {
        "nearest" => FilterType::Nearest,
        "triangle" => FilterType::Triangle,
        "catmullrom" => FilterType::CatmullRom,
        "gaussian" => FilterType::Gaussian,
        "lanczos3" => FilterType::Lanczos3,
        e => panic!("Unkown filter type: {}", e),
    };

    let mut sf = arg
        .value_of("scale_font")
        .unwrap_or("0.5")
        .parse::<f32>()
        .expect("scale_font must be float");

    let mut csize = lib_console::get_terminal_size().expect("Run not in console?");
    let mut scale = arg
        .value_of("scale")
        .unwrap_or("1.0")
        .parse::<f32>()
        .expect("Can`t parse scale");

    let res = String::from(
        arg.value_of("resolution")
            .unwrap_or("")
            .to_lowercase()
            .trim(),
    );
    let mut exact = false;
    if res != "" {
        let rw = res.split("x").collect::<Vec<&str>>();
        csize.rows = rw[0]
            .parse::<u16>()
            .expect("Can`t convert resolution to uint");
        csize.cols = rw[1]
            .parse::<u16>()
            .expect("Can`t convert resolution to uint");
        sf = 1.0;
        exact = true;
        scale = 1.0;
    };

    if scale > 1.0 {
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);
        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
        let _ = writeln!(&mut stdout, "Warning scale more then 1.0 can broke image");
    };

    let grayscale = arg.is_present("grayscale");

    worker(WorkerArgs {
        img: &img,
        csize: csize,
        ft: ft,
        sf: sf,
        exact: exact,
        grayscale: grayscale,
        scale: scale,
    });
}

#[test]
fn all_imgs() {
    let csize = lib_console::get_terminal_size().expect("Run not in console?");
    println!("Terminal size h/w: {}/{}\n", &csize.rows, &csize.cols);
    for file in std::fs::read_dir("imgs/").unwrap() {
        let fp = file.unwrap().path();
        println!("Draw image: {:?}", fp);
        let img = ImageReader::open(fp)
            .expect("Can`t open file")
            .decode()
            .expect("Can`t decode image");
        worker(WorkerArgs {
            img: &img,
            csize: csize.clone(),
            ft: FilterType::Triangle,
            sf: 0.5,
            exact: false,
            grayscale: false,
            scale: 1.0,
        });
    }
}
