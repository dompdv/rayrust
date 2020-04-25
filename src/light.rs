
use crate::vector::vector::Vector;

use crate::scene::{WorldObject, Ray};

pub trait LightSource {
    fn illumination_at_point(&self, ambient: f64, color: f64, point:&Vector, normal:&Vector, world: &Vec<Box<dyn WorldObject>>) -> f64;    
}

pub struct Sun {
    dir: Vector,
    intensity: f64
}

impl Sun {
    pub fn new(dir:&Vector, intensity:f64) -> Sun {
        Sun { dir: Vector::new(dir.x, dir.y, dir.z).normalized(),
        intensity}
    }
}


impl LightSource for Sun {
    fn illumination_at_point(&self, ambient: f64, color: f64, point:&Vector, normal:&Vector, world: &Vec<Box<dyn WorldObject>>) -> f64 {
        let cos_theta = normal.dot(&self.dir);
        let shadow = there_is_shadow(&point, &self.dir, &world, std::f64::MAX);
        let c = color * (if shadow || cos_theta < 0.0 { ambient } else { self.intensity });
        return c * cos_theta;
}
}

pub struct SpotLight {
    position: Vector,
    intensity: f64
}

impl SpotLight {
    pub fn new(position:&Vector, intensity:f64) -> SpotLight {
        SpotLight { position: Vector::new(position.x, position.y, position.z), intensity}
    }
}

impl LightSource for SpotLight {
    fn illumination_at_point(&self, ambient: f64, color: f64, point:&Vector, normal:&Vector, world: &Vec<Box<dyn WorldObject>>) -> f64 {
        let direction = self.position.minus(point);
        let cos_theta = normal.dot(&direction.normalized()); 
        let shadow = there_is_shadow(&point, &direction, &world, 1.0);
        //println!("s={}, cost={}", shadow, cos_theta);
        //println!("n={:?}, direction={:?}", normal, direction.normalized());
        let c = color * (if shadow || cos_theta < 0.0 { ambient } else { self.intensity });
        let distance = direction.norm2();
        return c * cos_theta / distance;
}
}


fn there_is_shadow(from: &Vector, dir: &Vector, world: &Vec<Box<dyn WorldObject>>, upper_bound:f64 ) -> bool {
    //print!("from pt {:?} /", from);
    let ray = Ray::new(&from, &dir);
    let mut t_s: Vec<f64> = Vec::with_capacity(5);
    for object in world.iter() {
        object.ray_intersections(&ray, &mut t_s);
        //println!("len:{}", t_s.len());
        if t_s.len() > 0 {
            for &t in t_s.iter() {
                if t > 0.0  && t < upper_bound {
                    //print!("(S) {}", object.who_am_i());
                    //print!(" w={:?}", ray.at_t(t_s[0]));
                    return true;
                }
            }
        }
    }
    //print!("(NS)");
    false
}
