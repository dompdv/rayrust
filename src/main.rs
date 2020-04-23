#[macro_use]
extern crate timeit;

use minifb::{Key, Window, WindowOptions};

mod vector;
use vector::vector::Vector;
mod scene;
use scene::{Ray, Camera, Sphere, RayIntersect, Floor};

fn is_there_shadow(from: &Vector, dir: &Vector, world: &Vec<Box<dyn RayIntersect>> ) -> bool {
    // print!("from pt {:?} /", from);
    let ray = Ray::new(&from, &dir);
    let mut t_s: Vec<f64> = Vec::with_capacity(5);
    for object in world.iter() {
        object.ray_intersections(&ray, &mut t_s);
        // println!("len:{}", t_s.len());
        if t_s.len() > 0 {
            for &t in t_s.iter() {
                if t > 0.0 {
                    // print!("(S) {}", object.who_am_i());
                    // print!(" w={:?}", ray.at_t(t_s[0]));
                    return true;
                }
            }
        }
    }
    //print!("(NS)");
    false
}


fn main() {
    let screen_width = 600;
    let screen_height = 600;
    let width: usize = screen_width as usize;
    let height: usize = screen_height as usize;

    let mut buffer: Vec<u32> = vec![0; width * height];


    let mut window = Window::new(
        "Ray - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let now = std::time::SystemTime::now();
    let mut time:f64 = 0.0;
    let mut fl:f64 = 0.0;
    const VELOCITY: f64 = 0.005;
    let mut v_fl:f64 = VELOCITY;
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_millis(30)));


    while window.is_open() && !window.is_key_down(Key::Escape) {
        match now.elapsed() {
            Ok(elapsed) => {
                time = (elapsed.as_millis() as f64) / 1000.0;
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }        
        fl = fl + v_fl;
        //println!("FL={} VFL={}", fl, v_fl);
        if fl > 0.7 { v_fl = - VELOCITY };
        if fl < 0.0 { v_fl = VELOCITY };


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
    
        let mut world: Vec<Box<dyn RayIntersect>> = Vec::new();
        world.push(Box::new(Sphere::new(&Vector::new(0.0, 2.5, 1.0), 1.0)));
        world.push(Box::new(Sphere::new(&Vector::new(0.3, 1.5, 1.8), 0.2)));
        world.push(Box::new(Floor::new(0.0)));

        let light = Vector::new(0.2 * time.sin() , 0.3 * time.cos(), 1.0).normalized();

    
        let mut t_s: Vec<f64> = Vec::with_capacity(5);
    
        for x in 0..screen_width {
            for y in 0..screen_height {
                // print!("{}:{} = ", x, y);
                let mut current_distance = std::f64::MAX;
                let mut current_object: Option<&Box<dyn RayIntersect>> = Option::None;
                let point = camera.pixel(x, y);
                let ray = Ray::new(&eye, &point.minus(&eye));
                for object in world.iter() {
                    t_s.clear();
                    object.ray_intersections(&ray, &mut t_s);
                    for &t in t_s.iter() {
                        if t > 0.0 && t < current_distance {
                            current_distance = t;
                            current_object = Some(object);
                        }
                    }
                }
                let mut pixel:u32 = 0x050505;
                match current_object {
                    None => {},
                    Some(object) => {
                        // print!("Collide:{} / ", object.who_am_i());
                        let distance = &ray.dist_at_t(current_distance);
                        let intersection_point = ray.at_t(current_distance);
                        // print!("inters pt {:?} /", intersection_point);
                        let n = object.normal_at_point(&intersection_point);
                        let cos_theta = n.dot(&light);
                        let c = if is_there_shadow(&ray.at_t(current_distance - 0.1), &light, &world) {
                            0.1 
                        } else { 
                            4.0 * (0.15 + 0.8 * if cos_theta > 0.0 { cos_theta } else { 0.0 }) / (1.0 + distance)
                        };
                        //println!();
                        // println!("{:?} {:?} {}", n, light, c);
                        let c = if c > 1.0 {1.0 } else { c };
                        let c = if c < 0.0 { 0.0 } else { c };
                        let c: u32 = (c * 255.0) as u32;
                        pixel = c + 0x100 * c + 0x10000 * c;
                    }
                }
                buffer[(y as usize) * width + (x as usize)] = pixel;
            }
        }
    
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }
    
}