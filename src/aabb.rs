use std::ops::Range;

use glam::Vec3;

use crate::{ray::Ray, util::Aixs};

#[derive(Clone)]
pub struct AABB {
    pub x: Range<f32>,
    pub y: Range<f32>,
    pub z: Range<f32>,
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            x: f32::INFINITY..-f32::INFINITY,
            y: f32::INFINITY..-f32::INFINITY,
            z: f32::INFINITY..-f32::INFINITY,
        }
    }
}

impl AABB {
    #[allow(unused)]
    pub fn new(x: Range<f32>, y: Range<f32>, z: Range<f32>) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Vec3, b: Vec3) -> Self {
        Self {
            x: a.x.min(b.x)..a.x.max(b.x),
            y: a.y.min(b.y)..a.y.max(b.y),
            z: a.z.min(b.z)..a.z.max(b.z),
        }
    }

    pub fn aixs(&self, idx: usize) -> &Range<f32> {
        if idx == 1 {
            &self.y
        } else if idx == 2 {
            &self.z
        } else {
            &self.x
        }
    }

    pub fn hit(&self, ray: &Ray, mut min: f32, mut max: f32) -> bool {
        let pos = ray.postion().to_array();
        let dir = ray.direction().to_array();

        for idx in 0..3 {
            let aix = self.aixs(idx);
            let adinv = 1.0 / dir[idx];

            let t0 = (aix.start - pos[idx]) * adinv;
            let t1 = (aix.end - pos[idx]) * adinv;

            if t0 < t1 {
                min = min.max(t0);
                max = max.min(t1);
            } else {
                min = min.max(t1);
                max = max.min(t0);
            }

            if max <= min {
                return false;
            }
        }

        true
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            x: self.x.start.min(other.x.start)..self.x.end.max(other.x.end),
            y: self.y.start.min(other.y.start)..self.y.end.max(other.y.end),
            z: self.z.start.min(other.z.start)..self.z.end.max(other.z.end),
        }
    }

    pub fn longest_axis(&self) -> Aixs {
        let mut axis = &self.x;
        let mut idx = Aixs::X;

        if self.y.end - self.y.start > axis.end - axis.start {
            axis = &self.y;
            idx = Aixs::Y
        }

        if self.z.end - self.z.start > axis.end - axis.start {
            idx = Aixs::Z
        }

        idx
    }
}
