use crate::{aabb::AABB, hit::Hitable, util::Aixs};

pub struct BvhNode {
    left: Option<Box<dyn Hitable>>,
    right: Option<Box<dyn Hitable>>,
    aabb: AABB,
}

impl BvhNode {
    pub fn new(mut objects: Vec<Box<dyn Hitable>>) -> Self {
        if objects.is_empty() {
            return Self {
                left: None,
                right: None,
                aabb: AABB::default(),
            };
        }

        if objects.len() == 1 {
            let object = objects.pop().unwrap();
            let aabb = object.bounding_box();
            return Self {
                left: Some(object),
                right: None,
                aabb,
            };
        }

        if objects.len() == 2 {
            let right = objects.pop().unwrap();
            let left = objects.pop().unwrap();
            let aabb = left.bounding_box().merge(&right.bounding_box());
            return Self {
                left: Some(left),
                right: Some(right),
                aabb,
            };
        }

        let mut aabb = AABB::default();
        for object in &objects {
            aabb = aabb.merge(&object.bounding_box());
        }

        match aabb.longest_axis() {
            Aixs::X => objects.sort_by(|left, right| {
                let left = left.bounding_box().x.start;
                let right = right.bounding_box().x.start;

                left.total_cmp(&right)
            }),
            Aixs::Y => objects.sort_by(|left, right| {
                let left = left.bounding_box().y.start;
                let right = right.bounding_box().y.start;

                left.total_cmp(&right)
            }),
            Aixs::Z => objects.sort_by(|left, right| {
                let left = left.bounding_box().z.start;
                let right = right.bounding_box().z.start;

                left.total_cmp(&right)
            }),
        };

        let mid = objects.len() / 2;
        let others = objects.split_off(mid);

        let left = Self::new(objects);
        let right = Self::new(others);

        Self {
            left: Some(Box::new(left) as _),
            right: Some(Box::new(right) as _),
            aabb,
        }
    }
}

impl Hitable for BvhNode {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_min: f32,
        mut ray_max: f32,
    ) -> Option<crate::hit::HitRecord<'_>> {
        if !self.aabb.hit(ray, ray_min, ray_max) {
            return None;
        }

        let hit_left = if let Some(left) = &self.left {
            left.hit(ray, ray_min, ray_max)
        } else {
            None
        };

        if let Some(hit_left) = &hit_left {
            ray_max = hit_left.t;
        }

        let hit_right = if let Some(right) = &self.right {
            right.hit(ray, ray_min, ray_max)
        } else {
            None
        };

        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> AABB {
        self.aabb.clone()
    }
}
