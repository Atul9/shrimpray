use std::ops::{Index, Add, Mul, Div, Sub};

#[derive(Copy, Clone)]
struct Vec3 {
    e0: f32,
    e1: f32,
    e2: f32,
}

impl Index<u32> for Vec3 {
    type Output = f32;
    fn index(&self, i: u32) -> &Self::Output {
        match i {
            0 => &self.e0,
            1 => &self.e1,
            2 => &self.e2,
            _ => panic!(),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            e0: self.e0 + other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            e0: self.e0 - other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            e0: self.e0 * rhs,
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e0: rhs.e0 * self,
            e1: rhs.e1 * self,
            e2: rhs.e2 * self,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            e0: self.e0 / rhs,
            e1: self.e1 / rhs,
            e2: self.e2 / rhs,
        }
    }
}

impl Vec3 {
    fn mag(&self) -> f32 {
        dot(*self, *self).sqrt()
    }
    fn unit(&self) -> Vec3 {
        *self / self.mag()
    }
    fn new(e0: f32, e1: f32, e2: f32) -> Vec3{
        Vec3{e0: e0, e1: e1, e2: e2}
    }
    fn x(&self) -> f32 {
        self.e0
    }
    fn y(&self) -> f32 {
        self.e1
    }
    fn z(&self) -> f32 {
        self.e2
    }
}

fn dot(u: Vec3, v: Vec3) -> f32 {
    u.e0*v.e0 + u.e1*v.e1 + u.e2*v.e2
}

#[derive(Copy, Clone)]
struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    fn direction(&self) -> Vec3 {
        self.a + self.b
    }
    fn new(a: Vec3, b: Vec3) -> Ray {
        Ray {a: a, b: b}
    }
}

fn color(r: Ray) -> Vec3 {
    let unit_direction = r.direction().unit();
    let t: f32 = 0.5*(unit_direction.y() + 1.0);
    (1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let (nx, ny): (u32, u32) = (200, 100);
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let ll_corner = Vec3 {e0: -2.0, e1: -1.0, e2: -1.0};
    let horizontal = Vec3 {e0: 4.0, e1: 0.0, e2: 0.0};
    let vertical = Vec3 {e0: 0.0, e1: 2.0, e2: 0.0};
    let origin = Vec3 {e0: 0.0, e1: 0.0, e2: 0.0};

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;
            let r = Ray::new(origin, ll_corner + u*horizontal + v*vertical);
            let col = color(r);
            let ir: u32 = (255.99*col[0]) as u32;
            let ig: u32 = (255.99*col[1]) as u32;
            let ib: u32 = (255.99*col[2]) as u32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
