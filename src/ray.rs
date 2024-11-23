use crate::vector;

struct Ray {
    origin: vector::Vec3,
    dir: vector::Vec3
}

impl Ray {
    fn at(self, t: f32) -> vector::Vec3 {
        vector::add(self.origin, vector::mul_scalar(self.dir, t))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let origin = vector::Vec3::new(1.0, 1.0, 1.0);
        let dir = vector::Vec3::new(2.0, 2.0, 3.0);
        let ray = Ray { origin, dir };

        assert_eq!(ray.at(4.0), vector::Vec3::new(9.0, 9.0, 13.0));
    }
}