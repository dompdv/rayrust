extern crate rand;
use rand::prelude::*;

static GRID_SIZE:usize = 128;
static GRID_SIZEF:f64 = 128.0;

pub fn create_noise() -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut noise_grid = vec![0.0; GRID_SIZE * GRID_SIZE * GRID_SIZE];
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            for z in 0..GRID_SIZE {
                noise_grid[x + y * GRID_SIZE + z * GRID_SIZE * GRID_SIZE] = rng.gen();
            }
        }
    };
    noise_grid
}

pub fn smooth_noise(noise:&Vec<f64>, x:f64, y:f64, z:f64) -> f64 {
    fn pick(noise:&Vec<f64>, x:usize, y:usize, z: usize) -> f64 {
        noise[x + y * GRID_SIZE + z * GRID_SIZE * GRID_SIZE]
    }
    let x1 = x.abs().fract() * GRID_SIZEF;
    let x2 = x1.fract();
    let i0 = x1.floor() as usize;
    let i1 = (i0 + 1) % GRID_SIZE;

    let y1 = y.abs().fract() * GRID_SIZEF;
    let y2 = y1.fract();
    let j0 = y1.floor() as usize;
    let j1 = (j0 + 1) % GRID_SIZE;

    let z1 = z.abs().fract() * GRID_SIZEF;
    let z2 = z1.fract();
    let k0 = z1.floor() as usize;
    let k1 = (k0 + 1) % GRID_SIZE;

    pick(&noise, i0, j0, k0) * (1.0 - x2) * (1.0 - y2) * (1.0 - z2) +
    pick(&noise, i1, j0, k0) * (x2) * (1.0 - y2) * (1.0 - z2) +
    pick(&noise, i0, j1, k0) * (1.0 - x2) * (y2) * (1.0 - z2) +
    pick(&noise, i0, j0, k1) * (1.0 - x2) * (1.0 -y2) * (z2) +
    pick(&noise, i1, j0, k1) * (x2) * (1.0 - y2) * (z2) +
    pick(&noise, i0, j1, k1) * (1.0 - x2) * (y2) * (z2) +
    pick(&noise, i1, j1, k0) * (x2) * (y2) * (1.0 - z2) +
    pick(&noise, i1, j1, k1) * (x2) * (y2) * (z2)
}

pub fn turbulence(noise:&Vec<f64>, x:f64, y:f64, z:f64, size:f64) -> f64 {
  let mut value = 0.0;
  let initial_size = 2.0 * size;
  let mut mut_size = size;
  while mut_size >= 1.0 {
    value += smooth_noise(&noise, x / mut_size, y / mut_size, z / mut_size) * mut_size;
    mut_size /= 2.0;
  }
  value / initial_size
}

pub fn marble(noise: &Vec<f64>,  x:f64, y:f64, z:f64, size:f64, xp:f64, yp:f64, zp:f64, turb:f64,) -> f64 {
    let xyz_value = x * xp  + y * yp + turb * turbulence(&noise, x, y, z, size);
    (xyz_value * 3.141592654).sin().abs()
}

