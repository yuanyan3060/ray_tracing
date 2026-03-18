use image::Rgb;
use rand::RngExt;

use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::util::{near_zero, random_unit_vec3};

pub struct ScatterRecord {
    pub attenuation: Rgb<f32>,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    pub albedo: Rgb<f32>,
}

impl Lambertian {
    pub fn new(albedo: Rgb<f32>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = hit.normal + random_unit_vec3();

        if near_zero(scatter_direction) {
            scatter_direction = hit.normal;
        }
        let scatter_ray = Ray::new(hit.pos, scatter_direction);
        return Some(ScatterRecord {
            attenuation: self.albedo,
            ray: scatter_ray,
        });
    }
}

pub struct Metal {
    pub albedo: Rgb<f32>,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Rgb<f32>, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let reflected = ray.direction().reflect(hit.normal);
        let reflected = reflected.normalize() + (self.fuzz * random_unit_vec3());
        let scatter_ray = Ray::new(hit.pos, reflected);
        return Some(ScatterRecord {
            attenuation: self.albedo,
            ray: scatter_ray,
        });
    }
}

pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let dir = ray.direction().normalize();

        let cos_theta = (-dir).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut cannot_refract = ri * sin_theta > 1.0;
        cannot_refract =
            cannot_refract || reflectance(cos_theta, ri) > rand::rng().random_range(0.0..1.0);

        let refracted = if cannot_refract {
            dir.reflect(hit.normal)
        } else {
            dir.refract(hit.normal, ri)
        };

        let scatter_ray = Ray::new(hit.pos, refracted);

        return Some(ScatterRecord {
            attenuation: Rgb([1.0, 1.0, 1.0]),
            ray: scatter_ray,
        });
    }
}

fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
