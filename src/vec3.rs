pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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