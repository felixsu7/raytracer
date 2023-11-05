mod hittable;
mod interval;
mod material;
mod ray;
mod renderer;
mod scene;
mod sphere;
mod utils;
mod vec3;

use renderer::{RenderOptions, Renderer};
use scene::Scene;
use std::time::Instant;
use utils::*;
use vec3::Vec3;

fn main() {
    let now = Instant::now();

    let mut scene = Scene::new();

    // ground
    scene.push(lambertian_sphere(0.0, -100.5, -1.0, 100.0, 0.2, 0.6, 0.2));

    // glass
    scene.push(glass_sphere(0.0, 0.0, 0.0, 0.5, 0.8, 0.8, 0.4, 2.7));

    // red
    scene.push(metal_sphere(0.0, 0.0, 1.5, 0.5, 0.7, 0.3, 0.3, 0.01));

    // blue
    scene.push(lambertian_sphere(-1.5, 0.0, 0.0, 0.5, 0.3, 0.3, 0.7));

    // rgb sphere
    scene.push(something_sphere(0.8, 0.0, -0.8, 0.3));

    let renderer = Renderer::new(RenderOptions {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 250,
        vfov: 100.0,
        lookfrom: Vec3::new(1.0, 1.5, 0.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        sky_color: Vec3::new(0.4, 0.4, 0.9),
        defocus_angle: 15.0,
        focus_dist: 1.5,
    });
    renderer.render(&scene, "render.png");

    println!("Render done! (took {:.2?})", now.elapsed());
}
