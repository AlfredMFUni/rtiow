#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    //Constructors 
    pub fn new(min: f64, max: f64) -> Self {
        //Use the field init shorthand.
        // If we omit values for the fields then Rust will look for 
        // a function parameter with the same name
        Interval{
            min, 
            max,
        }
    }

    pub const fn new_empty() -> Self {
        Interval {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    //Methods 
    pub fn size(self: &Self) -> f64 {
        self.max - self.min 
    }

    pub fn contains(self: &Self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self: &Self, x: f64) -> bool {
        self.min < x && x < self.max
    } 

    pub fn clamp(self: &Self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    //Associated constants
    pub const EMPTY: Self = Self::new_empty(); 
    pub const UNIVERSE: Self = Self{
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
        };
}