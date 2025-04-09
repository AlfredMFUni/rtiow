use std::fmt::Debug;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Vec3, color::Color};


pub trait Material: Debug {
    fn scatter(self: &Self, _r_in: &Ray, _hit_record: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
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
    /// 
    
    // The incoming ray is not actually needed for this material, but is for others. 
    fn scatter(self: &Self, _r_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> { 
        let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        let scatter_direction
            = if scatter_direction.near_zero() { 
                hit_record.normal
            } else {
                scatter_direction
            };
        let scattered = Ray::new(hit_record.p, scatter_direction);
        Some((self.albedo, scattered))   
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz_factor = if fuzz < 1.0 {fuzz} else {1.0};
        Metal { 
            albedo,
            fuzz: fuzz_factor,
         } 
    }
}

impl Material for Metal {
    ///Metal materials reflect the incoming rays about the hit point normal
    fn scatter(self: &Self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(r_in.direction(), &hit_record.normal);
        let reflected = Vec3::unit_vector(&reflected) + (self.fuzz * Vec3::random_unit_vector());
        let fuzzed_reflection = Ray::new(hit_record.p, reflected);
        if Vec3::dot(&fuzzed_reflection.direction(), &hit_record.normal) > 0.0 {
            Some((self.albedo, fuzzed_reflection))
        } else {
            //Fuzzed reflected ray points into the object
            None
        }        
    }
}