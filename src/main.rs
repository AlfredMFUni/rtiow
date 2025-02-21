fn main() {
    //Create an image
    let image_height = 256;
    let image_width = 256;

    let mut image_buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(image_width, image_height);

    rtiow::render(&mut image_buffer);

    image_buffer.save("image.png").unwrap();
    // Render
}
