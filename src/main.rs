#![feature(generic_const_exprs)]
#![allow(dead_code)]

#[cfg(not(feature = "raytracing"))]
extern crate anyhow;
#[cfg(not(feature = "raytracing"))]
extern crate yew;

mod la;
mod shader;
mod tga;
mod camera;
mod transform;
mod utils;
mod raytracing;
mod scene;
mod models;
mod render;
#[cfg(not(feature = "raytracing"))]
mod web;


#[cfg(feature = "raytracing")]

use utils::{random_scene,ray_color};
use la::Vec3f;
use camera::CameraTrait;
use raytracing::{
    ExposureCamera,
    Exposure,
};
use indicatif::ProgressBar;
use lodepng::RGB;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::path::Path;

#[cfg(not(feature = "raytracing"))]
use web::web;

#[cfg(not(feature = "raytracing"))]
fn main() {
    web();
}

#[cfg(feature = "raytracing")]

fn main() {
    use rand::Rng;


    const WIDTH: usize = 1200;
    const HEIGHT: usize = 800;
    const ASPECT: f32 = WIDTH as f32/ HEIGHT as f32;
    const SAMPLES_PER_PIXEL: usize = 50;
    const MAX_DEPTH: usize = 200;
    // World
    let world = random_scene();
    let mut camera = ExposureCamera::default();
    let position = Vec3f(13.0, 2.0, 3.0);
    let view = Vec3f(0.0,0.0,0.0);
    let aperture = 0.1;
    camera.set_aspect(ASPECT);
    camera.set_aperture(aperture);
    camera.set_focus(view);
    camera.set_position(position);

    // Progress bar
    let bar = ProgressBar::new(HEIGHT as u64);

    // Render
    let white = (256.0 * f32::clamp(1.0, 0.0, 0.999)) as u8;
    let mut pixels: Vec<RGB<u8>> = vec![RGB::new(white, white, white); WIDTH * HEIGHT];
    let bands: Vec<(usize, &mut [RGB<u8>])> = pixels.chunks_mut(WIDTH).enumerate().collect();
    bands.into_par_iter().for_each(|(row, band)| {
        let height = HEIGHT - row;
        let mut rng = rand::thread_rng();
        for column in 0..WIDTH {
            let mut pixel_color = Vec3f(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u: f32 = (column as f32 + rng.gen_range(0.0,1.0)) / (WIDTH - 1) as f32;
                let v: f32 = (height as f32 + rng.gen_range(0.0,1.0)) / (HEIGHT - 1) as f32;
                let r = camera.exposure_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
            }
            let pixel = pixel_color.to_rgb_sampled(SAMPLES_PER_PIXEL);
            band[column] = pixel;
        }
        bar.inc(1);
    });

    bar.finish();

    let path = &Path::new("image.png");

    if let Err(e) = lodepng::encode_file(path, &pixels, WIDTH, HEIGHT, lodepng::ColorType::RGB, 8) {
        panic!("failed to write png: {:?}", e);
    }

    println!("Written to {}", path.display());
}