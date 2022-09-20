use crate::libs::Vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl ray {
    pub fn new() -> ray {
        ray {
            origin: Vec3::new(),
            direction: Vec3::new(),
        }
    }

    pub fn initialize(origin: Vec3, direction: Vec3) -> ray {

        ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {

        self.origin
    }

    pub fn direction(&self) -> Vec3 {

        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}


pub fn hello() {
    println!("Hello from the libs module!");
}