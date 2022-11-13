extern crate rand;

mod camera;
mod hit;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use crate::camera::*;
use crate::hit::*;
use crate::hittable_list::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vec3::{Color, Point3, Vec3};

use std::f64::{consts::PI, INFINITY};
use std::rc::Rc;

fn main() {
    let aspect_ratio = 16.0f64 / 9.0f64;
    let image_width = 1200u32;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let max_depth = 50;

    let mut world = HitList::default();
    world.push(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::default();

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::default();

            for _ in 0..100 {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;

                let r = camera.get_ray(&u, &v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            println!("{}", pixel_color);
        }
    }

    eprintln!("Done");
}

fn ray_color(ray: &Ray, world: &HitList, depth: i32) -> Color {
    let hit = world.hit(ray, &0.0, &INFINITY);

    if depth <= 0 {
        return Color::default();
    }

    if let Some(hit) = hit {
        let target = &hit.p + &hit.normal + Vec3::random_unit_vector();
        return 0.5 * ray_color(&Ray::new(&hit.p, &target - &hit.p), world, depth - 1);
    }

    let unit_direction = vec3::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.len_squared();
    let half_b = vec3::dot(&oc, &ray.direction);
    let c = oc.len_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

#[inline]
fn degrees_to_radius(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
fn random_double() -> f64 {
    rand::random::<f64>()
}
#[inline]
fn random_double_in_range(min: &f64, max: &f64) -> f64 {
    min + (max - min) * random_double()
}
