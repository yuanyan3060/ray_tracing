use glam::Vec3;
use rand::RngExt;

pub fn random_unit_vec3() -> Vec3 {
    let mut rng = rand::rng();

    loop {
        let v = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        );
        let lensq = v.length_squared();
        if lensq <= 1.0 {
            if let Some(v) = v.try_normalize() {
                return v;
            }
        }
    }
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vec3();
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::rng();
    loop {
        let v = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            0.0,
        );
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

pub fn near_zero(v: Vec3) -> bool {
    let s = 1e-8;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

#[derive(Clone, Copy, Debug)]
pub enum Aixs {
    X = 0,
    Y = 1,
    Z = 2,
}
