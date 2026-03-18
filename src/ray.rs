use glam::Vec3;

pub struct Ray {
    pos: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(pos: Vec3, dir: Vec3) -> Self {
        Self { pos, dir }
    }

    pub fn postion(&self) -> Vec3 {
        self.pos
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.pos + self.dir * t
    }
}
