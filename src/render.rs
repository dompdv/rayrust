

use super::vector::vector::Vector;
use super::scene::{Ray, Camera, Sphere, WorldObject, Floor, Texture};
use super::light::{Sun, LightSource, SpotLight};


pub fn render(noise:&Vec<f64>, buffer: &mut Vec<u32>, screen_width:u16, screen_height:u16, time:f64) -> () {

    fn clamp(c:f64) -> f64 {
        if c > 1.0 { return 1.0 };
        if c < 0.0 { return 0.0 };
        c
    }
    let width: usize = screen_width as usize;

    let eye = Vector::new(0.0, 0.0, 1.0);
    let direction = Vector::new(0.0, 1.0, 0.1);
    let camera = Camera::new(
        &eye,
        &direction,
        0.2,
        0.3,
        screen_width,
        screen_height
    );

    let mut world: Vec<Box<dyn WorldObject>> = Vec::new();
    world.push(Box::new(Sphere::new(&Vector::new(0.0, 2.5, 1.0), 1.0, Texture::Marble(16.0, 8.0, 9.0, 10.0, 6.0))));
    world.push(Box::new(Sphere::new(&Vector::new(0.3, 1.5, 1.8), 0.2, Texture::Uniform(1.0) )));
    world.push(Box::new(Floor::new(0.0, Texture::Checker(0.2, 1.0))));

    let sunlight = Sun::new(&Vector::new(1.5 * (4.0 + time / 10.0).cos() , 1.5 * (4.0 + time / 10.0).sin(), 1.0), 3.0);
    let spotlight = SpotLight::new(&Vector::new(1.5 * time.sin() , 1.5 * time.cos(), 5.0 + 1.5 * (2.0*time).cos()), 18.0);

    let mut t_s: Vec<f64> = Vec::with_capacity(5);

    for x in 0..screen_width {
        for y in 0..screen_height {
            //print!("{}:{} = ", x, y);
            let mut current_distance = std::f64::MAX;
            let mut current_object: Option<&Box<dyn WorldObject>> = Option::None;
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
                    let intersection_point = ray.at_t(current_distance);
                    let n = object.normal_at_point(&intersection_point);

                    let mut illumination:f64 = 0.0;
                    let color_at_point = object.color_at_point(&intersection_point, &noise);
                    illumination += sunlight.illumination_at_point(0.19, color_at_point, &ray.at_t(current_distance - 0.01), &n, &world);
                    illumination += spotlight.illumination_at_point(0.19, color_at_point, &ray.at_t(current_distance - 0.01), &n, &world);

                    // Effet distance
                    let c = illumination / (1.0 + &ray.dist_at_t(current_distance));

                    let c = (clamp(c) * 255.0) as u32;
                    pixel = c + 0x100 * c + 0x10000 * c;
                }
            }
            buffer[(y as usize) * width + (x as usize)] = pixel;
        }
    }
}
