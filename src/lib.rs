pub mod vec3;

use image::ImageBuffer;

pub fn render(img_buf: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>){
    let width: u32 = img_buf.width();
    let height: u32 = img_buf.height();
    for (x,y,pixel) in img_buf.enumerate_pixels_mut()
    {
        let r: f64 = (x as f64) / (width as f64 - 1.0);
        let g: f64 = (y as f64) / (height as f64 - 1.0);
        let b: f64 = 0.0;

        *pixel = image::Rgb([(r * 255.999) as u8, (g * 255.999) as u8, (b * 255.999) as u8]);
    }
}