use nalgebra_glm as glm;

/// Representation of a 3D float vector, based on
/// nalgebra-glm extern crate.
pub type FVec3 = glm::TVec3<f32>;

/// A ray has a direction and a origin point, both as
/// a 3d vector.
pub struct Ray {
    origin: FVec3,
    direction: FVec3,
}

impl Ray {
    pub fn new(origin: FVec3, direction: FVec3) -> Self {
        Self { origin, direction }
    }

    /// Get the origin of the ray.
    pub fn get_origin(&self) -> FVec3 {
        self.origin
    }

    /// Return the direction of the ray.
    pub fn get_direction(&self) -> FVec3 {
        self.direction
    }

    pub fn point_at_parameter(&self, t: f32) -> FVec3 {
        // Equation: a + t*b
        self.origin + (self.direction * t)
    }
}

pub struct HitRecord {
    pub t: f32,
    pub p: FVec3,
    pub normal: FVec3,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            t: 0.,
            p: glm::vec3(0., 0., 0.),
            normal: glm::vec3(0., 0., 0.),
        }
    }
}

/// Each object that implement this trait could be visible in the
/// scene.
pub trait Hitable {
    // We're going to set a color if the current object is hit by a ray.
    fn hit(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord,
    ) -> bool;
}
