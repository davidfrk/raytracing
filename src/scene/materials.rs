#![allow(dead_code, unused_variables)]
extern crate nalgebra as na;
use na::Vector3;

use rand::Rng;
use crate::intersection::HitData;
use crate::intersection::Ray;

#[derive(Copy, Clone)]
pub enum Material {
        Emission(Emission),
        Diffuse(Diffuse),
        Metal(Metal),
        Glass(Glass),
        Portal(Portal),
}

#[derive(Copy, Clone)]
pub struct Emission{
        pub color:Vector3<f64>,
        pub emission:Vector3<f64>,
}

#[derive(Copy, Clone)]
pub struct Diffuse{
        pub color:Vector3<f64>,
}

#[derive(Copy, Clone)]
pub struct Metal{
        pub color:Vector3<f64>,
        pub fuzz:f64,
}

#[derive(Copy, Clone)]
pub struct Glass{
        pub color:Vector3<f64>,
        pub refraction:f64,
}

#[derive(Copy, Clone)]
pub struct Portal{
        pub color:Vector3<f64>,
        pub position:Vector3<f64>,
        pub target:Vector3<f64>,
}

impl Material{
        pub fn attenuation(&self) -> Vector3<f64>{
                match *self{
                        Material::Emission(ref m) => {
                                return m.color; //Vector3::new(0.0, 0.0, 0.0);
                        },
                        Material::Diffuse(ref m) => {
                                return m.color;
                        },
                        Material::Metal(ref m) => {
                                return m.color;
                        },
                        Material::Glass(ref m) => {
                                return m.color;
                        },
                        Material::Portal(ref m) => {
                                return m.color;
                        },
                }
        }

        pub fn scatter(&self, dir_in:&Vector3<f64>, hit_data:&HitData, out:&mut Ray) -> bool{
                match *self{
                        Material::Emission(ref m) => {
                                return m.scatter(dir_in, hit_data, out);
                        },
                        Material::Diffuse(ref m) => {
                                return m.scatter(dir_in, hit_data, out);
                        },
                        Material::Metal(ref m) => {
                                return m.scatter(dir_in, hit_data, out);
                        },
                        Material::Glass(ref m) => {
                                return m.scatter(dir_in, hit_data, out);
                        },
                        Material::Portal(ref m) => {
                                return m.scatter(dir_in, hit_data, out);
                        },
                }
        }
}

pub trait Scatterable{
        fn attenuation(&self) -> Vector3<f64>;
        fn scatter(&self, dir_in:&Vector3<f64>, hit_data:&HitData, ray:&mut Ray) -> bool;
}

impl Emission{
        pub fn create(color:Vector3<f64>, emission: Vector3<f64>) -> Material{
                Material::Emission(Emission{
                        color,
                        emission,
                })
        }

        pub fn scatter(&self, dir_in:&Vector3<f64>, hit_data:&HitData, out:&mut Ray) -> bool{
                return scatter_diffuse(dir_in, hit_data, &mut out.direction);
        }
}

impl Scatterable for Emission{
        fn attenuation(&self) -> Vector3<f64>{
                return self.color;
        }

        fn scatter(&self, dir_in:&Vector3<f64>, hit_data:&HitData, out:&mut Ray) -> bool{
                return scatter_diffuse(dir_in, hit_data, &mut out.direction);
        }
}

impl Diffuse{
        pub fn create(color:Vector3<f64>) -> Material{
                Material::Diffuse(Diffuse{
                        color,
                })
        }

        pub fn scatter(&self, dir_in:&Vector3<f64>, hit_data:&HitData, out:&mut Ray) -> bool{
                return scatter_diffuse(dir_in, hit_data, &mut out.direction);
        }
}

impl Metal{
        pub fn create(color:Vector3<f64>, fuzz:f64) -> Material{
                Material::Metal(Metal{
                        color,
                        fuzz,
                })
        }

        pub fn scatter(&self, dir_in:&Vector3<f64>, hit_data:&HitData, out:&mut Ray) -> bool{
                return scatter_metal(dir_in, hit_data, self.fuzz, &mut out.direction);
        }

        pub fn set_fuzz(&mut self, fuzz:f64){
                self.fuzz = fuzz;
        }
}

impl Glass{
        pub fn create(color:Vector3<f64>, refraction:f64) -> Material{
                Material::Glass(Glass{
                        color,
                        refraction,
                })
        }

        pub fn scatter(&self, dir_in:&Vector3<f64>, hit_data:&HitData, out:&mut Ray) -> bool{
                return scatter_glass(dir_in, hit_data, self.refraction, &mut out.direction);
        }
}

