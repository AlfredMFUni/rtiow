pub mod vec3; //includes the sub-module color 
pub mod ray;
pub mod hittable;
pub mod interval;
pub mod camera;

use core::f64;

use image::{ImageBuffer, Rgb};

use hittable::HittableList;
use camera::Camera;


///Render an image onto the supplied ImageBuffer
pub fn render(img_buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, world: &HittableList, samples_per_pixel: u32) {
  //Camera: renders an image of the world onto the ImageBuffer
  let mut cam = Camera::new(img_buf.width() as f64, img_buf.height() as f64);
  cam.set_samples_per_pixel(samples_per_pixel);	
  cam.render(img_buf, world);

}