use crate::vec3::{Point3, Vec3};

pub struct Ray<'a> {
    pub origin: &'a Point3,
    pub direction: Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: &f64) -> Point3 {
        self.origin + &(t * &self.direction)
    }
}
