use std::ops::{Add, Mul};

use crate::vec3::Vec3;
use crate::interval::Interval;

//Use the newtype pattern to create Color as a thin wrapper around Vec3.
//  We can control which of the public functions of Vec3 are accessible
//  when using it as a Color. 
//  We can also add new, Color only public functions. 
#[derive(Clone, Copy, Debug)]
pub struct Color(Vec3);

impl Color {
    pub fn new(x : f64, y: f64, z: f64) -> Color {
        Color (Vec3::new(x, y, z))
    } 

    pub fn new_zeroes() -> Color {
        Color(Vec3::new_zeroes())
    }

    //This time, implement the getter methods 
    pub fn r (&self) -> f64 {
        self.0.x
    }

    pub fn g (&self) -> f64 {
        self.0.y
    }
    
    pub fn b (&self) -> f64 {
        self.0.z
    }

    //
    pub fn output_color(&self) -> [u8; 3] {    
        //Move from [0 .. 1] colour values to [0 .. 255] colour values. 
        [(Self::INTENSITY.clamp(Self::linear_to_gamma(self.r())) * 256.0) as u8, 
            (Self::INTENSITY.clamp(Self::linear_to_gamma(self.g())) * 256.0) as u8, 
            (Self::INTENSITY.clamp(Self::linear_to_gamma(self.b())) * 256.0) as u8
        ]
    }

    //Associated constants
    pub const INTENSITY: Interval = Interval {min: 0.0, max: 0.999};

    //Associated functions
    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0f64
        }
    }

}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color ( self * rhs.0)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color (self.0 + rhs.0)
    }
}


#[cfg(test)]
mod test {
    use super::*; 

    #[test]
    fn new_color_created_from_3_floats() {
        //Arrange 
        let (first, second, third) = (0.1, 0.2, 0.3);

        //Act
        let c : Color = Color::new(first, second, third);

        //Assert
        assert_eq!(c.r(), first);
        assert_eq!(c.g(), second);
        assert_eq!(c.b(), third);
    }

    #[test]
    fn output_color_converts_from_f64_to_u8_color_channels() {
        //Arrange 
        let (first, second, third) = (0.1, 0.2, 0.3);
        let expected = [25u8, 51u8, 76u8];

        //Act
        let c : Color = Color::new(first, second, third);
        let result = c.output_color();

        //Assert
        assert_eq!(expected, result);
    }
}
