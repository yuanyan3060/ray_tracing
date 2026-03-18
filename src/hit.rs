use std::f32::consts::PI;

use glam::{Vec2, Vec3};

use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;

pub struct HitRecord<'a> {
    pub pos: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

#[cfg(feature = "parallel")]
pub trait Hitable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_min: f32, ray_max: f32) -> Option<HitRecord<'_>>;
    fn bounding_box(&self) -> AABB;
}

#[cfg(not(feature = "parallel"))]
pub trait Hitable {
    fn hit(&self, ray: &Ray, ray_min: f32, ray_max: f32) -> Option<HitRecord<'_>>;
    fn bounding_box(&self) -> AABB;
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

impl<M: Material> Sphere<M> {
    pub fn new(pos: Vec3, radius: f32, material: M) -> Self {
        Self {
            pos,
            radius,
            material,
        }
    }

    fn get_uv(&self, pos: Vec3) -> (f32, f32) {
        let theta = (-pos.y).acos();
        let phi = (-pos.z).atan2(pos.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
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

        let (u, v) = self.get_uv(normal);
        return Some(HitRecord {
            pos,
            normal,
            t,
            u,
            v,
            front_face,
            material: &self.material,
        });
    }

    fn bounding_box(&self) -> AABB {
        let offset = Vec3::new(self.radius, self.radius, self.radius);
        AABB::from_points(self.pos - offset, self.pos + offset)
    }
}

pub struct Quad<M: Material> {
    pub start: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub material: M,
}

impl<M: Material> Quad<M> {
    pub fn new(start: Vec3, u: Vec3, v: Vec3, material: M) -> Self {
        Self {
            start,
            u,
            v,
            material,
        }
    }

    pub fn is_interior(&self, a: f32, b: f32) -> Option<(f32, f32)> {
        if !(0.0..1.0).contains(&a) {
            None
        } else if !(0.0..1.0).contains(&b) {
            None
        } else {
            Some((a, b))
        }
    }
}

impl<M: Material> Hitable for Quad<M> {
    fn hit(&self, ray: &Ray, ray_min: f32, ray_max: f32) -> Option<HitRecord<'_>> {
        let n = self.u.cross(self.v);
        let mut normal = n.normalize();
        let d = normal.dot(self.start);
        let w = n / n.dot(n);

        let denom = normal.dot(ray.direction());
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (d - normal.dot(ray.postion())) / denom;
        if t < ray_min || t > ray_max {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.start;
        let alpha = w.dot(planar_hitpt_vector.cross(self.v));
        let beta = w.dot(self.u.cross(planar_hitpt_vector));
        let (u, v) = self.is_interior(alpha, beta)?;
        let front_face = face_normal(ray, &mut normal);

        Some(HitRecord {
            pos: intersection,
            normal,
            t,
            u,
            v,
            front_face,
            material: &self.material,
        })
    }

    fn bounding_box(&self) -> AABB {
        let aabb0 = AABB::from_points(self.start, self.start + self.u + self.v);
        let aabb1 = AABB::from_points(self.start + self.u, self.start + self.v);
        aabb0.merge(&aabb1)
    }
}

pub struct Tri<M: Material> {
    pub start: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub uvs: [(f32, f32); 3],
    pub material: M,
}

impl<M: Material> Tri<M> {
    pub fn new(start: Vec3, u: Vec3, v: Vec3, material: M) -> Self {
        Self {
            start,
            u,
            v,
            uvs: [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)],
            material,
        }
    }

    pub fn with_uvs(self, uvs: [(f32, f32); 3]) -> Self {
        Self {
            start: self.start,
            u: self.u,
            v: self.v,
            uvs,
            material: self.material,
        }
    }

    pub fn is_interior(&self, a: f32, b: f32) -> Option<(f32, f32)> {
        if a > 0.0 && b > 0.0 && a + b < 1.0 {
            Some((a, b))
        } else {
            None
        }
    }
}

impl<M: Material> Hitable for Tri<M> {
    fn hit(&self, ray: &Ray, ray_min: f32, ray_max: f32) -> Option<HitRecord<'_>> {
        let n = self.u.cross(self.v);
        let mut normal = n.normalize();
        let d = normal.dot(self.start);
        let w = n / n.dot(n);

        let denom = normal.dot(ray.direction());
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (d - normal.dot(ray.postion())) / denom;
        if t < ray_min || t > ray_max {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.start;
        let alpha = w.dot(planar_hitpt_vector.cross(self.v));
        let beta = w.dot(self.u.cross(planar_hitpt_vector));
        let (u, v) = self.is_interior(alpha, beta)?;
        let front_face = face_normal(ray, &mut normal);

        let hit_u = (1.0 - u - v) * self.uvs[0].0 + u * self.uvs[1].0 + v * self.uvs[2].0;
        let hit_v = (1.0 - u - v) * self.uvs[0].1 + u * self.uvs[1].1 + v * self.uvs[2].1;

        Some(HitRecord {
            pos: intersection,
            normal,
            t,
            u: hit_u,
            v: hit_v,
            front_face,
            material: &self.material,
        })
    }

    fn bounding_box(&self) -> AABB {
        let aabb0 = AABB::from_points(self.start, self.start + self.u + self.v);
        let aabb1 = AABB::from_points(self.start + self.u, self.start + self.v);
        aabb0.merge(&aabb1)
    }
}

pub struct HitableList {
    pub objects: Vec<Box<dyn Hitable>>,
    pub aabb: AABB,
}

impl HitableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            aabb: AABB::default(),
        }
    }

    pub fn push<T: Hitable + 'static>(&mut self, h: T) {
        let aabb = h.bounding_box();
        self.aabb = self.aabb.merge(&aabb);
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

    fn bounding_box(&self) -> AABB {
        self.aabb.clone()
    }
}
