use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

type Point3 = Vec3; // 3D point
type Color = Vec3; // RGB color

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn initialize(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
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

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        // v / v.length();
        // Vec3{e: [v[0] / v[0], self.e[1] / _rhs.e[1], self.e[2] / _rhs.e[2]]}
        println!("v: {:?}", v.length());
        Vec3::new()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

// operator overloads
impl ops::Add<Vec3> for Vec3 {
type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3{e: [self.e[0] + _rhs.e[0], self.e[1] + _rhs.e[1], self.e[2] + _rhs.e[2]]}
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: f64) -> Vec3 {
        Vec3{e: [self.e[0] + _rhs, self.e[1] + _rhs, self.e[2] + _rhs]}
    }
}

impl ops::Sub<Vec3> for Vec3 {
type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3{e: [self.e[0] - _rhs.e[0], self.e[1] - _rhs.e[1], self.e[2] - _rhs.e[2]]}
    }
}

impl ops::Mul<Vec3> for Vec3 {
type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3{e: [self.e[0] * _rhs.e[0], self.e[1] * _rhs.e[1], self.e[2] * _rhs.e[2]]}
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3{e: [self.e[0] * _rhs, self.e[1] * _rhs, self.e[2] * _rhs]}
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3{e: [self * _rhs.e[0], self * _rhs.e[1], self * _rhs.e[2]]}
    }
}

impl ops::Div<Vec3> for Vec3 {
type Output = Vec3;

    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3{e: [self.e[0] / _rhs.e[0], self.e[1] / _rhs.e[1], self.e[2] / _rhs.e[2]]}
    }
}

// here's the documentation for overloading the index operator
// https://stackoverflow.com/questions/49593793/is-there-a-way-to-overload-the-index-assignment-operator
impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}