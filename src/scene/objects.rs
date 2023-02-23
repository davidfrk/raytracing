extern crate nalgebra as na;
use na::Vector3;

use super::materials::Material;

pub struct Object{
        pub shape:Shape,
        pub material:Material,
}

pub enum Shape{
        Sphere(Sphere),
}

pub struct Sphere {
        pub position:Vector3<f64>,
        pub radius:f64,
}

impl Sphere{
        pub fn create(position:Vector3<f64>, radius:f64, material:Material) -> Object{
                return Object{
                        shape:Shape::Sphere(
                                Sphere{
                                        position,
                                        radius,
                                }),
                        material,/*:Material{
                                color:material.color,
                                emission:material.emission,
                                fuzz:material.fuzz,
                        },*/
                };
        }
}
