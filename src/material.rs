use std::fmt::Debug;

use crate::hittable::HitRecord; 
use crate::ray::Ray; 
use crate::vec3::{Vec3, color::Color};

pub trait Material: Debug {	//We require that ANY struct implementing our Material trait must also implement Debug
    fn scatter(self: &Self, r_in: &Ray, hit_record: &HitRecord) -> (Color, Ray);
}

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo } 
    }
}

impl Material for Lambertian {
    ///Lambertian materials scatter incoming rays randomly about the outward 
    /// facing normal of the incoming ray's hit point.
    
    // The incoming ray is not actually needed for this material, but is for others.
    //Prefix the parameter with an '_' to turn of warnings 
    fn scatter(self: &Self, _r_in: &Ray, hit_record: &HitRecord) -> (Color, Ray) { 
        let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        let scatter_direction
            = if scatter_direction.near_zero() { 
                hit_record.normal
            } else {
                scatter_direction
            };
        let scattered = Ray::new(hit_record.p, scatter_direction);
        (self.albedo, scattered)   
    }
}