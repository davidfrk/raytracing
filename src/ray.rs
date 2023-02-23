extern crate nalgebra as na;
use na::Vector3;

pub struct Ray{
	pub origin:Vector3<f64>,
	pub direction:Vector3<f64>,
}