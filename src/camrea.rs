use std::f32::consts::PI;

use glam::Vec3;
use image::{Rgb, RgbImage};
use rand::RngExt;

use crate::hit::{Hitable, HitableList};
use crate::ray::Ray;
use crate::util::random_in_unit_disk;

pub struct Camera {
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub vup: Vec3,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f32,
    pub defocus: f32,
    pub foucus_dist: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            look_from: Vec3::ZERO,
            look_at: Vec3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            samples_per_pixel: 200,
            max_depth: 50,
            vfov: PI * 0.5,
            defocus: PI * 0.0,
            foucus_dist: 10.0,
        }
    }
}

impl Camera {
    pub fn set_vfov_from_degree(&mut self, degree: f32) {
        self.vfov = PI * degree / 180.0
    }

    fn defocus_disk_sample(&self, defocus_disk_u: Vec3, defocus_disk_v: Vec3) -> Vec3 {
        let v = random_in_unit_disk();
        self.look_from + (v.x * defocus_disk_u) + (v.y * defocus_disk_v)
    }

    pub fn render(&self, img: &mut RgbImage, world: &HitableList) {
        let (img_w, img_h) = img.dimensions();

        let pos = self.look_from;

        let viewport_height = 2.0 * (self.vfov * 0.5).tan() * self.foucus_dist;
        let viewport_width = viewport_height * (img_w as f32 / img_h as f32);

        let w = (self.look_from - self.look_at).normalize();
        let u = self.vup.cross(w);
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_delta_u = viewport_u / img_w as f32;
        let pixel_delta_v = viewport_v / img_h as f32;

        let viewport_left_up =
            pos - (w * self.foucus_dist) - (viewport_u * 0.5) - (viewport_v * 0.5);

        let pixel_left_up = viewport_left_up + pixel_delta_u * 0.5 + pixel_delta_v * 0.5;

        let defocus_radius = self.foucus_dist * (self.defocus * 0.5).tan();

        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

        let mut rng = rand::rng();

        let mut get_ray = |x: u32, y: u32| {
            let x = x as f32 + rng.random_range(-0.5..0.5);
            let y = y as f32 + rng.random_range(-0.5..0.5);

            let ray_pos = if self.defocus <= 0.0 {
                pos
            } else {
                self.defocus_disk_sample(defocus_disk_u, defocus_disk_v)
            };

            let pixel_sample = pixel_left_up + x * pixel_delta_u + y * pixel_delta_v;
            let ray_dir = pixel_sample - ray_pos;
            Ray::new(ray_pos, ray_dir)
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
