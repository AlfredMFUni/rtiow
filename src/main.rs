use std::rc::Rc;

use rtiow::hittable::{Sphere, HittableList};
use rtiow::vec3::Vec3;

// Handle configuration logic: 
//  create an appropriately sized ImageBuffer;
//  set up the world being imaged.
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

  //Create the World: we must place hittable objects into the scene  
  let mut world: HittableList = HittableList::new_empty();
  world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
  world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

  // eprint!("Starting render\n");

  rtiow::render(&mut image_buffer, &world); 
  
  // Write the ImageBuffer to a file
  //  We can ignore errors for now so just "unwrap" the Ok result.
  //  If there is an Err result then thread 'main' will panic.   
  image_buffer.save("image.png").unwrap();

  // eprint!("\rDone                    ");

}
