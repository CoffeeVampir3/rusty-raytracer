use crate::{ray::Ray, rnggens::NormRngGen};

type v3 = bevy_math::DVec3;

pub struct Camera {
    aspect: f64,
    viewport_height: f64,
    viewport_width: f64,

    origin: v3,
    horizontal: v3,
    vertical: v3,
    u:v3,
    v:v3,
    w:v3,
    lens_radius:f64,
    shutter_open:f64,
    shutter_close: f64,
    lower_left_corner: v3
}

impl Camera {
    pub fn new(
            lookfrom: v3,
            lookat: v3,
            up: v3,
            vfov: f64,
            aspect: f64, 
            aperature: f64, 
            focus_dist: f64,
            shutter_open: f64,
            shutter_close: f64) -> Self 
        {
            let theta = vfov * std::f64::consts::PI / 180.0;
            let h = f64::tan(theta / 2.0);
            let viewport_height = 2.0 * h;
            let viewport_width = viewport_height*aspect;

            let w = (lookfrom - lookat).normalize_or_zero();
            let u = v3::cross(up, w).normalize_or_zero();
            let v = v3::cross(w, u);

            let origin = lookfrom;


            let horizontal = viewport_width * u;
            let vertical = viewport_height * v;
            let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;
            let lens_radius = aperature / 2.0;
            Self { 
                aspect, 
                origin,
                u,
                v,
                w,
                viewport_height, 
                viewport_width,
                horizontal,
                vertical,
                lower_left_corner,
                shutter_open,
                shutter_close,
                lens_radius
            } 
        }

    pub fn make_ray_uv(&self, s:f64, t:f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin)
    }    

    /// Get the camera's viewport height.
    pub fn viewport_height(&self) -> f64 {
        self.viewport_height
    }

    /// Get the camera's viewport width.
    pub fn viewport_width(&self) -> f64 {
        self.viewport_width
    }

    /// Get the camera's aspect.
    pub fn aspect(&self) -> f64 {
        self.aspect
    }
}