impl Portal{
        pub fn create(color:Vector3<f64>, position:Vector3<f64>, target:Vector3<f64>) -> Material{
                Material::Portal(Portal{
                        color,
                        position,
                        target,
                })
        }

        pub fn scatter(&self, dir_in:&Vector3<f64>, hit_data:&HitData, out:&mut Ray) -> bool{
                //Creating portal border
                if dir_in.dot(&hit_data.norm).abs() < 0.1{
                        return false;
                }

                if hit_data.inside {
                        out.origin = hit_data.point - self.position + self.target;// + 0.00001 * dir_in;
                        out.direction = *dir_in;
                }else{
                        out.origin = hit_data.point;// + 0.00001 * dir_in;
                        out.direction = *dir_in;
                }
                return true;
        }
}

fn scatter_diffuse(dir_in:&Vector3<f64>, hit_data:&HitData, out:&mut Vector3<f64>) -> bool{
        let effective_norm:Vector3<f64>;

        if hit_data.inside{
                //Inside of object
                effective_norm = -hit_data.norm;
        }else{
                effective_norm = hit_data.norm;
        }
        
        //*out = (random_in_hemisphere(&effective_norm)).normalize();
        //*out = (effective_norm + random_in_unit_sphere()).normalize();
        *out = (effective_norm + random_unit_vector()).normalize(); //Lambertian
        return true;
}

fn scatter_metal(dir_in:&Vector3<f64>, hit_data:&HitData, fuzz:f64, out:&mut Vector3<f64>) -> bool{
        let effective_norm:Vector3<f64>;

        if hit_data.inside{
                //Inside of object
                effective_norm = -hit_data.norm;
        }else{
                effective_norm = hit_data.norm;
        }

        let reflected = reflect(dir_in, &effective_norm);
        let dir = reflected + fuzz * random_in_unit_sphere();

        if dir.dot(&effective_norm) > 0.0{
                *out = dir.normalize();
                return true;
        }

        return false;
}

fn scatter_glass(dir_in:&Vector3<f64>, hit_data:&HitData, refraction:f64, out:&mut Vector3<f64>) -> bool{
        let effective_norm:Vector3<f64>;
        let r:f64;

        if hit_data.inside{
                //Inside of object
                effective_norm = -hit_data.norm;
                r = refraction/1.0;
        }else{
                effective_norm = hit_data.norm;
                r = 1.0/refraction;
        }

        *out = refract(dir_in, &effective_norm, r);
        return true;
}

fn refract(dir_in:&Vector3<f64>, norm:&Vector3<f64>, refraction_relation:f64) -> Vector3<f64>{
        /*
        let cos = dir_in.dot(norm);
        let out_perpendicular = refraction_relation * (dir_in + cos.abs()*norm);
        let out_parallel = -(1.0 - out_perpendicular.dot(&out_perpendicular) ).sqrt() * norm;
        return (out_parallel + out_perpendicular).normalize();
*/
        
        let cos_theta = dir_in.dot(norm).abs();
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let sin_theta_2 = refraction_relation*sin_theta;//.min(1.0);

        if (sin_theta_2 > 1.0) || (reflectance(cos_theta, refraction_relation) > rand::thread_rng().gen::<f64>()){
                //Impossible to cannot refract or reflectance chance is greater
                return reflect(dir_in, norm);
        }

        let cos_theta_2 = (1.0 - sin_theta_2*sin_theta_2).sqrt();
        let out_perpendicular = sin_theta_2 * (dir_in + cos_theta.abs()*norm).normalize();
        let out_parallel = -cos_theta_2 * norm;
        return (out_parallel + out_perpendicular).normalize();
}

fn reflectance(cos:f64, refraction_relation:f64) -> f64{
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - refraction_relation) / (1.0 + refraction_relation);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * f64::powf(1.0 - cos, 5.0);
}

fn reflectance2(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0-ref_idx) / (1.0+ref_idx);
        let r0 = r0*r0;
        r0 + (1.0-r0)*((1.0-cosine).powi(5))
}

fn random_in_unit_sphere() -> Vector3<f64>{
    let mut rng = rand::thread_rng();
    loop{
            let p = Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
            if p.norm_squared() <= 1.0{
                    return p;
            }
    }
}

fn random_in_hemisphere(norm:&Vector3<f64>) -> Vector3<f64>{
        let v = random_in_unit_sphere();
        if v.dot(norm) < 0.0{
                return -v;
        }
        return v;
}

fn random_unit_vector() -> Vector3<f64>{
    let v = random_in_unit_sphere();
    return v.normalize();
}

fn reflect(v:&Vector3<f64>, norm:&Vector3<f64>) -> Vector3<f64>{
    return v - 2.0 * v.dot(norm) * norm;
}