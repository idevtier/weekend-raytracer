mod ray;
mod vec3;

use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

fn main() {
    let aspect_ratio = 16.0f64 / 9.0f64;
    let image_width = 400u32;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        &origin - &(&horizontal / 2.0) - &vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        // eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let tmp = &u * &horizontal + &v * &vertical;
            let dir = &(&lower_left_corner + &tmp) - &origin;

            let ray = Ray::new(&origin, &dir);

            let color = ray_color(&ray);

            println!("{}", color);
        }
    }
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
