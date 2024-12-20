use std::ops;

#[derive(Debug,Copy,Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn add(&mut self, other: Vec3) -> &Vec3 {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self
    }

    pub fn multiply(&mut self, t: f32) -> &Vec3 {
        self.x *= t;
        self.y *= t;
        self.z *= t;
        self
    }

    pub fn divide(&mut self, t: f32) -> &Vec3 {
        self.multiply((1f32) / t)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3{x: v1.y * v2.z - v1.z * v2.y, y: v1.z * v2.x - v1.x * v2.z, z: v1.x * v2.y - v1.y * v2.x}
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_add() {
        let mut v1 = Vec3::new(1f32, 2f32, 3f32);
        let v2 = Vec3::new(4f32, 5f32, 6f32);

        v1.add(v2);
        assert_eq!(5f32, v1.x);
        assert_eq!(7f32, v1.y);
        assert_eq!(9f32, v1.z);
    }

    #[test]
    fn test_self_multiply() {
        let mut v1 = Vec3::new(1f32, 2f32, 3f32);

        v1.multiply(3f32);
        assert_eq!(3f32, v1.x);
        assert_eq!(6f32, v1.y);
        assert_eq!(9f32, v1.z);
    }

    #[test]
    fn test_self_divide() {
        let mut v1 = Vec3::new(9f32, 18f32, 27f32);

        v1.divide(3f32);
        assert_eq!(3f32, v1.x);
        assert_eq!(6f32, v1.y);
        assert_eq!(9f32, v1.z);
    }

    #[test]
    fn test_length() {
        let v1 = Vec3::new(1f32, 2f32, 3f32);

        let result = v1.length();
        assert_eq!(14f32.sqrt(), result);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1f32, 2f32, 3f32);
        let v2 = Vec3::new(4f32, 5f32, 6f32);

        let result = v1 + v2;
        assert_eq!(5f32, result.x);
        assert_eq!(7f32, result.y);
        assert_eq!(9f32, result.z);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1f32, 2f32, 3f32);
        let v2 = Vec3::new(4f32, 5f32, 6f32);

        let result = v1 - v2;
        assert_eq!(-3f32, result.x);
        assert_eq!(-3f32, result.y);
        assert_eq!(-3f32, result.z);
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::new(1f32, 2f32, 3f32);
        let v2 = Vec3::new(4f32, 5f32, 6f32);

        let result = v1 * v2;
        assert_eq!(4f32, result.x);
        assert_eq!(10f32, result.y);
        assert_eq!(18f32, result.z);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1f32, 2f32, 3f32);
        let v2 = Vec3::new(4f32, 5f32, 6f32);

        let result = dot(v1, v2);
        assert_eq!(32f32, result);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1f32, 2f32, 3f32);
        let v2 = Vec3::new(4f32, 5f32, 6f32);

        let result = cross(v1, v2);
        assert_eq!(-3f32, result.x);
        assert_eq!(6f32, result.y);
        assert_eq!(-3f32, result.z);
    }
}