use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};
#[derive(Default, Clone, Copy, Debug)]

pub struct Vec3 {
    pub e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    // initialize a new vec3 
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self {
            e: [e0, e1, e2]
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0]
            + self.e[1] * self.e[1]
            + self.e[2] * self.e[2]
    }


    pub fn write_color<W: std::io::Write>(self, writer: &mut W) -> std::io::Result<()> {
        let ir = (255.99 * self.x()) as u32;
        let ig = (255.99 * self.y()) as u32;
        let ib = (255.99 * self.z()) as u32;

        writeln!(writer, "{ir} {ig} {ib}")
    }

    pub fn unit(v: &Vec3) -> Vec3 {
        //normalizing the vector 
        *v / v.length()
    }

}

// -v 
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(
            self.e[0] * t,
            self.e[1] * t,
            self.e[2] * t,
        )
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3)->Vec3{
        v * self
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self,t:f64){
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t:f64) -> Vec3{
        self * (1.0/t)
    }
}

// a / b = a × (1/b)
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64){
        // self -> &mut vect3 -> *self -> Vec3
        *self *= 1.0/t;
    }

}


pub fn dot(u: Vec3, v: Vec3) -> f64{
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}



