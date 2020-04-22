#[macro_use]
extern crate timeit;

use minifb::{Key, Window, WindowOptions};

mod vector;
use vector::Vector::Vector;
mod scene;
use scene::Ray;


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