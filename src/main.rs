use glam::Vec3;
use image::Rgb;

use crate::hit::{HitableList, Sphere};
use crate::material::{Dielectric, Lambertian, Metal};

mod camrea;
mod hit;
mod material;
mod ray;
mod util;

fn main() {
    let mut img = image::RgbImage::from_fn(1280, 720, |_, _| Rgb([0, 0, 0]));
    let camera = camrea::Camera::default();
    let mut world = HitableList::new();

    world.push(Sphere {
        pos: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Lambertian::new(Rgb([0.8, 0.8, 0.0])),
    });

    world.push(Sphere {
        pos: Vec3::new(0.0, 0.0, -1.2),
        radius: 0.5,
        material: Lambertian::new(Rgb([0.1, 0.2, 0.5])),
    });

    world.push(Sphere {
        pos: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Dielectric::new(1.5),
    });

    world.push(Sphere {
        pos: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.4,
        material: Dielectric::new(1.0 / 1.5),
    });
    
    world.push(Sphere {
        pos: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Metal::new(Rgb([0.8, 0.6, 0.2]), 0.0),
    });

    camera.render(&mut img, &world);
    img.save("output.png").unwrap()
}
