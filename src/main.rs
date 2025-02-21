
use image::ImageBuffer;

fn render(img_buf: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>){
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

fn main() {
    //Create an image
    let image_height = 256;
    let image_width = 256;

    let mut image_buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(image_width, image_height);

    render(&mut image_buffer);

    image_buffer.save("image.png").unwrap();
    // Render
}
