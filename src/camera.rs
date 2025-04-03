use image::{ImageBuffer, Rgb};
use rand::{thread_rng, Rng};

use crate::hittable::{Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{color::Color, Vec3};

pub struct Camera {
    center: Vec3,
    //Viewport data 
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    //Sampling data
    samples_per_pixel: u32,   //default to 10 but allowed to change
    pixel_samples_scale: f64, // = 1/samples_per_pixel 
    max_depth: u32,
}

impl Camera { 
    //Constructors 
    pub fn new(image_width: f64, image_height: f64) -> Self {
        //Configure the camera
        let center = Vec3::new_zeroes();

        //Configure the viewport, using the actual aspect ratio for the image
        let focal_length = 1.0; 
        let viewport_height = 2.0;
        let viewport_width 
          = viewport_height * (image_width / image_height);
      
        let viewport_u  //Vector along viewport top edge going r-to-l
          = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v  //Vector along viewport left edge going t-to-b 
          = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u  //Horizontal vector between two pixels
          = viewport_u / image_width;
        let pixel_delta_v  //Vertical vector between two pixels
          = viewport_v / image_height;

        //Calculate location of upper left pixel, pixel00, relative to the camera center
        let viewport_upper_left 
        = center 
            - Vec3::new(0.0, 0.0, focal_length) 
            - viewport_u / 2.0 
            - viewport_v / 2.0;
        let pixel00_loc   //Pixels are inset by half the pixel-to-pixel distance 
        = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v); 

        Camera { 
            center: center, 
            pixel00_loc: pixel00_loc, 
            pixel_delta_u: pixel_delta_u, 
            pixel_delta_v: pixel_delta_v, 
            samples_per_pixel: 10,
            pixel_samples_scale: 0.1,
            max_depth: 10,
        }
    }

    //Methods
    pub fn render(self: &mut Self, img_buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, world: &HittableList) {
        //  Update the Pixels in the ImageBuffer with the RGB values we want    
        for (u, v, pixel) in img_buf.enumerate_pixels_mut() {
            // eprint!("\rScanlines remaining: {}   ", height - y);

            //Calculate the pixel colour by random sampling in a square 
            //  around the pixel's viewport location and averaging the samples
            let mut pixel_color = Color::new_zeroes();
            for _sample in 0..=self.samples_per_pixel {
                let r = self.get_ray(u as f64, v as f64);
                //All colour calculations are done using f64 values in [0.0 .. 1.0]
                pixel_color  = pixel_color + Camera::ray_color(&r, self.max_depth, world); 
            }
            pixel_color = self.pixel_samples_scale * pixel_color;

            //Now we store the colour in the image buffer
            *pixel = Rgb(pixel_color.output_color());
        }
    }
     
    ///Construct a ray passing through a randomly chosen point in the 
    /// unit square around the given (u,v) location on the camera's viewport.
    fn get_ray(self: &Self, u: f64, v: f64) ->Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
          + ((u + offset.x) * self.pixel_delta_u)
          + ((v + offset.y) * self.pixel_delta_v);
        let ray_direction = pixel_sample - self.center; 
        Ray::new(self.center, ray_direction) 
    }

    pub fn set_samples_per_pixel(self: &mut Self, rate: u32) {
        self.samples_per_pixel = rate;
        self.pixel_samples_scale = 1f64 / rate as f64;
    }

    pub fn set_max_depth(self: &mut Self, depth: u32) {
        self.max_depth = depth;
    }

    //Associated functions    
    fn ray_color(r: &Ray, depth:u32, world: &HittableList) -> Color {
        if depth <=0 {
            return Color::new_zeroes();
        }
        let hit_test = world.hit(r, Interval::new(0.0, f64::INFINITY));
      
        match hit_test {
            Some(hit_record) => {                
                //Part of a hittable, so compute colour for a mid-grey diffuse material
                let direction = Vec3::random_on_hemisphere(&hit_record.normal);
                0.5 * Camera::ray_color(&Ray::new(hit_record.p, direction), depth - 1, world)
            }
            None => {
                //Part of the background, so compute blue gradient
                let unit_direction = Vec3::unit_vector(r.direction());
                let a = 0.5 * (unit_direction.y + 1.0); 
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)   
            }
        }
    } 

    ///Returns a Vec3 through a random point in the unit square around (0,0).
    /// 
    /// Excludes points on the right and bottom edges of the square.
    fn sample_square() -> Vec3 {
        let mut rng = thread_rng();
        Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0f64)
    }
    
}