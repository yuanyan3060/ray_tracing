use std::sync::Arc;

use glam::Vec3;
use image::{Rgb, Rgb32FImage};

#[cfg(feature = "parallel")]
pub trait Texture: Send + Sync {
    fn color(&self, u: f32, v: f32, pos: Vec3) -> Rgb<f32>;
}

#[cfg(not(feature = "parallel"))]
pub trait Texture {
    fn color(&self, u: f32, v: f32, pos: Vec3) -> Rgb<f32>;
}

impl<T: Texture + Send> Texture for Arc<T> {
    fn color(&self, u: f32, v: f32, pos: Vec3) -> Rgb<f32> {
        T::color(&self, u, v, pos)
    }
}

pub struct SolidColor {
    pub albedo: Rgb<f32>,
}

impl SolidColor {
    pub fn new(albedo: Rgb<f32>) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn color(&self, _u: f32, _v: f32, _pos: Vec3) -> Rgb<f32> {
        self.albedo
    }
}

pub struct CheckerTexture {
    pub inv_scale: f32,
    pub even: Box<dyn Texture>,
    pub odd: Box<dyn Texture>,
}

impl CheckerTexture {
    #[allow(unused)]
    pub fn new<T0, T1>(inv_scale: f32, even: T0, odd: T1) -> Self
    where
        T0: Texture + 'static,
        T1: Texture + 'static,
    {
        Self {
            inv_scale,
            even: Box::new(even) as _,
            odd: Box::new(odd) as _,
        }
    }

    #[allow(unused)]
    pub fn from_color(inv_scale: f32, even: Rgb<f32>, odd: Rgb<f32>) -> Self {
        Self::new(inv_scale, SolidColor::new(even), SolidColor::new(odd))
    }
}

impl Texture for CheckerTexture {
    fn color(&self, u: f32, v: f32, pos: Vec3) -> Rgb<f32> {
        let x = (self.inv_scale * pos.x).floor() as i32;
        let y = (self.inv_scale * pos.y).floor() as i32;
        let z = (self.inv_scale * pos.z).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.color(u, v, pos)
        } else {
            self.odd.color(u, v, pos)
        }
    }
}

pub struct ImageTexture {
    pub img: Rgb32FImage,
}

impl ImageTexture {
    pub fn new(img: Rgb32FImage) -> Self {
        Self { img }
    }
}

impl Texture for ImageTexture {
    fn color(&self, u: f32, v: f32, _pos: Vec3) -> Rgb<f32> {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let x = (u * self.img.width() as f32) as u32;
        let y = (v * self.img.height() as f32) as u32;

        let x = x.clamp(0, self.img.width() - 1);
        let y = y.clamp(0, self.img.height() - 1);
        *self.img.get_pixel(x, y)
    }
}
