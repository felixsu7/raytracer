use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Default)]
pub struct Intersection {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl Intersection {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            outward_normal.inverse()
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, intersection: &mut Intersection) -> bool;
}
