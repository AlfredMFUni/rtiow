pub mod vec3; //includes the sub-module color 
pub mod ray;

use image::{ImageBuffer, Rgb};
use vec3::color::Color; 
use vec3::Vec3;
use ray::Ray;

fn ray_color(r: &Ray) -> Color {
  //Calculate a blue gradient based on where the ray hits the viewport 
  let unit_direction= Vec3::unit_vector(r.direction());
  let a = 0.5 * (unit_direction.y + 1.0); 
  (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)

}

///Render an image onto the supplied ImageBuffer  
pub fn render(img_buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
  //Recover the image dimensions
  let width = img_buf.width();
  let height = img_buf.height();

  //Configure the camera
  let focal_length = 1.0;
  let camera_center = Vec3::new_zeroes();

  //Configure the viewport, using the actual aspect ratio for the image
  let viewport_height = 2.0;
  let viewport_width 
    = viewport_height * (width as f64 / height as f64);

  let viewport_u  //Vector along viewport top edge going r-to-l
    = Vec3::new(viewport_width, 0.0, 0.0);
  let viewport_v  //Vector along viewport left edge going t-to-b 
    = Vec3::new(0.0, -viewport_height, 0.0);
  let pixel_delta_u  //Horizontal vector between two pixels
    = viewport_u / width as f64;
  let pixel_delta_v  //Vertical vector between two pixels
    = viewport_v / height as f64;

  //Calculate location of upper left pixel, pixel00, relative to the camera center
  let viewport_upper_left 
    = camera_center 
      - Vec3::new(0.0, 0.0, focal_length) 
      - viewport_u / 2.0 
      - viewport_v / 2.0;
  let pixel00_loc   //Pixels are inset by half the pixel-to-pixel distance 
    = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);  
  
  //  Update the Pixels in the ImageBuffer with the RGB values we want    
  for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
      // eprint!("\rScanlines remaining: {}   ", height - y);

      //Calculate the ray passing through the current pixel
      let pixel_center 
        = pixel00_loc + x as f64 * pixel_delta_u + y as f64 * pixel_delta_v;
      let ray_direction = pixel_center - camera_center;
      let r = Ray::new(camera_center, ray_direction);

      //All colour calculations are done using f64 values in [0.0 .. 1.0]
      let pixel_color  = ray_color(&r);

      //Now we store the colour in the image buffer
      *pixel = Rgb(pixel_color.output_color());
  }
}