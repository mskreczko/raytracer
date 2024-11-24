use crate::vector;
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Copy,Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn collide(&self, ray: &Ray) -> bool {
        let oc = vector::sub(self.center, ray.origin);
        let a = vector::dot(ray.dir, ray.dir);
        let b = -2.0 * vector::dot(oc, ray.dir);
        let c = vector::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant >= 0.0
    }
}