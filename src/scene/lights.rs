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

	pub fn set_position(&mut self, pos:Vector3){
		match self{
			Light::PointLight(light) => {
				light.position = pos;
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

	pub fn get_color_attenuated(&self, distance:f64) -> Vector3{
		match self {
			Light::PointLight(light) => {
				return light.get_color_attenuated(distance);
			}
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

	pub fn get_color_attenuated(&self, distance:f64) -> Vector3{
		//physically correct square decay, difficult to insert artistic view
		//return 1.0 / (distance * distance) * self.color;

		//square decay with 0.0 to 1.0 ajust
		return 1.0 / (1.0 + distance * distance) * self.color;

		//linear decay
		//return 1.0 / distance * self.color;

		//no decay
		//return self.color;

		//more complicated but ajustable decay
		//https://lisyarus.github.io/blog/graphics/2022/07/30/point-light-attenuation.html
		//let radius = 20.0;
		//let fallout = 10.0;
		//let normalized_distance = distance / radius;
//
		//if normalized_distance >= 1.0  {return Vector3::new(0.0, 0.0, 0.0);}
//
		//let distance_square = normalized_distance * normalized_distance;
		//let mut n = 1.0 - distance_square;
		//n = n * n;
		//let d = 1.0 + fallout * distance_square;
		//return (n / d) * self.color;
	}
}