use glam::Vec3;
use image::{Rgb, RgbImage};
use rand::RngExt;

use crate::hit::{Hitable, HitableList};
use crate::ray::Ray;

pub struct Camera {
    pub focal: f32,
    pub viewport_height: f32,
    pub pos: Vec3,
    pub dir: Vec3,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            focal: 1.0,
            viewport_height: 2.0,
            pos: Vec3::ZERO,
            dir: Vec3::NEG_Z,
            samples_per_pixel: 500,
            max_depth: 50,
        }
    }
}

impl Camera {
    /// panic if output_height == 0
    pub fn viewport_width(&self, output_width: u32, output_height: u32) -> f32 {
        assert_ne!(output_height, 0);
        self.viewport_height * (output_width as f32 / output_height as f32)
    }

    pub fn render(&self, img: &mut RgbImage, world: &HitableList) {
        let (img_w, img_h) = img.dimensions();

        let viewport_width = self.viewport_width(img_w, img_h);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -self.viewport_height, 0.0);

        let pixel_delta_u = viewport_u / img_w as f32;
        let pixel_delta_v = viewport_v / img_h as f32;

        let viewport_left_up =
            self.pos + self.dir * self.focal - (viewport_u * 0.5) - (viewport_v * 0.5);

        let pixel_left_up = viewport_left_up + pixel_delta_u * 0.5 + pixel_delta_v * 0.5;

        let mut rng = rand::rng();

        let mut get_ray = |x: u32, y: u32| {
            let x = x as f32 + rng.random_range(-0.5..0.5);
            let y = y as f32 + rng.random_range(-0.5..0.5);

            let pixel_sample = pixel_left_up + x * pixel_delta_u + y * pixel_delta_v;
            let ray_dir = pixel_sample - self.pos;
            Ray::new(self.pos, ray_dir)
        };

        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;
        for y in 0..img_h {
            for x in 0..img_w {
                let mut color = [0.0, 0.0, 0.0];

                for _ in 0..self.samples_per_pixel {
                    let ray = get_ray(x, y);
                    let c = ray_color(&ray, world, self.max_depth);
                    for i in 0..3 {
                        color[i] += c.0[i]
                    }
                }

                let rgb = linear_to_gamma(Rgb(color), pixel_samples_scale);
                img.put_pixel(x, y, rgb);
            }
        }
    }
}

fn ray_normal_color<H: Hitable>(ray: &Ray, hitable: &H, depth: u32) -> Rgb<f32> {
    if depth == 0 {
        return Rgb([0.0, 0.0, 0.0]);
    }
    if let Some(hit) = hitable.hit(ray, 0.0, 100000.0) {
        let n = (ray.at(hit.t) - Vec3::NEG_Z).normalize();
        return Rgb([0.5 * (n.x + 1.0), 0.5 * (n.y + 1.0), 0.5 * (n.z + 1.0)]);
    }

    let dir = ray.direction().normalize();
    let a = 0.5 * (dir.y + 1.0);
    return Rgb([
        (1.0 - a) + (a * 0.5),
        (1.0 - a) + (a * 0.7),
        (1.0 - a) + (a * 1.0),
    ]);
}

fn ray_color<H: Hitable>(ray: &Ray, hitable: &H, depth: u32) -> Rgb<f32> {
    if depth == 0 {
        return Rgb([0.0, 0.0, 0.0]);
    }

    if let Some(hit) = hitable.hit(ray, 0.01, 100000.0) {
        if let Some(scatter) = hit.material.scatter(ray, &hit) {
            let mut color = ray_color(&scatter.ray, hitable, depth - 1);
            for i in 0..3 {
                color.0[i] *= scatter.attenuation.0[i];
            }
            return color;
        }
        return Rgb([0.0, 0.0, 0.0]);
    }

    let dir = ray.direction().normalize();
    let a = 0.5 * (dir.y + 1.0);
    return Rgb([
        (1.0 - a) + (a * 0.5),
        (1.0 - a) + (a * 0.7),
        (1.0 - a) + (a * 1.0),
    ]);
}

fn linear_to_gamma(color: Rgb<f32>, pixel_samples_scale: f32) -> Rgb<u8> {
    Rgb([
        ((color.0[0].max(0.0) * pixel_samples_scale).sqrt() * 255.0).clamp(0.0, 255.0) as u8,
        ((color.0[1].max(0.0) * pixel_samples_scale).sqrt() * 255.0).clamp(0.0, 255.0) as u8,
        ((color.0[2].max(0.0) * pixel_samples_scale).sqrt() * 255.0).clamp(0.0, 255.0) as u8,
    ])
}
