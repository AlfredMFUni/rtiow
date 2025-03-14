pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn length_squared(self: &Self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3{
            x: u.y*v.z - u.z*v.y,
            y: u.z*v.x - u.x*v.z,
            z: u.x*v.y - u.y*v.x,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn cross_product_correct() -> Result<(), String> {
        // Arrange
        let v1: Vec3 = Vec3::new(0.1, 0.2, 0.3);
        let v2: Vec3 = Vec3::new(0.2, 0.3, 0.4);
        let expected: Vec3 = Vec3::new(-0.01, 0.02, -0.01);
        
        
        // Act
        let result: Vec3 = Vec3::cross(&v1, &v2);

        // Assert
        if expected.x - 0.00001 <= result.x && result.x < expected.x + 0.00001 {
            if (expected.y - 0.00001 .. expected.y +0.00001).contains(&result.y) {
                if (expected.z - 0.00001 .. expected.z +0.00001).contains(&result.z) {
                    Ok(())
                }
                else {
                    Err(String::from("Cross product, Z outside Range"))
                }
            }
            else {
                Err(String::from("Cross product, Y outside Range"))
            }
        }
        else {
            Err(String::from("Cross product, X outside Range"))
        }
        
    }

    #[test]
    fn length_squared_returns_correct_value() {
        // Arrange
        let (first, second, third) = (0.1, 0.2, 0.3);
        let v: Vec3 = Vec3::new(first, second, third);

        // Act
        let ls: f64 = v.length_squared();
        
        // Assert
        assert_eq!(ls, 0.14);
    }

    #[test]
    fn new_vec3_created_from_3_floats() {
        //Data
        let (first, second, third) = (0.1, 0.2, 0.3);

        //Test
        let v: Vec3 = Vec3::new(first, second, third);

        //Evaluate
        assert_eq!(v.x, first);
        assert_eq!(v.y, second);
        assert_eq!(v.z, third);
    }
}