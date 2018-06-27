extern crate image;
extern crate clap;

use image::{GenericImage, DynamicImage, ImageBuffer, Pixel};
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("test")
                        .version("0.1")
                        .author("nju33 <nju33.ki@gmail.com>")
                        .about("test")
                        .arg(Arg::with_name("out-file")
                            .short("o")
                            .long("out-file")
                            .value_name("image.png")
                            .help("")
                            .takes_value(true)
                        )
                        .arg(Arg::with_name("base")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let base_file = matches.value_of("base").unwrap();
    let out_file = matches.value_of("out-file").unwrap_or("icon.png");
    let padding = 16;
    let geek_img = image::open(base_file).unwrap();
    let (geek_img_width, geek_img_height) = geek_img.dimensions();

    let scrapbox_img = image::open("scrapbox.png").unwrap();
    let resized_scrapbox_img = scrapbox_img.resize(
        ((geek_img_width as f32) * 0.4) as u32,
        ((geek_img_width as f32) * 0.4) as u32,
        image::FilterType::Nearest
    );
    let (resized_scrapbox_img_width, resized_scrapbox_img_height) = resized_scrapbox_img.dimensions();

    let mut img_buf: image::RgbaImage = ImageBuffer::new(geek_img_width, geek_img_height);
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        if geek_img_width - x < padding || geek_img_height - y < padding {
            pixel.data = geek_img.get_pixel(x, y).data;
        } else if geek_img_width - x + padding < resized_scrapbox_img_width && geek_img_height - y + padding < resized_scrapbox_img_height {
            let x = resized_scrapbox_img_width - (geek_img_width - x);
            let y = resized_scrapbox_img_height - (geek_img_height - y);
            let pixel_data = resized_scrapbox_img.get_pixel(x, y).data;
            pixel.data = pixel_data;
        } else {
            pixel.data = geek_img.get_pixel(x, y).data;
        }
    }

    img_buf.save(out_file);
}
