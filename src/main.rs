use std::{fs::File};
use std::io::Write;
use bevy_math;
mod ray;
mod sphere;
pub mod hittable;
pub mod camera;
pub mod material;
pub mod rnggens;

type v3 = bevy_math::DVec3;

use std::time::Instant;
use crate::{sphere::Sphere, material::{Lambertian, Metalic, Dielectric}, rnggens::{NormRngGen, UniRngGen}};
use crate::{camera::Camera};

fn main() {
    let now = Instant::now();
    let cam = 
    Camera::new(
        v3::new(-2.0,2.0,1.0),
            v3::new(0.0,0.0,-1.0),
            v3::new(0.0,1.0, 0.0), 
            90.0, 16.0/9.0, 0.1, 30.0, 0.0, 0.0);

    let i_width:u64 = 1920;
    let i_height = ((i_width as f64) / cam.aspect()) as u64;

    let mut things:Vec<Sphere> = Vec::new();

    things.push(
        Sphere::new(
            v3::new
                (0.0, 0.5,-1.0), 
                1.0, 
                Box::new(
                        Lambertian::new(
                        v3::new(1.0, 1.0, 1.0),
                        ))));

    things.push(
        Sphere::new(
            v3::new
                (1.0, 0.5,-1.0), 
                1.0, 
                Box::new(
                        Metalic::new(
                        v3::new(1.0, 0.4, 1.0),
                        0.1
                        ))));

    things.push(
        Sphere::new(
            v3::new
                (-1.0, 0.5,-1.0), 
                -1.0, 
                Box::new(
                        Dielectric::new(
                        0.9,
                        v3::new(1.0, 1.0, 1.0),
                        ))));

    things.push(
        Sphere::new(
            v3::new
                (0.0, -100.5,-1.0), 
                100.0, 
                Box::new(
                        Lambertian::new(
                        v3::new(0.2, 0.2, 0.2)
                        ))));

    let samples_per_pixel:usize = 128;
    let mut uni_gen = UniRngGen::new();
    let mut norm_gen = NormRngGen::new();
    let mut buf:Vec<u8> = Vec::with_capacity((3*i_width*i_height) as usize); //vec![0; (3*i_width*i_height) as usize];
    for j in (0..i_height).rev() {
        for i in 0..i_width {
            let mut ray_color = v3::ZERO;
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + uni_gen.gen()) / ((i_width-1) as f64);
                let v = ((j as f64) + uni_gen.gen()) / ((i_height-1) as f64);
                let r = cam.make_ray_uv(u, v);
                ray_color += r.color( &mut norm_gen,&things, 50);
            }

            let rx = f64::sqrt(ray_color.x / (samples_per_pixel as f64));
            let gy = f64::sqrt(ray_color.y / (samples_per_pixel as f64));
            let rz = f64::sqrt(ray_color.z / (samples_per_pixel as f64));

            let ir = (256.0 * f64::clamp(rx, 0.0, 0.999)) as u8;
            let ig = (256.0 * f64::clamp(gy, 0.0, 0.999)) as u8;
            let ib = (256.0 * f64::clamp(rz, 0.0, 0.999)) as u8;
            buf.push(ir);
            buf.push(ig);
            buf.push(ib);
        }
    }

    let q = now.elapsed();
    let mut file = File::create("test_render.ppm").expect("Failed to open file.");
    let header = format!("P6 {} {} 255\n", i_width, i_height);
    file.write(header.as_bytes()).expect("Failed to write header.");
    file.write(&buf).expect("Failed to write buffer.");

    let m = now.elapsed();
    println!("Took: {:.2?} to render and {:.2?} to write.", q, m-q);
}
