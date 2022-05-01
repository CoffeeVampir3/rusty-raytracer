use crate::{hittable::Hittable, ray::{RayHit, Ray}, material::{Material}};

type v3 = bevy_math::DVec3;

pub struct Sphere {
    origin:v3,
    radius:f64,
    material: Box<dyn Material>
}

impl Sphere {
    pub fn new(origin: v3, radius: f64, material: Box<dyn Material>) -> Self { Self { origin, radius, material } }

    /// Get the sphere's origin.
    pub fn origin(&self) -> v3 {
        self.origin
    }

    /// Get the sphere's radius.
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, min:f64, max:f64) -> Option<RayHit> {
        //t^2b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−R^2=0
    
        //(A-C)
        let oc = r.origin() - self.origin();
        //b * b
        let a = r.dir().length_squared();
        //2tb
        let half_b = v3::dot(oc, r.dir());
        //(A-C)*(A-C)-R^2
        let c = oc.length_squared() - self.radius()*self.radius();
        //We discriminate based on the roots of t^2, if discriminant >0 there's at least one real solution
        let discriminant = half_b*half_b - a*c;

        if discriminant < 0.0 {
            return None
        }

        let sqrtd = f64::sqrt(discriminant);
        let root = (-half_b - sqrtd) / a;
        if root >= min && root <= max {
            let hitp = r.at(root);
            let normal = (hitp - self.origin) / self.radius;
            return Some(RayHit::new(hitp, normal, r, root, self))
        }
        let root = (-half_b + sqrtd) / a;
        if root >= min && root <= max {
            let hitp = r.at(root);
            let normal = (hitp - self.origin) / self.radius;
            return Some(RayHit::new(hitp, normal, r, root, self))
        }

        None
    }

    fn get_material(&self) -> &dyn Material {
        self.material.as_ref()
    }
}