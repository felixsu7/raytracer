use rand::random;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use crate::hittable::Intersection;
use crate::interval::Interval;
use crate::material::scatter;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::Vec3;

type RawColor = [u8; 3];

pub struct RenderOptions {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u8,
    pub vfov: f64,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub sky_color: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

pub struct Renderer {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub center: Vec3,
    pub pixel00_loc: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub samples_per_pixel: u32,
    pub max_depth: u8,
    pub vfov: f64,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub sky_color: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
}

impl Renderer {
    pub fn new(options: RenderOptions) -> Renderer {
        let image_height = (options.image_width as f64 / options.aspect_ratio) as u32;

        let theta = f64::to_radians(options.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * options.focus_dist;
        let viewport_width = viewport_height * (options.image_width as f64 / image_height as f64);

        let w = (options.lookfrom - options.lookat).unit_vector();
        let u = options.vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * v.inverse();

        let pixel_delta_u = viewport_u / options.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            options.lookfrom - (options.focus_dist * w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius =
            options.focus_dist * f64::tan(f64::to_radians(options.defocus_angle / 2.0));

        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Renderer {
            aspect_ratio: options.aspect_ratio,
            image_width: options.image_width,
            image_height,
            center: options.lookfrom,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: options.samples_per_pixel,
            max_depth: options.max_depth,
            vfov: options.vfov,
            lookfrom: options.lookfrom,
            lookat: options.lookat,
            vup: options.vup,
            u,
            v,
            w,
            sky_color: options.sky_color,
            defocus_angle: options.defocus_angle,
            focus_dist: options.focus_dist,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, scene: &Scene, file_name: &str) {
        let path = Path::new(file_name);
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);

        let mut encoder = png::Encoder::new(writer, self.image_width, self.image_height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();
        let mut writer = writer.stream_writer().unwrap();

        for y in 0..self.image_height {
            println!(
                "Scanlines remaining: {}/{}",
                self.image_height - y,
                self.image_height
            );

            let scanline = self.render_scanline(scene, y);
            let buffer = scanline.concat();
            writer.write_all(&buffer).unwrap();
        }

        writer.finish().unwrap();
    }

    fn render_scanline(&self, scene: &Scene, y: u32) -> Vec<RawColor> {
        let mut scanline = Vec::<RawColor>::new();
        scanline.reserve_exact(self.image_width as usize);

        for x in 0..self.image_width {
            let pixel = self.render_pixel(scene, x, y);
            scanline.push(pixel);
        }

        scanline
    }

    fn render_pixel(&self, scene: &Scene, x: u32, y: u32) -> RawColor {
        let mut pixel_color = Vec3::default();
        for _ in 0..self.samples_per_pixel {
            let sample_ray = self.sample_ray(x, y);
            pixel_color = pixel_color + self.ray_color(&sample_ray, scene, self.max_depth);
        }

        self.raw_color(pixel_color)
    }

    fn ray_color(&self, ray: &Ray, scene: &Scene, depth: u8) -> Vec3 {
        let mut intersection = Intersection::default();

        if depth == 0 {
            return Vec3::default();
        }

        if scene.hit(ray, &Interval::new(0.001, f64::INFINITY), &mut intersection) {
            let mut scattered = Ray::default();
            let mut attenuation = Vec3::default();
            if scatter(
                intersection.material.clone(),
                ray,
                &intersection,
                &mut attenuation,
                &mut scattered,
            ) {
                return attenuation * self.ray_color(&scattered, scene, depth - 1);
            }
            return Vec3::default();
        }

        let unit_direction = ray.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + self.sky_color * a
    }

    fn sample_ray(&self, u: u32, v: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (self.pixel_delta_u * u as f64) + (self.pixel_delta_v * v as f64);
        let pixel_sample = pixel_center + self.pixel_sample_sqaure();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        ray_origin.with_direction(ray_direction)
    }

    fn pixel_sample_sqaure(&self) -> Vec3 {
        let px = -0.5 + random::<f64>();
        let py = -0.5 + random::<f64>();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn raw_color(&self, color: Vec3) -> RawColor {
        let scale = 1.0 / self.samples_per_pixel as f64;

        // sqrting does the gamma correction
        let r = f64::sqrt(color.x * scale);
        let g = f64::sqrt(color.y * scale);
        let b = f64::sqrt(color.z * scale);

        [
            (r * 255.999) as u8,
            (g * 255.999) as u8,
            (b * 255.999) as u8,
        ]
    }
}
