use crate::ray::{FVec3, HitRecord, Hitable, Ray};
use nalgebra_glm as glm;

/// A spherical object based from the simple definition of
/// center and radius.
///
/// As a reminder, the sphere equation at the origin of radius R is x*x + y*y + z*z = R*R.
/// Also, the sphere equation in vector form is dot((p - c), (p - c)) = R*R.
pub struct Sphere {
    center: FVec3,
    radius: f32,
}

impl Sphere {
    /// # Example
    ///
    /// ```no_run
    /// use crate::sphere::Sphere;
    ///
    /// let sphere = Sphere::new(glm::vec3(0., 0., 0.), 0.5);
    /// ```
    pub fn new(center: FVec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord,
    ) -> bool {
        let oc = r.get_origin() - self.center;
        let a = glm::dot(&r.get_direction(), &r.get_direction());
        let b = 2. * glm::dot(&oc, &r.get_direction());
        let c = glm::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4. * a * c;

        if discriminant > 0. {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.point_at_parameter(hit_record.t);
                hit_record.normal = (hit_record.p - self.center) / self.radius;
                return true;
            }
        }

        return false;
    }
}
