pub struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point3d {
    pub fn new(x: f32, y: f32, z: f32) {
        Point3d { x, y, z };
    }

    pub fn length_squared(self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }
}

impl std::ops::Add for Point3d {
    type Output = Point3d;
    fn add(self, other: Point3d) -> Self::Output {
        Point3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl std::ops::Sub for Point3d {
    type Output = Point3d;
    fn sub(self, other: Point3d) -> Self::Output {
        Point3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl std::ops::Neg for Point3d {
    type Output = Point3d;
    fn neg(self) -> Self::Output {
        Point3d {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

