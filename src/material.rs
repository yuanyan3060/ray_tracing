use image::Rgb;

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
