use crate::point_type::Point3d;

pub struct Ray {
    pub origin: Point3d,
    pub direction: Point3d
}

impl Ray {
    pub fn at(self, t: f32) -> Point3d {
        self.origin + self.direction.scalar_multiply(t)
    }
}