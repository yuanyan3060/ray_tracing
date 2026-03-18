use glam::Vec3;

use crate::material::Material;
use crate::ray::Ray;

pub struct HitRecord<'a> {
    pub pos: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, ray_min: f32, ray_max: f32) -> Option<HitRecord<'_>>;
}

pub fn face_normal(ray: &Ray, outward_normal: &mut Vec3) -> bool {
    let front_face = ray.direction().dot(*outward_normal) < 0.0;
    if !front_face {
        *outward_normal = -*outward_normal;
    }

    front_face
}

pub struct Sphere<M: Material> {
    pub pos: Vec3,
    pub radius: f32,
    pub material: M,
}

impl<M: Material> Hitable for Sphere<M> {
    fn hit(&self, ray: &Ray, ray_min: f32, ray_max: f32) -> Option<HitRecord<'_>> {
        let oc = self.pos - ray.postion();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root <= ray_min || ray_max <= root {
            root = (h + sqrtd) / a;
            if root <= ray_min || ray_max <= root {
                return None;
            }
        }

        let t = root;
        let pos = ray.at(t);
        let mut normal = (pos - self.pos) / self.radius;
        let front_face = face_normal(ray, &mut normal);

        return Some(HitRecord {
            pos,
            normal,
            t,
            front_face,
            material: &self.material,
        });
    }
}

pub struct HitableList {
    pub objects: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn push<T: Hitable + 'static>(&mut self, h: T) {
        self.objects.push(Box::new(h) as _);
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, ray_min: f32, ray_max: f32) -> Option<HitRecord<'_>> {
        let mut output = None;
        let mut closet_so_far = ray_max;

        for obj in &self.objects {
            let Some(hit) = obj.hit(ray, ray_min, closet_so_far) else {
                continue;
            };

            closet_so_far = hit.t;
            output = Some(hit);
        }

        output
    }
}
