use crate::{ray::{Ray, RayHit}, material::Material};

pub trait Hittable {
    fn hit(&self, r: Ray, min:f64, max:f64) -> Option<RayHit>;
    fn get_material(&self) -> &dyn Material;
}