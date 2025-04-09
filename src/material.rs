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

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    pub refraction_index: f64, 
} 

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index, } 
    }

    //Calculate reflectance using Schlick's approximation
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0) 
    }
}

impl Material for Dielectric {
    //The sphere will always refraact, so will look odd
    fn scatter(self: &Self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        //Refractive index is different depending on whether the 
        // ray is entering or exiting the material 
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = Vec3::unit_vector(r_in.direction());
        //Check for total internal reflection
        let cos_theta = (-Vec3::dot(&unit_direction, &hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0; 

        //Randomly reflect rays 
        let randomly_reflected = Dielectric::reflectance(cos_theta, ri) > rand::random::<f64>();

        let direction = if cannot_refract || randomly_reflected {
            Vec3::reflect(&unit_direction, &hit_record.normal)
        } else {
            Vec3::refract(&unit_direction, &hit_record.normal, ri)
        };

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(hit_record.p, direction)))
    }
} 