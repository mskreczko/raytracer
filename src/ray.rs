use crate::vector;
use crate::vector::Vec3;
use crate::sphere::Sphere;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    fn at(self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }

    pub fn color(self, sphere: Sphere) -> Vec3 {
        if sphere.collide(&self) {
            return Vec3::new(1.0, 0.0, 0.0)
        }
        let unit_direction = vector::unit_vector(self.dir);
        let a = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) +
            Vec3::new(0.5, 0.7, 1.0) * a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let origin = Vec3::new(1.0, 1.0, 1.0);
        let dir = Vec3::new(2.0, 2.0, 3.0);
        let ray = Ray { origin, dir };

        assert_eq!(ray.at(4.0), Vec3::new(9.0, 9.0, 13.0));
    }
}