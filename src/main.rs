use std::rc::Rc;

mod vec3;
use vec3::{Ray, Vec3};

mod camera;
use camera::Camera;

mod obj;
use obj::Sphere;

mod hit;
use hit::Hittable;

mod material;
use material::{Dielectric, Lambertian, Metal};

mod util;
use util::*;

fn color(r: Ray, world: &Vec<Box<dyn Hittable>>, depth: u32) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit.material.scatter(r, &hit) {
                return attenuation * color(scattered, world, depth + 1);
            } else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction = r.direction().unit();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}

fn main() {
    let (nx, ny, ns) = (200, 100, 100);
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (lookfrom - lookat).mag();
    let aperture = 2.0;

    let cam = Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 20.0,
                          nx as f32 / ny as f32, aperture, dist_to_focus);

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))))),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))))),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)))),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Dielectric::new(1.5)))),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Rc::new(Dielectric::new(1.5)))),
    ];

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let u: f32 = (i as f32 + rand_float()) / nx as f32;
                let v: f32 = (j as f32 + rand_float()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(r, &world, 0);
            }
            col /= ns as f32;
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

            let ir: u32 = (255.99 * col[0]) as u32;
            let ig: u32 = (255.99 * col[1]) as u32;
            let ib: u32 = (255.99 * col[2]) as u32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
