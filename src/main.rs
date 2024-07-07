use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};

use std::env::args;
use std::error::Error;
use std::u32;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    let img_path = &args[1];
    let img = ImageReader::open(img_path)?.decode()?;
    let ascii_img =image_to_ascii(img, 100);
    println!("{}", ascii_img);
    Ok(())
}

fn image_to_ascii(img: DynamicImage, width: u32) -> String {
    let ascii_chars = "@%#*+=-:.";
    let (img_width, img_heigth) = img.dimensions();
    let aspect_ratio = img_width as f32 / img_heigth as f32;
    let scaled_width = width;
    let scaled_heigth = (scaled_width as f32 / aspect_ratio).round() as u32;

    let img_resized = img.resize_exact(
        scaled_width,
        scaled_heigth,
        image::imageops::FilterType::Nearest,
    );

    let mut ascii_art = String::new();

    for y in 0..scaled_heigth {
        for x in 0..scaled_width {
            let pixel = img_resized.get_pixel(x, y);
            let gray_value =
                pixel[0] as f32 * 0.3 + pixel[1] as f32 * 0.59 + pixel[2] as f32 * 0.11;
            let index = ((gray_value / 255.0) * (ascii_chars.len() - 1) as f32).round() as usize;
            ascii_art.push_str(&ascii_chars.chars().nth(index).unwrap().to_string());
        }
        ascii_art.push('\n');
    }

    ascii_art
}
