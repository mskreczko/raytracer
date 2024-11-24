use crate::vector;
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Copy,Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn collide(&self, ray: &Ray) -> f32 {
        let oc = self.center - ray.origin;
        let a = vector::dot(ray.dir, ray.dir);
        let b = -2.0 * vector::dot(oc, ray.dir);
        let c = vector::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if (discriminant < 0.0) {
            return -1.0;
        }
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}