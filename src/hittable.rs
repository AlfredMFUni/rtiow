use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64) -> Self {
        HitRecord{p, normal, t}
    }
}

pub trait Hittable {
    fn hit(self: &Self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
} 

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    //  Constructors
    pub fn new (center: Vec3, radius: f64) -> Sphere {
        Sphere {center, radius} //Using the Field Init Shorthand 
    }
}

impl Hittable for Sphere {
    fn hit(self: &Self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h -  a * c;

        if discriminant < 0.0 { 
            return None; 
        } 
        
        let sqrtd = discriminant.sqrt();   

        //Find the nearest root within the acceptable range 
        let mut root = (h - sqrtd) / a; 
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a; 
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        return Some(HitRecord {
            p: (r.point_at(root)),
            normal: ((r.point_at(root) - self.center) / self.radius),
            t: (root)
            });
    }
    
}