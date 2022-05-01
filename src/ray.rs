use crate::{hittable::Hittable, rnggens::{NormRngGen}};

type v3 = bevy_math::DVec3;
pub struct RayHit<'a> {
    point: v3,
    normal: v3,
    t: f64,
    front_face: bool,
    hitthing: &'a dyn Hittable
}

impl<'a> RayHit<'a> {
    pub fn new(point: v3, normal: v3, r: Ray, t: f64, hitthing: &'a dyn Hittable) -> 
    Self 
    { 
        let facing = v3::dot(r.norm_dir(), normal) < 0.0;
        let faced_normal = if facing { normal } else { -normal };
        Self { 
            point, 
            normal:faced_normal, 
            t, 
            front_face:facing, 
            hitthing } 
    }

    /// Get the ray hit's normal.
    pub fn normal(&self) -> v3 {
        self.normal
    }

    /// Get the ray hit's point.
    pub fn point(&self) -> v3 {
        self.point
    }

    /// Get the ray hit's hitthing.
    pub fn hitthing(&self) -> &dyn Hittable {
        self.hitthing
    }

    /// Get the ray hit's front face.
    #[must_use]
    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: v3,
    dir: v3,
    norm_dir: v3
}

impl Ray {
    pub fn new(o:v3, d:v3) -> Ray {
        Ray {origin:o, dir:d, norm_dir: d.normalize_or_zero()}
    }

    pub fn at(&self, t:f64) -> v3 {
        self.origin + (t*self.dir)
    }

    pub fn origin(&self) -> v3 {
        self.origin
    }

    pub fn dir(&self) -> v3 {
        self.dir
    }

    /// Get the ray's norm dir.
    pub fn norm_dir(&self) -> v3 {
        self.norm_dir
    }

    pub fn color<H: Hittable>(&self, norm_rng: &mut NormRngGen, hittables:&Vec<H>, depth:u64) -> v3 {
        if depth == 0 {
            return v3::ZERO
        }
        for item in hittables {
            let hit = item.hit(*self, 0.001, f64::INFINITY);
            match hit {
                Some(rayhit) => {
                    let mat = rayhit.hitthing.get_material();
                    let scattered = mat.scatter(norm_rng, *self, &rayhit);
                    match scattered {
                        Some((scattering_ray, albedo)) => return albedo * scattering_ray.color(norm_rng, hittables, depth-1),
                        None => ()
                    }
                }
                None => ()
            };
        }
        let ud = self.norm_dir;
        let t = 0.5*(ud.y+1.0);
        return (1.0-t)*v3::ONE + t*v3::new(0.5, 0.7, 1.0);
    }
}