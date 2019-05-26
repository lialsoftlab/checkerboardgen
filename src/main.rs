// use std::io::Write;
// use std::str::FromStr;
use image;
use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "Checker Board Generator", about = "A checkerboard pattern image generator.")]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,
    /// Set image width
    #[structopt(short, long, default_value = "800")]
    width: u32,
    /// Set image height
    #[structopt(short, long, default_value = "800")]
    height: u32,
    /// Set cell size
    #[structopt(short, long, default_value = "100")]
    cell_size: u32,
    /// Output file
    #[structopt(default_value="checkerboard.png", parse(from_os_str))]
    output: PathBuf,
}


fn main() {
    // Loading the CLI parameters config and parsing it.
    let opt = Opt::from_args();

    if opt.debug { dbg!(&opt); };


    let mut imgbuf = image::GrayImage::new(opt.width, opt.height);
    let cell_step = opt.cell_size * 2;
    let cell_size = opt.cell_size;

    if opt.debug { dbg!(cell_step); };

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut c = if x % cell_step < cell_size { 0u8 } else { 255u8 };
        c = if y % cell_step > cell_size { !c } else { c };
        *pixel = image::Luma([c]);
    }

    imgbuf.save(opt.output).unwrap();
}
