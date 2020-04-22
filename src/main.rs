#[macro_use]
extern crate timeit;

use minifb::{Key, Window, WindowOptions};

#[derive(Debug)]
enum QuadraticRoots {
    None,
    Double(f64),
    Couple(f64, f64)
}

impl QuadraticRoots {
    fn _trim(&self, threshold: f64) -> QuadraticRoots {
        match self {
            QuadraticRoots::None => QuadraticRoots::None,
            QuadraticRoots::Double(x) => if x > &threshold { QuadraticRoots::Double(*x)} else { QuadraticRoots::None},
            QuadraticRoots::Couple(x1, x2) => 
                if x1 > &threshold && x2 > &threshold { 
                        QuadraticRoots::Couple(*x1, *x2) 
                    } else if x1 > &threshold && x2 <= &threshold {
                        QuadraticRoots::Double(*x1)
                    } else if x1 <= &threshold && x2 > &threshold {
                        QuadraticRoots::Double(*x2)
                    } else { 
                        QuadraticRoots::None 
                    }
        }
    }
}

#[derive(Debug)]
struct Vector {
    x:f64,
    y:f64,
    z:f64,
}

impl Vector {
    fn new(x: f64, y:f64, z:f64) -> Self {
        Vector {x, y, z}
    }

    fn norm2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn norm(&self) -> f64 {
        self.norm2().sqrt()
    }

    fn normalized(&self) -> Vector {
        let n = self.norm();
        self.scale(1.0 / n)
    }

    /* fn negate(&self)-> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }
    */

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
        let intes = solve_quadratic(a, b, c);
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


fn main() {
    let screen_width = 300;
    let screen_height = 300;
    let width: usize = screen_width as usize;
    let height: usize = screen_height as usize;

    let mut buffer: Vec<u32> = vec![0; width * height];

    let eye = Vector::new(0.0, 0.0, 1.0);
    let direction = Vector::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        &eye,
        &direction,
        0.2,
        0.3,
        screen_width,
        screen_height
    );
    let sphere = Sphere {
        center: Vector::new(0.0, 2.0, 1.0),
        radius: 1.0
    };
    let light = Vector::new(0.1, -0.1, 1.0).normalized();
    timeit_loops!( 1, {
    for x in 0..screen_width {
        for y in 0..screen_height {
            let mut intersections: Vec<f64> = Vec::with_capacity(5);
            let point = camera.pixel(x, y);
            let ray = Ray::new(&eye, &point.minus(&eye));
            sphere.ray_intersections(&ray, &mut intersections);
            // println!("{}:{} -> {:?} {:?}", x,y, ray, intersections);
            let mut pixel:u32 = 0;
            intersections.retain(|&x|x > 0.0);   // Keep only what's in front of the eye (>1.0 ? after the screen ?)
            if intersections.len() > 0 {
                let mut t: f64 = std::f64::MAX;
                for tc in intersections.iter() {
                    if tc < &t {
                        t = *tc;
                    }
                }
                let n = sphere.normal_at_point(&ray.at_t(t));
                let cos_theta = n.dot(&light);
                let c = 0.15 + 0.8 * if cos_theta > 0.0 { cos_theta} else { 0.0 };
//                println!("{:?} {:?} {}", n, light, c);
                let c = if c > 1.0 {1.0 } else { c };
                let c = if c <0.0 { 0.0 } else { c };
                let c: u32 = (c * 255.0) as u32;
                pixel = c + 0x100 * c + 0x10000 * c;
            }
            buffer[(y as usize) * width + (x as usize)] = pixel;
        }
    }
    } );

    let mut window = Window::new(
        "Ray - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }
    
}