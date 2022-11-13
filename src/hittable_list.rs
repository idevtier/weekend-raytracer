use crate::{Hit, HitRecord, Ray};
use std::rc::Rc;

#[derive(Default)]
pub struct HitList {
    objects: Vec<Rc<dyn Hit>>,
}

impl HitList {
    pub fn new(objects: Vec<Rc<dyn Hit>>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn push(&mut self, hittable: Rc<dyn Hit>) {
        self.objects.push(hittable);
    }
}

impl Hit for HitList {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64) -> Option<HitRecord> {
        let mut res: Option<HitRecord> = None;

        for object in self.objects.iter() {
            let mut closest_so_far = t_max;
            if let Some(r) = &res {
                closest_so_far = &r.t;
            }

            let buf = object.hit(r, t_min, closest_so_far);

            drop(closest_so_far);

            if buf.is_some() {
                res = buf;
            }
        }

        res
    }
}
