use crate::vec3;
use crate::{Hit, HitRecord, Point3, Ray};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64) -> Option<HitRecord> {
        let oc = r.origin - &self.center;
        let a = r.direction.len_squared();
        let half_b = vec3::dot(&oc, &r.direction);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if &root < t_min || t_max < &root {
            root = (-half_b + sqrtd) / a;
            if &root < t_min || t_max < &root {
                return None;
            }
        }

        let mut rec = HitRecord::default();
        rec.t = root.clone();
        rec.p = r.at(&rec.t);
        let outward_normal = (&rec.p - &self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}
