use std::ops::{Add, Sub, Mul};

/// Represents a Vector3 of f32 components
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3f {

    /// Creates a new vector3
    ///
    /// # Arguments
    ///
    /// * `x` - X component of the vector3
    /// * `y` - Y component of the vector3
    /// * `z` - Z component of the vector3
    ///
    /// # Example
    ///
    /// ```
    /// let vector3: Vector3 = Vector3::new(5.0, 2.25, 1.0);
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x, y, z
        }
    }

    /// Does the same as `Vector3f::new()` but with zeroes
    ///
    /// # Example
    ///
    /// ```
    /// let vector: Vector3f = Vector3f::zero();
    /// assert_eq!(vector.get_x(), 0);
    /// assert_eq!(vector.get_y(), 0);
    /// assert_eq!(vector.get_z(), 0);
    /// ```
    pub fn zero() -> Self {
        Vector3f::new(0.0, 0.0, 0.0)
    }

    /// Computes the squared length of the vector3
    pub fn squared_len(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    /// Computes the length of the vector3
    pub fn len(&self) -> f32 {
        self.squared_len().sqrt()
    }

    /// Compute a new normalized vector3 from an other vector3
    pub fn normalize(&self) -> Self {
        let len = self.len();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len
        }
    }

    /// Mutates a vector3 by normalizing it
    pub fn normalize_mut(&mut self) {
        *self = self.normalize();
    }

    /// Computes the dot product of two vectors
    ///
    /// # Arguments
    ///
    /// * `a` - First Vector3f
    /// * `b` - Second Vector3f
    ///
    /// # Example
    ///
    /// ```
    /// let v1: Vector3f = Vector3f::new(15.0, 2.0, -6.0);
    /// let v2: Vector3f = Vector3f::new(16.0, 1.0, 0.0);
    /// let result: f32 = Vector3f::dot(&v1, &v2);
    /// ```
    pub fn dot(a: &Vector3f, b: &Vector3f) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross_product(left: &Vector3f, right: &Vector3f) -> Vector3f {
        Vector3f {
            x: left.y * right.z - left.z * right.y,
            y: left.x * right.z - left.z * right.x,
            z: left.x * right.y - left.y * right.x
        }
    }
}

impl Add<Vector3f> for Vector3f {
    type Output = Vector3f;

    fn add(self, rhs: Vector3f) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub<Vector3f> for Vector3f {
    type Output = Vector3f;

    fn sub(self, rhs: Vector3f) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Mul<f32> for Vector3f {
    type Output = Vector3f;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vector3f;

    #[test]
    fn test_new() {
        let vec3: Vector3f = Vector3f::new(1.0, 2.0, 3.0);
        assert_eq!(vec3.x, 1.0);
        assert_eq!(vec3.y, 2.0);
        assert_eq!(vec3.z, 3.0);
    }

    #[test]
    fn test_zero() {
        let zero: Vector3f = Vector3f::zero();
        assert_eq!(zero.x, 0.0);
        assert_eq!(zero.y, 0.0);
        assert_eq!(zero.z, 0.0);
    }

    #[test]
    fn test_squared_len() {
        let mut vector: Vector3f = Vector3f::new(1.0, 1.0, 1.0);
        assert_eq!(vector.squared_len(), 3.0);

        vector = Vector3f::new(0.0, 0.0, 0.0);
        assert_eq!(vector.squared_len(), 0.0);

        vector = Vector3f::new(0.0, 1.0, 0.0);
        assert_eq!(vector.squared_len(), 1.0);

        vector = Vector3f::new(-1.0, -1.0, -1.0);
        assert_eq!(vector.squared_len(), 3.0);
    }

    #[test]
    fn test_len() {
        let mut vector: Vector3f = Vector3f::new(1.0, 1.0, 1.0);
        assert_eq!(vector.len(), f32::sqrt(3.0));

        vector = Vector3f::zero();
        assert_eq!(vector.len(), 0.0);
    }

    #[test]
    fn test_normalize() {
        let vec: Vector3f = Vector3f::new(5.0, 5.0, 5.0);
        let vec2: Vector3f = Vector3f::new(18.0, 18.0, 18.0);
        assert_eq!(vec.normalize(), vec2.normalize());
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vector3f::new(1.0, 2.0, 3.0);
        let v2 = Vector3f::new(1.0, 5.0, 7.0);
        assert_eq!(32.0, Vector3f::dot(&v1, &v2));
    }
}
