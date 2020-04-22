
#[derive(Debug)]
pub struct Vector {
    pub x:f64,
    pub y:f64,
    pub z:f64,
}

impl Vector {
    pub fn new(x: f64, y:f64, z:f64) -> Self {
        Vector {x, y, z}
    }

    pub fn norm2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(&self) -> f64 {
        self.norm2().sqrt()
    }

    pub fn normalized(&self) -> Vector {
        let n = self.norm();
        self.scale(1.0 / n)
    }

    /* fn negate(&self)-> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }
    */

    pub fn add(&self, v: &Vector) -> Vector {
        Vector::new(self.x + v.x, self.y + v.y, self.z + v.z) 
    }

    pub fn minus(&self, v: &Vector) -> Vector {
        Vector::new(self.x - v.x, self.y - v.y, self.z - v.z) 
    }

    pub fn scale(&self, l: f64) -> Vector {
        Vector::new(l * self.x, l * self.y, l * self.z) 
    }

    pub fn dot(&self, v: &Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}
