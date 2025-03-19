use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    orig: Vec3, 
    dir: Vec3,
} 

///A ray represented as the parametric vector equation: 
/// ray = origin + (t * direction) 
impl Ray {
    //Constructors
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    } 

    //Accessors: methods providing immutable references
    pub fn origin(self: &Self) -> &Vec3 {   //Full syntax for the self parameter
        &self.orig
    } 

    pub fn direction(&self) -> &Vec3 {      //Short syntax for the self parameter
        &self.dir
    }

    //Other methods
    /// Returns a vector locating the point on the ray at position t
    /// 
    /// # Examples
    /// ```
    /// use rtiow::{vec3::Vec3, ray::Ray};
    /// let ray = Ray::new(Vec3::new_zeroes(), Vec3::new(1.0, 2.0, 3.0));
    /// let point = ray.point_at(2.0);
    /// 
    /// assert_eq!(point, Vec3::new(2.0, 4.0, 6.0))
    /// ```
    ///  
    pub fn point_at(&self, t: f64) -> Vec3 {
        self.orig + (t * self.dir)
    }
}