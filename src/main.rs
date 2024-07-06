use image::io::Reader as ImageReader;

use std::env::args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    let img_path = &args[1];
    let img = ImageReader::open(img_path)?.decode()?;
    let img_grey = img.to_luma8();
    dbg!(img_grey);

    Ok(())
}
