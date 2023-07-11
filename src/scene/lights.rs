//extern crate nalgebra as na;
//use na::Vector3;
use crate::vector3::Vector3;

pub enum Light{
	PointLight(PointLight),
}

pub struct PointLight{
	position:Vector3,
	color:Vector3,
}

impl Light{
	pub fn get_position(&self) -> Vector3{
		match self{
			Light::PointLight(light) => {
				return light.position;
			},
		}
	}

	pub fn get_color(&self) -> Vector3{
		match self{
			Light::PointLight(light) => {
				return light.color;
			},
		}
	}
}

impl PointLight{
	pub fn create(position:Vector3, color:Vector3) -> Light{
		return Light::PointLight(PointLight{
			position,
			color,
		});
	}
}