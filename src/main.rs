use std::rc::Rc;

use rtiow::hittable::{Sphere, HittableList};
use rtiow::material::{Lambertian, Material, Metal, Dielectric};
use rtiow::vec3::color::Color;
use rtiow::vec3::Vec3;

// Handle configuration logic: 
//  create an appropriately sized ImageBuffer;
//  set up the world being imaged
//  set the number of ray saamples per pixel
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

  //Create the Materials 
  // We must specify the type to get dynamic allocation. Leaving this to type inference
  // would incorrectly infer a type of Rc<Lambertian>, Rc<Metal>, etc. 
  let material_ground: Rc<dyn Material>  = Rc::new(Lambertian{albedo: Color::new(0.8, 0.8, 0.0)});
  let material_center: Rc<dyn Material> = Rc::new(Lambertian{albedo: Color::new(0.1, 0.2, 0.5)});
    let material_left: Rc<dyn Material> = Rc::new(Dielectric{refraction_index: 1.5});         //Refactor
  let material_bubble: Rc<dyn Material> = Rc::new(Dielectric{refraction_index: 1.0 / 1.5}); //Add
  let material_right: Rc<dyn Material> = Rc::new(Metal{albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.1});

  //Create the World: we must place hittable objects into the scene  
  let mut world: HittableList = HittableList::new_empty();
  world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground.clone())));
  world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center.clone())));
  world.add(Rc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())));
  world.add(Rc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.4, material_bubble.clone())));
  world.add(Rc::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right.clone())));
  
  // eprint!("Starting render\n");
  rtiow::render(&mut image_buffer, &world, 50, 10); 
  
  // Write the ImageBuffer to a file
  //  We can ignore errors for now so just "unwrap" the Ok result.
  //  If there is an Err result then thread 'main' will panic.   
  image_buffer.save("image.png").unwrap();

  // eprint!("\rDone                    ");

}
