#[macro_use]
extern crate timeit;

#[derive(Debug)]
enum QuadraticRoots {
    None,
    Double(f64),
    Couple(f64, f64)
}

struct Vector {
    x:f64,
    y:f64,
    z:f64,
}

impl Vector {
    fn new(x: f64, y:f64, z:f64) -> Self {
        Vector {
            x,
            y,
            z
        }
    }

    fn norm2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn norm(&self) -> f64 {
        self.norm2().sqrt()
    }

    fn negate(&self)-> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }

    fn add(&self, v: &Vector) -> Vector {
        Vector::new(self.x + v.x, self.y + v.y, self.z + v.z) 
    }

    fn minus(&self, v: &Vector) -> Vector {
        Vector::new(self.x - v.x, self.y - v.y, self.z - v.z) 
    }

    fn scale(&self, l: f64) -> Vector {
        Vector::new(l * self.x, l * self.y, l * self.z) 
    }

    fn dot(&self, v: &Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

struct Ray {
    origin: Vector,
    dir : Vector,
}


fn solve_quadratic (a: f64, b: f64, c: f64) -> QuadraticRoots { 
    let discr = b * b - 4.0 * a * c; 
    if discr < 0.0 {
        return QuadraticRoots::None;
    } 
    else if discr == 0.0 {
        return QuadraticRoots::Double(- 0.5 * b / a);
    }  
    else { 
        let q = if b > 0.0 { -0.5 * (b + discr.sqrt()) } else { -0.5 * (b - discr.sqrt()) };
        let x0 = q / a; 
        let x1 = c / q; 
        if x0 < x1 {
            return QuadraticRoots::Couple(x0, x1)
        } else {
            return QuadraticRoots::Couple(x1, x0)
        }
    } 
}

fn ray_inter_sphere(ray: &Ray, center: &Vector, radius: f64) -> QuadraticRoots {
    let a = ray.origin.norm2();
    let diff_o_c = ray.origin.minus(center);
    let b = 2.0 * ray.dir.dot(&diff_o_c);
    let c = - radius * radius + diff_o_c.norm2();
    solve_quadratic(a, b, c)
}

fn main() {
    println!("{}", "coucou");
    let center = Vector::new(0.0, 1.0, 1.0);
    let radius = 1.0;
    let ray = Ray { origin: Vector::new(0.0, 0.0, 1.0), dir: Vector::new(0.0, 1.0, 0.0) };

    let intersections = ray_inter_sphere(&ray, &center, radius);
    println!("{:?}", intersections);
}


