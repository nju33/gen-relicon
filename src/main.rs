extern crate image;

use image::{GenericImage, ImageBuffer, Pixel};

fn main() {
    let mut geek_img = image::open("geek.png").unwrap();
    let (geek_img_width, geek_img_height) = geek_img.dimensions();

    let scrapbox_img = image::open("scrapbox.png").unwrap();
    let resized_scrapbox_img = scrapbox_img.resize(
        ((geek_img_width as f32) * 0.25) as u32,
        ((geek_img_width as f32) * 0.25) as u32,
        image::FilterType::Nearest
    );
    let (resized_scrapbox_img_width, resized_scrapbox_img_height) = geek_img.dimensions();

    // println!("{:?}", resized_scrapbox_img.dimensions());


    let mut img_buf: image::RgbaImage = ImageBuffer::new(geek_img_width, geek_img_height);
    let mut img_buf2: image::RgbaImage = ImageBuffer::new(geek_img_width, geek_img_height);


    // println!("{:?}", img.color());
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let relative_x = ((geek_img_width as f32) * 0.75) as u32;
        let relative_y = ((geek_img_height as f32) * 0.75) as u32;

        let mut real_x = x;
        let mut real_y = y;
        let mut target = &geek_img;

        if x + resized_scrapbox_img_width > geek_img_width {
            real_x = geek_img_width - x;
            target = &resized_scrapbox_img;
        }

        if y + resized_scrapbox_img_height > geek_img_height {
            real_y = geek_img_height - y;
            target = &resized_scrapbox_img;
        }

        println!("real_x={}, real_y={}", real_x, real_y);
        img_buf2.put_pixel(x, y, target.get_pixel(real_x, real_y));
    }

    img_buf2.save("icon.png");
}
