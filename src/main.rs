extern crate image;

use image::{GenericImage, DynamicImage, ImageBuffer, Pixel};

// enum A {
//     DynamicImage,
//     ImageBuffer
// }

fn main() {
    let padding = 16;
    let mut geek_img = image::open("geek.png").unwrap();
    let (geek_img_width, geek_img_height) = geek_img.dimensions();
    let geek_img_buf = ImageBuffer::from_raw(geek_img_width, geek_img_height, geek_img.raw_pixels()).unwrap();

    let scrapbox_img = image::open("scrapbox.png").unwrap();
    let resized_scrapbox_img = scrapbox_img.resize(
        ((geek_img_width as f32) * 0.4) as u32,
        ((geek_img_width as f32) * 0.4) as u32,
        image::FilterType::Nearest
    );
    let (resized_scrapbox_img_width, resized_scrapbox_img_height) = resized_scrapbox_img.dimensions();

    let mut real_scrapbox_img: image::RgbaImage = ImageBuffer::new(resized_scrapbox_img_width + padding, resized_scrapbox_img_height);
    for (x, y, pixel) in real_scrapbox_img.enumerate_pixels_mut() {
        if resized_scrapbox_img_width >= x && resized_scrapbox_img_height >= y {
            pixel.data[0] = 0;
            pixel.data[1] = 0;
            pixel.data[2] = 0;
            pixel.data[3] = 0;
        }
    }
    let (real_scrapbox_img_width, real_scrapbox_img_height) = resized_scrapbox_img.dimensions();
    // println!("{}, {}", real_scrapbox_img_width, real_scrapbox_img_height);



    let mut img_buf: image::RgbaImage = ImageBuffer::new(geek_img_width, geek_img_height);
    let mut img_buf2: image::RgbaImage = ImageBuffer::new(geek_img_width, geek_img_height);


    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let relative_x = ((geek_img_width as f32) * 0.6) as u32;
        let relative_y = ((geek_img_height as f32) * 0.6) as u32;

        let mut real_x = x;
        let mut real_y = y;
        let mut target = &geek_img_buf;

        if x + real_scrapbox_img_width > geek_img_width && y + real_scrapbox_img_height > geek_img_height {
            let maybe_real_x = x + real_scrapbox_img_width - geek_img_width;
            let maybe_real_y = y + real_scrapbox_img_height - geek_img_height;

            let maybe_real_pixel = resized_scrapbox_img.get_pixel(maybe_real_x, maybe_real_y);
            let alpha_value = maybe_real_pixel.channels()[3];

            if (alpha_value != 0) {
                real_x = maybe_real_x;
                real_y = maybe_real_y;
                target = &real_scrapbox_img;
            }
        }

        img_buf2.put_pixel(x, y, *target.get_pixel(real_x, real_y));
    }

    img_buf2.save("icon.png");
}
