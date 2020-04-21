//#[macro_use]
//extern crate timeit;

use minifb::{Key, Window, WindowOptions};


#[derive(Debug)]
enum QuadraticRoots {
    None,
    Double(f64),
    Couple(f64, f64)
}

#[derive(Debug)]
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
        let y = y - half_height;
        let h = self.hor.scale(x as f64);
        let v = self.vert.scale(y as f64);
        self.center.add(&h.add(&v))
    }
}


fn ray_inter_sphere(ray: &Ray, center: &Vector, radius: f64) -> QuadraticRoots {
    let a = ray.dir.norm2();
    let diff_o_c = ray.origin.minus(center);
    let b = 2.0 * ray.dir.dot(&diff_o_c);
    let c = - radius * radius + diff_o_c.norm2();
    solve_quadratic(a, b, c)
}


fn main() {
    let screen_width = 100;
    let screen_height = 100;
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

    let center = Vector::new(0.0, 2.0, 1.0);
    let radius = 1.0;

    for x in 0..screen_width {
        for y in 0..screen_height {
            let point = camera.pixel(x, y);
            let ray = Ray::new(&eye, &point.minus(&eye));
            let intersections = ray_inter_sphere(&ray, &center, radius);
            // println!("{}:{} -> {:?} {:?}", x,y, ray, intersections);
            let pixel: u32 = 
                match intersections {
                    QuadraticRoots::None => 0, 
                    QuadraticRoots::Double(_) => 0x00FFFFFF,
                    QuadraticRoots::Couple(_, _) => 0x00FFFFFF,
                };
            buffer[(y as usize) * width + (x as usize)] = pixel;
        }
    }


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