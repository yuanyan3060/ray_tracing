use glam::Vec3;
use image::Rgb;

use crate::hit::{HitableList, Sphere};
use crate::material::{Lambertian, Metal};

mod ray;
mod camrea;
mod hit;
mod material;
mod util;

fn main() {
    let mut img = image::RgbImage::from_fn(1280, 720, |_, _| Rgb([0, 0, 0]));
    let camera = camrea::Camera::default();
    let mut world = HitableList::new();

    world.push(Sphere {
        pos: Vec3::new(-0.5, 0.0, -1.0),
        radius: 0.5,
        material: Metal::new(Rgb([0.8, 0.1, 0.1]), 0.01)
    });
    world.push(Sphere {
        pos: Vec3::new(0.5, -0.3, -1.0),
        radius: 0.2,
        material: Lambertian::new(Rgb([0.1, 0.1, 0.8]))
    });
    world.push(Sphere {
        pos: Vec3::new(0.0, -1000.5, -1.0),
        radius: 1000.0,
        material: Lambertian::new(Rgb([0.5, 0.5, 0.5]))
    });

    camera.render(&mut img, &world);
    img.save("output.png").unwrap()
}
