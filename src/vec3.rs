pub mod color;

use std::ops::{Add, AddAssign, Mul, Neg, Sub, Div};

#[derive(Copy, Clone, PartialEq, Debug )]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
} 

impl Vec3{
    //Constructors: just associated functions returning a new Vec3 value
    pub fn new(x : f64, y: f64, z: f64) -> Vec3 {
        Vec3{
            x: x,
            y: y,
            z: z, 
        }
    } 

    pub fn new_zeroes() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    //Methods 
    pub fn length_squared(self: &Self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    } 

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    //Other associated functions 
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 { 
        //Vec3::new_zeroes()
        Vec3 {
            x: u.y*v.z - u.z*v.y,
            y: u.z*v.x - u.x*v.z,
            z: u.x*v.y - u.y*v.x,
        }
    }

    pub fn unit_vector(u: &Vec3) -> Vec3 {
        let inv_length = 1.0 / u.length();
        Vec3::new(u.x * inv_length, u.y * inv_length, u.z * inv_length)
    }
}

//Note that vector addition is implemented as a method.  
impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z,
        }
    }
    
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}



#[cfg(test)]
mod test {
    //We need to bring all the code in the containing module, vec3, into the 
    //  scope of this inner module, test, so we can unit test the vec3 code. 
    //  Could write "use super::Vec3" to bring the Vec3 struct and all of its 
    //  associated functions into scope. Or do as I've done and just bring  
    //  everything declared in this file into scope. 
    use super::*; 

    #[test]
    fn new_vec3_created_from_3_floats() {
        //Arrange 
        let (first, second, third) = (0.1, 0.2, 0.3);

        //Act
        let v : Vec3 = Vec3::new(first, second, third);

        //Assert
        assert_eq!(v.x, first);
        assert_eq!(v.y, second);
        assert_eq!(v.z, third);
    }

    #[test]
    fn length_squared_returns_correct_value() {
        //Arrange 
        let (first, second, third) = (0.1, 0.2, 0.3);
        let v : Vec3 = Vec3::new(first, second, third);

        //Act
        let ls = v.length_squared();

        //Assert
        assert_eq!(ls, 0.14);
    }

   #[test]
   fn length_returns_correct_value() {
    //Arrange 
    //Get test values from a Pythagorean quadruple such as (2,6,9,11)   
    let (first, second, third) = (0.02, 0.06, 0.09);
    let v : Vec3 = Vec3::new(first, second, third);

    //Act
    let l = v.length();

    //Assert
    assert_eq!(l, 0.11);
   }

   #[test]
   fn dot_calculates_scalar_product () {
    //Arrange 
    let vector1 : Vec3 = Vec3::new(0.1, 0.2, 0.3); 
    let vector2 : Vec3 = Vec3::new(0.2, 0.3, 0.4); 
    let expected = 0.2; 

    //Act
    let result = Vec3::dot(&vector1, &vector2);

    //Assert
    assert_eq!(result, expected);

   }

   #[test]
   fn cross_calculates_cross_product() -> Result<(), String> {
    //Arrange 
    let vector1 : Vec3 = Vec3::new(0.1, 0.2, 0.3); 
    let vector2 : Vec3 = Vec3::new(0.2, 0.3, 0.4); 
    let expected = Vec3::new(-0.01, 0.02, -0.01); 

    //Act
    let result = Vec3::cross(&vector1, &vector2);

    //Assert
    //Two different approaches to checking whether a value lies within a
    //  given range are demonstrated here.
    if expected.x - 0.00001 < result.x && result.x < expected.x + 0.00001  {
        if (expected.y - 0.00001 .. expected.y + 0.00001).contains(&result.y) {
            if (expected.z - 0.00001 .. expected.z + 0.00001).contains(&result.z) {
                Ok(())
            } else {
                Err(String::from("z is outside the expected range"))
            }
        } else {
            Err(String::from("y is outside the expected range"))
        }
    } else {
        Err(String::from("x is outside the expected range"))
    }

   }

   #[test]
   fn unit_vector_has_unit_length_() {
    //Arrange
    let (first, second, third) = (0.02, 0.06, 0.09);
    let vector1 : Vec3 = Vec3::new(first, second, third); 
    let expected = 1.0;

    //Act 
    let vector2 = Vec3::unit_vector(&vector1);
    let result = vector2.length();

    //Assert
    assert_eq!(expected, result);
   }

   #[test]
   fn unit_vector_has_same_direction_as_input() {
    //Two vectors have the same direction when their dot product equals 
    //  the product of their lengths - ||u|| * ||v|| * cos(0).
    //  Here, the second vector is a unit vector, so the expected value
    //  of their dot product is the length of the first vector. 
    //Arrange
    let (first, second, third) = (2.0, 6.0, 9.0);
    let vector1 : Vec3 = Vec3::new(first, second, third); 
    let expected = vector1.length(); 

    //Act 
    let vector2 = Vec3::unit_vector(&vector1); 
    let result = Vec3::dot(&vector1, &vector2);

    //Assert
    assert_eq!(expected, result);    
   } 

   #[test]
   fn add_does_componentwise_vector_addition () -> Result<(), String> {
    //Arrange 
    let vector1 : Vec3 = Vec3::new(0.1, 0.2, 0.3); 
    let vector2 : Vec3 = Vec3::new(0.2, 0.3, 0.4); 
    let expected = Vec3::new(0.3, 0.5, 0.7); 

    //Act
    let result = vector1 + vector2;

    //Assert
    //We can get Rust to automatically implement the equality 
    //  comparison operator by putting #[derive(PartialEq)] 
    //  before the struct declaration. We also need #[derive(Debug)]
    //  to allow the assert! macros to print out debugging messages. 
    //  This makes equality testing a bit easier, unless you get rounding
    //  errors, as we do with this choice of test Vec3. 
    //assert_eq!(expected, result); //rounidng errors mean this fails
    if (expected.x - 0.00001 .. expected.x + 0.00001).contains(&result.x)  {
        if (expected.y - 0.00001 .. expected.y + 0.00001).contains(&result.y) {
            if (expected.z - 0.00001 .. expected.z + 0.00001).contains(&result.z) {
                Ok(())
            } else {
                Err(String::from("z is outside the expected range"))
            }
        } else {
            Err(String::from("y is outside the expected range"))
        }
    } else {
        Err(String::from("x is outside the expected range"))
    }
   } 

   //SOME TESTS OMITTED
   //   The above tests cover a range of unit testing techniques. 
   //   The remaining operators can be tested similarly.  
}