// Handle configuration logic (creating an appropriately sized ImageBuffer)
// Call the library code to carry out the program logic (generating a ray traced image).
// Handle errors.
fn main() {
  // Configure the ImageBuffer size using the desired aspect ratio and width
  let aspect_ratio = 16.0 / 9.0; 
  let image_width = 400;
  //Calculate image_height ensuring it is >=1
  let height = image_width as f64 / aspect_ratio;
  let image_height = if height < 1.0 { 1 } else { height as u32}; 

  //Create the ImageBuffer; 
  //  Note: could use the type alias RgbImage for ImageBuffer<image::Rgb<u8>, Vec<u8>>, 
  //  see https://docs.rs/image/latest/image/type.RgbImage.html
  let mut image_buffer  = image::ImageBuffer::new(image_width, image_height);

  // eprint!("Starting render\n");
  rtiow::render(&mut image_buffer); 
  
  // Write the ImageBuffer to a file
  //  We can ignore errors for now so just "unwrap" the Ok result.
  //  If there is an Err result then thread 'main' will panic.   
  image_buffer.save("image.png").unwrap();

  // eprint!("\rDone                    ");

}
