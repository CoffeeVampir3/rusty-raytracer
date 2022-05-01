use rand::{thread_rng, Rng};
use rand_distr::Uniform;

use crate::{ray::{Ray, RayHit}, rnggens::NormRngGen};

type v3 = bevy_math::DVec3;

pub trait Material {
    fn scatter(&self, rng: &mut NormRngGen, r: Ray, hit: &RayHit) -> Option<(Ray, v3)>;
}

pub struct Lambertian {
    albedo: v3
}

impl Lambertian {
    pub fn new(albedo: v3) -> Self { Self { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, rng: &mut NormRngGen, _r: Ray, hit: &RayHit) -> Option<(Ray, v3)> {
        let mut scattering_dir = hit.normal() + rng.random_in_unit_sphere_from_norm();

        //Degenerate scattering case, checks if the scattering dir is nearly zero.
        if scattering_dir.abs_diff_eq(v3::ZERO, 0.00005) {
            scattering_dir = hit.normal() + 0.00005;
        }
        let scattered_ray = Ray::new(hit.point(), scattering_dir);
        Some((scattered_ray, self.albedo))
    }
}

pub struct Metalic {
    albedo: v3,
    fuzziness: f64,
}

impl Metalic {
    pub fn new(albedo: v3, fuzziness: f64) -> Self { Self { albedo, fuzziness:f64::max(fuzziness, 1.0) } }
}

impl Material for Metalic {
    fn scatter(&self, rng: &mut NormRngGen, r: Ray, hit: &RayHit) -> Option<(Ray, v3)> {
        let reflected = reflect_vector(r.norm_dir(), hit.normal());
        let scattered = Ray::new(hit.point(), reflected + self.fuzziness * rng.random_in_unit_sphere_from_norm());
        if v3::dot(scattered.dir(), hit.normal()) > 0.0 {
            return Some((scattered, self.albedo))
        }
        None
    }
}

fn reflect_vector(v:v3, n:v3) -> v3 {
    v - (2.0*v3::dot(v, n)*n)
}

fn refract_vector (v:v3, n:v3, cos_theta:f64, etai_etat: f64) -> v3 {
    let rprime_perp = etai_etat * (v + cos_theta*n);
    let rprime_parallel = -f64::sqrt(f64::abs(1.0 - rprime_perp.length_squared())) * n;
    rprime_perp + rprime_parallel
}

fn approx_reflectance(cos_theta:f64, index_of_refraction:f64) -> f64 {
    let r0 = (1.0-index_of_refraction) / (1.0+index_of_refraction);
    let r0 = r0*r0;
    r0 + (1.0-r0)*f64::powf(1.0 - cos_theta, 5.0)
}

pub struct Dielectric {
    index_of_refraction:f64,
    attenuation:v3
}

impl Dielectric {
    pub fn new(index_of_refraction: f64, attenuation: v3) -> Self { Self { index_of_refraction, attenuation } }
}

impl Material for Dielectric {
    fn scatter(&self, rng: &mut NormRngGen, r: Ray, hit: &RayHit) -> Option<(Ray, v3)> {
        let refraction_ratio = if hit.front_face() { 1.0/self.index_of_refraction } else { self.index_of_refraction };

        let cos_theta = f64::min(v3::dot(-r.norm_dir(), hit.normal()), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let uni: Uniform<f64> = Uniform::new_inclusive(0.0, 1.0);
        if sin_theta * refraction_ratio > 1.0 || approx_reflectance(cos_theta, self.index_of_refraction) > thread_rng().sample(uni) {
            //Reflection
            let reflection = reflect_vector(r.norm_dir(), hit.normal());
            return Some((Ray::new(hit.point(), reflection), self.attenuation))
        }

        let refracted = refract_vector(r.norm_dir(), hit.normal(), cos_theta, refraction_ratio);
        Some((Ray::new(hit.point(), refracted), self.attenuation))
    }
}

