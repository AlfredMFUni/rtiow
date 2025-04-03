use std::vec::Vec;
use std::rc::Rc;

use crate::interval::Interval;
use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool) -> Self {
        HitRecord{p, normal, t, front_face}
    }

    ///Set the hit record normal vector and direction it faces. 
    /// 
    ///The normal points in the opposite direction to the incident ray.
    /// 
    /// Pre-condition: outward_normal is unit length.
    pub fn set_face_normal(self: &mut Self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(self: &Self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
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
    fn hit(self: &Self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
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
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a; 
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut hit_record = HitRecord::new(r.point_at(root), Vec3::new_zeroes(), root, false);
        let outward_normal = (r.point_at(root) - self.center) / self.radius;
        hit_record.set_face_normal(r, outward_normal);

        return Some(hit_record);
    }
    
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    //Constructors
    pub fn new_empty() -> Self {
        HittableList{ objects: Vec::new(), }
    }

    pub fn new_with_element(elem: Rc<dyn Hittable>) -> Self {
        HittableList{ objects: vec![elem], }
    }

    //methods
    pub fn clear(self: &mut Self) {
        self.objects.clear();
    }

    pub fn add(self: &mut Self, elem: Rc<dyn Hittable>) {
        //This method moves the Rc<dyn Hittable> value elem by
        // taking ownership of it then transferring ownership
        // to the self.objects vector 
        self.objects.push(elem);
    } 
}

impl Hittable for HittableList {
    fn hit(self: &Self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        //Work through the list to see if any object is hit. 
        //Return the one closest to the ray's origin. 
        let mut closest_so_far = ray_t.max; 
        let mut closest_hit: Option<HitRecord> = None;

        //Must use &self.objects to borrow the Hittables as we don't want to move 
        // them out of self.objects
        for hittable in &self.objects {
            //If the ray hits the current Hittable object AND does so
            // closer to the ray origin, make this Hittable our closest_hit  
            let did_ray_hit = hittable.hit(r, Interval::new(ray_t.min, closest_so_far));
            if let Some(hit) = did_ray_hit {
                //We have a closer hit, so record this
                closest_so_far = hit.t;
                closest_hit = did_ray_hit;
            }
        }  

        //Finally, return a reference to the closest hittable object that the
        // ray hit, or None if the ray missed them all
        closest_hit
    }
}
