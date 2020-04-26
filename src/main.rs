//extern crate timeit;

use minifb::{Key, Window, WindowOptions};


mod noise;
use crate::noise::{create_noise};
mod render;
use crate::render::render;
mod vector;
mod scene;
mod light;

fn main() {

    let noise = create_noise();
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

    // Limit to max ~xx fps update rate
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

        render(&noise, &mut buffer, screen_width, screen_height, time);
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }
    
}