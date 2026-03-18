use std::f32::consts::PI;

use glam::Vec3;
use image::Rgb;
use rand::RngExt;

use crate::hit::{HitableList, Sphere};
use crate::material::{Dielectric, Lambertian, Metal};

mod camrea;
mod hit;
mod material;
mod ray;
mod util;

fn main() {
    let mut world = HitableList::new();

    world.push(Sphere {
        pos: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian::new(Rgb([0.5, 0.5, 0.5])),
    });

    let mut rng = rand::rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random_range(0.0..1.0);
            let pos = Vec3::new(
                a as f32 + rng.random_range(0.0..1.0),
                0.2,
                b as f32 + 0.9 * rng.random_range(0.0..1.0),
            );

            if (pos - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let mut albedo = Vec3::new(
                        rng.random_range(0.0..1.0),
                        rng.random_range(0.0..1.0),
                        rng.random_range(0.0..1.0),
                    );
                    albedo *= albedo;
                    world.push(Sphere {
                        pos,
                        radius: 0.2,
                        material: Lambertian::new(Rgb(albedo.to_array())),
                    });
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::new(
                        rng.random_range(0.5..1.0),
                        rng.random_range(0.5..1.0),
                        rng.random_range(0.5..1.0),
                    );
                    let fuzz = rng.random_range(0.0..0.5);
                    world.push(Sphere {
                        pos,
                        radius: 0.2,
                        material: Metal::new(Rgb(albedo.to_array()), fuzz),
                    });
                } else {
                    world.push(Sphere {
                        pos,
                        radius: 0.2,
                        material: Dielectric::new(1.5),
                    });
                }
            }
        }
    }

    world.push(Sphere {
        pos: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric::new(1.5),
    });

    world.push(Sphere {
        pos: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian::new(Rgb([0.4, 0.2, 0.1])),
    });

    world.push(Sphere {
        pos: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal::new(Rgb([0.7, 0.6, 0.5]), 0.0),
    });

    let mut img = image::RgbImage::from_fn(1280, 720, |_, _| Rgb([0, 0, 0]));
    let mut camera = camrea::Camera::default();

    camera.samples_per_pixel = 500;
    camera.max_depth = 50;
    camera.vfov = (PI / 180.0) * 20.0;
    camera.look_from = Vec3::new(13.0, 2.0, 3.0);
    camera.look_at = Vec3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus = (PI / 180.0) * 0.6;
    camera.foucus_dist = 10.0;

    camera.render(&mut img, &world);
    img.save("output.png").unwrap()
}
