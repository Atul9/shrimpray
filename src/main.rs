use std::sync::Arc;

use rayon::prelude::*;

mod vec3;
use vec3::{Ray, Vec3};

mod camera;
use camera::Camera;

mod obj;

mod hit;
use hit::*;

mod bvh;

mod material;

mod texture;

mod util;
use util::*;

mod perlin;

mod transf;

mod scene;
use scene::*;

fn color(r: Ray, world: &Vec<Arc<dyn Hittable>>, depth: u32) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        let emitted = hit.material.emitted(hit.u, hit.v, &hit.p);
        if depth < 50 {
            if let Some(s_rec) = hit.material.scatter(r, &hit) {
                return emitted
                    + s_rec.albedo
                        * hit.material.scattering_pdf(&r, &hit, &s_rec.scattering)
                        * color(s_rec.scattering, world, depth + 1)
                        / s_rec.pdf;
            } else {
                return emitted;
            }
        } else {
            return emitted;
        }
    }
    Vec3::new(0.0, 0.0, 0.0)
}

fn main() {
    let (nx, ny, ns) = (300, 300, 500);
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let (cam, world) = cornell_mc(ny as f32 / nx as f32);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col: Vec3 = (0..ns)
                .into_par_iter()
                .map(|_| {
                    let u: f32 = (i as f32 + rand_float()) / nx as f32;
                    let v: f32 = (j as f32 + rand_float()) / ny as f32;
                    let r = cam.get_ray(u, v);
                    color(r, &world, 0)
                })
                .sum();
            col /= ns as f32;
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

            let ir: u32 = (255.99 * col[0]) as u32;
            let ig: u32 = (255.99 * col[1]) as u32;
            let ib: u32 = (255.99 * col[2]) as u32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::*;
    use crate::vec3::*;

    #[inline]
    fn pdf(p: Vec3) -> f32 {
        return 1.0 / (4.0 * std::f32::consts::PI);
    }

    #[test]
    fn mc() {
        let n = 1000000;
        let mut sum = 0.0;
        for i in 0..n {
            let d = random_unit_vector();
            let cos_sqr = d.z() * d.z();
            sum += cos_sqr / pdf(d);
        }
        println!("I = {}", sum / n as f32);
    }
}
