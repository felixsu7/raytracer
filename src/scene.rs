use crate::hittable::{Hittable, Intersection};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct Scene {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            hittables: Vec::<Box<dyn Hittable>>::new(),
        }
    }

    pub fn push(&mut self, obj: Box<dyn Hittable>) {
        self.hittables.push(obj);
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval, intersection: &mut Intersection) -> bool {
        let mut temp_record = Intersection::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for hittable in &self.hittables {
            if hittable.hit(
                ray,
                Interval::new(ray_t.min, closest_so_far),
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *intersection = temp_record.clone();
            }
        }

        hit_anything
    }
}
