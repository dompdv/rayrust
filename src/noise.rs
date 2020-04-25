extern crate rand;
use rand::prelude::*;

static GRID_SIZE:usize = 128;
static GRID_SIZEf:f64 = 128.0;

fn create_noise() -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut noise_grid = Vec::with_capacity(GRID_SIZE * GRID_SIZE * GRID_SIZE);
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            for z in 0..GRID_SIZE {
                noise_grid[x + y * GRID_SIZE + z * GRID_SIZE * GRID_SIZE] = rng.gen();
            }
        }
    };
    noise_grid
}

fn smmooth_noise(noise:&Vec<f64>, x:f64, y:f64, z:f64) -> f64 {
    fn pick(noise:&Vec<f64>, x:usize, y:usize, z: usize) -> f64 {
        noise[x + y * GRID_SIZE + z * GRID_SIZE * GRID_SIZE]
    }
    let x1 = x.fract() * GRID_SIZEf;
    let x2 = x1.fract();
    let i0 = x1.floor() as usize;
    let i1 = (i0 + 1) % GRID_SIZE;

    let y1 = y.fract() * GRID_SIZEf;
    let y2 = y1.fract();
    let j0 = y1.floor() as usize;
    let j1 = (j0 + 1) % GRID_SIZE;

    let z1 = z.fract() * GRID_SIZEf;
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