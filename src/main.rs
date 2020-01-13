use std::rc::Rc;

use image::GenericImageView;

mod vec3;
use vec3::{Ray, Vec3};

mod camera;
use camera::Camera;

mod obj;
use obj::*;

mod hit;
use hit::*;

mod bvh;
use bvh::BvhNode;

mod material;
use material::*;

mod texture;
use texture::*;

mod util;
use util::*;

mod perlin;
use perlin::Perlin;

fn color(r: Ray, world: &BvhNode, depth: u32) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        let emitted = hit.material.emitted(hit.u, hit.v, &hit.p);
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit.material.scatter(r, &hit) {
                return emitted + attenuation * color(scattered, world, depth + 1);
            } else {
                return emitted;
            }
        } else {
            return emitted;
        }
    }
    Vec3::new(0.0, 0.0, 0.0)
}

fn regular_scene() -> Vec<Rc<dyn Hittable>> {
    let world: Vec<Rc<dyn Hittable>> = vec![
        Rc::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
                0.1, 0.2, 0.5,
            ))))),
        )),
        Rc::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
                0.8, 0.8, 0.0,
            ))))),
        )),
        Rc::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
        )),
        Rc::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Rc::new(Dielectric::new(1.5)),
        )),
        Rc::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Rc::new(Dielectric::new(1.5)),
        )),
    ];
    world
}

fn random_scene() -> Vec<Rc<dyn Hittable>> {
    let mut scene: Vec<Rc<dyn Hittable>> = Vec::new();
    let checker = Rc::new(CheckerTexture::new(
        Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
    ));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, -1.0),
        1000.0,
        Rc::new(Lambertian::new(checker)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_float();
            let center = Vec3::new(
                a as f32 + 0.9 * rand_float(),
                0.2,
                b as f32 + 0.9 * rand_float(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).mag() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    scene.push(Rc::new(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, 0.5 * rand_float(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
                            rand_float() * rand_float(),
                            rand_float() * rand_float(),
                            rand_float() * rand_float(),
                        ))))),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    scene.push(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rand_float()),
                                0.5 * (1.0 + rand_float()),
                                0.5 * (1.0 + rand_float()),
                            ),
                            0.5 * rand_float(),
                        )),
                    )));
                } else {
                    // glass
                    scene.push(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
            0.4, 0.2, 0.1,
        ))))),
    )));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));
    scene
}

fn two_spheres_scene() -> Vec<Rc<dyn Hittable>> {
    let mut scene: Vec<Rc<dyn Hittable>> = Vec::new();
    let checker = Rc::new(CheckerTexture::new(
        Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
    ));
    let checker2 = Rc::new(CheckerTexture::new(
        Box::new(ConstantTexture::new(Vec3::new(0.1, 0.2, 0.3))),
        Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
    ));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new(checker)),
    )));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new(checker2)),
    )));
    scene
}

fn two_perlin_spheres_scene() -> Vec<Rc<dyn Hittable>> {
    let mut scene: Vec<Rc<dyn Hittable>> = Vec::new();
    let perlin_texture = Rc::new(NoiseTexture::new(4.0, Perlin::new()));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(perlin_texture.clone())),
    )));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(perlin_texture)),
    )));
    scene
}

fn earth_scene() -> Vec<Rc<dyn Hittable>> {
    let mut scene: Vec<Rc<dyn Hittable>> = Vec::new();
    let perlin_texture = Rc::new(NoiseTexture::new(4.0, Perlin::new()));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(perlin_texture)),
    )));

    let img = image::open("texture/earthmap.jpg").unwrap();
    let (nx, ny) = img.dimensions();
    let data = img.raw_pixels();
    let image_texture = Rc::new(ImageTexture::new(data, nx as i32, ny as i32));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(image_texture)),
    )));
    scene
}

fn simple_light() -> Vec<Rc<dyn Hittable>> {
    let perlin_texture = Rc::new(NoiseTexture::new(4.0, Perlin::new()));
    let mut scene: Vec<Rc<dyn Hittable>> = Vec::new();
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(perlin_texture.clone())),
    )));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(perlin_texture)),
    )));

    let constant_texture = Rc::new(ConstantTexture::new(Vec3::new(4.0, 4.0, 4.0)));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        Rc::new(DiffuseLight::new(constant_texture.clone())),
    )));
    scene.push(Rc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Rc::new(DiffuseLight::new(constant_texture.clone())),
    )));
    scene
}

fn cornell_box() -> Vec<Rc<dyn Hittable>> {
    let mut scene: Vec<Rc<dyn Hittable>> = Vec::new();

    let red = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)))));
    let white = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)))));
    let green = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)))));
    let light = Rc::new(DiffuseLight::new(Rc::new(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0)))));

    scene.push(Rc::new(FlipNormals::new(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)))));
    scene.push(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    scene.push(Rc::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    scene.push(Rc::new(FlipNormals::new(Rc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())))));
    scene.push(Rc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    scene.push(Rc::new(FlipNormals::new(Rc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)))));

    scene
}

fn main() {
    let (nx, ny, ns) = (500, 300, 500);
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        vfov,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let world = BvhNode::new(&mut cornell_box(), 0.0, 1.0);

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
