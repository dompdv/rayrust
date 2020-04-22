mod quadratics;
use quadratics::{quadratic_roots::QuadraticRoots};

mod vector;
use vector::{Vector};

#[derive(Debug)]
struct Ray {
    origin: Vector,
    dir : Vector,
}

impl Ray {
    fn new(origin: &Vector, dir: &Vector) -> Ray {
        let o = Vector::new(origin.x, origin.y, origin.z);
        let d = Vector::new(dir.x, dir.y, dir.z);
        Ray {
            origin: o, 
            dir: d
        }
    }

    fn at_t(&self, t: f64) -> Vector {
        self.origin.add(&self.dir.scale(t))
    }
}


#[derive(Debug)]
struct Camera {
    origin: Vector,
    dir: Vector,
    focale: f64,
    width: f64,
    screen_width: u16,
    screen_height: u16,
    center: Vector,
    hor: Vector,
    vert: Vector
}

impl Camera {
    fn new(origin: &Vector, dir: &Vector, focale: f64, width: f64, screen_width: u16, screen_height: u16) -> Camera {
        let normalized_dir = dir.normalized();
        let center = origin.add(&normalized_dir.scale(focale));
        let hor = if dir.y == 0.0 { Vector::new(0.0, -1.0, 0.0)} else { Vector::new(1.0, -dir.x / dir.y, 0.0) };
        let hor = hor.normalized();
        let screen_w = screen_width as f64;
        let hor = hor.scale(width / screen_w);
        let vert = if dir.y == 0.0 { Vector::new(0.0, 1.0, 0.0)} else { Vector::new(0.0, -dir.z / dir.y, 1.0) };
        let vert = vert.normalized();
        let vert = vert.scale(hor.norm());

        Camera {
            origin: Vector::new(origin.x, origin.y, origin.z),
            dir: normalized_dir,
            focale,
            width,
            screen_width,
            screen_height,
            center,
            hor,
            vert
        }
    }

    fn pixel(&self, x: u16, y: u16) -> Vector {
        let x = x as i32;
        let y = y as i32;
        let half_width = (self.screen_width / 2) as i32;
        let half_height = (self.screen_height / 2) as i32;
        let x = x - half_width;
        let y = -y + half_height;
        let h = self.hor.scale(x as f64);
        let v = self.vert.scale(y as f64);
        self.center.add(&h.add(&v))
    }
}

#[derive(Debug)]
struct Sphere {
    center: Vector,
    radius:f64,
}

impl Sphere {
    fn ray_intersections(&self, ray: &Ray, v: &mut Vec<f64>) -> () {
        let a = ray.dir.norm2();
        let diff_o_c = ray.origin.minus(&self.center);
        let b = 2.0 * ray.dir.dot(&diff_o_c);
        let c = - self.radius * self.radius + diff_o_c.norm2();
        let intes = QuadraticRoots::solve(a, b, c);
        match intes {
            QuadraticRoots::None => (),
            QuadraticRoots::Double(x) => v.push(x),
            QuadraticRoots::Couple(x1, x2) => { v.push(x1); v.push(x2); }
        };
    }
    
    fn normal_at_point(&self, point: &Vector) -> Vector {
        point.minus(&self.center).normalized()
    }
}
