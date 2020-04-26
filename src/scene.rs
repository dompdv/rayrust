

use crate::vector::vector::Vector;
use crate::noise::{marble};

pub mod quadratics {

    #[derive(Debug)]
    pub enum QuadraticRoots {
        None,
        Double(f64),
        Couple(f64, f64)
    }
    
    impl QuadraticRoots {
        pub fn solve (a: f64, b: f64, c: f64) -> QuadraticRoots { 
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
    }
    
}
    
use quadratics::QuadraticRoots;

#[derive(Debug)]
pub struct Ray {
    origin: Vector,
    dir : Vector
}

impl Ray {
    pub fn new(origin: &Vector, dir: &Vector) -> Ray {
        let o = Vector::new(origin.x, origin.y, origin.z);
        let d = Vector::new(dir.x, dir.y, dir.z);
        Ray {
            origin: o, 
            dir: d
        }
    }

    pub fn at_t(&self, t: f64) -> Vector {
        self.origin.add(&self.dir.scale(t))
    }

    pub fn dist_at_t(&self, t: f64) -> f64 {
        self.dir.scale(t).norm()
    }
}


#[derive(Debug)]
pub struct Camera {
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
    pub fn new(origin: &Vector, dir: &Vector, focale: f64, width: f64, screen_width: u16, screen_height: u16) -> Camera {
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

    pub fn pixel(&self, x: u16, y: u16) -> Vector {
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
pub enum Texture {
    Marble (f64, f64, f64, f64, f64), // texture depth, freqx, freqy, frez, turbulence
    Checker(f64, f64), // Color 1 Color2
    Uniform(f64) // Color
}
pub trait ApplyTexture {
    fn apply(&self, point: &Vector, noise:&Vec<f64>) -> f64 ;
}

impl ApplyTexture for Texture {
    fn apply(&self, point: &Vector, noise:&Vec<f64>) -> f64 {
        match self {
            Texture::Uniform(color) => { *color },
            Texture::Checker(min, amp) => { *min + *amp * (point.x.fract() + point.y.fract()).abs()},
            Texture::Marble(size, xp, yp, zp, turb) => { marble(noise, point.x, point.y, point.z, *size, *xp, *yp, *zp, *turb) },
        }
    }
}

pub trait WorldObject {
    fn ray_intersections(&self, ray: &Ray, v: &mut Vec<f64>) -> ();
    fn normal_at_point(&self, point: &Vector) -> Vector;
    fn who_am_i(&self) -> u32;
    fn color_at_point(&self, point: &Vector, noise:&Vec<f64>) -> f64;
}


#[derive(Debug)]
pub struct Sphere {
    center: Vector,
    radius:f64,
    texture:Texture
}

impl Sphere {
    pub fn new(center: &Vector, radius: f64, texture: Texture) -> Sphere {
        Sphere {
            center: Vector::new(center.x, center.y, center.z),
            radius,
            texture
        }
    }
}


impl WorldObject for Sphere {
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
//        let n = point.minus(&self.center).normalized();
//        n.add(&Vector::new((20.0 * point.x).cos(), (25.0 * point.y).sin(), 0.0)).normalized()
        point.minus(&self.center).normalized()
    }
    fn who_am_i(&self) -> u32 {
        1
    }
    fn color_at_point(&self, point: &Vector, noise:&Vec<f64>) -> f64 { 
        self.texture.apply(&point, &noise) 
    }
}

#[derive(Debug)]
pub struct Floor {
    z: f64,
    texture:Texture
}

impl Floor {
    pub fn new(z: f64, texture:Texture) -> Floor {
        Floor { z , texture}
    }
}

impl WorldObject for Floor {
    fn ray_intersections(&self, ray: &Ray, v: &mut Vec<f64>) -> () {
        if ray.dir.z != 0.0 {
            v.push((self.z - ray.origin.z) / ray.dir.z);
        }
    }
    fn normal_at_point(&self, _point: &Vector) -> Vector {
        Vector::new(0.0, 0.0, 1.0)
        //Vector::new((20.0 * point.x).cos(), (25.0 * point.y).sin(), 1.0).normalized()
    }
    fn color_at_point(&self, point: &Vector, noise:&Vec<f64>) -> f64 { 
        self.texture.apply(&point, &noise) 
    }
    fn who_am_i(&self) -> u32 {
        2
    }
}

