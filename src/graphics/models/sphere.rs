use crate::{graphics::materials::Material, math::Point};

use crate::graphics::{HitRecord, Hittable, Ray};
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        // Quadratic formula
        let b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = b.powi(2) - c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut root = -b - sqrt_discriminant;

        if root < t_min || t_max < root {
            root = -b + sqrt_discriminant;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let intersection_point = ray.at(root);
        let normal = (intersection_point - self.center) / self.radius;

        Some(HitRecord::new(
            intersection_point,
            normal,
            root,
            ray,
            &*self.material,
        ))
    }
}
