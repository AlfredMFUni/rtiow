use std::fmt::Debug;

use crate::{hittable::HitRecord, ray::Ray, vec3::color::Color};

pub trait Material: Debug {	//We require that ANY struct implementing our Material trait must also implement Debug
    fn scatter(self: &Self, r_in: &Ray, hit_record: &HitRecord, attenuation: &Color) -> Option<Ray> {
        None
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
  //Use the default implementation of the scatter() function for now
}