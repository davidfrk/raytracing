#![allow(dead_code)]

use std::ops;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub struct Vector3{
	pub x:f64,
	pub y:f64,
	pub z:f64,
}

impl Vector3{
	pub fn new(x:f64, y:f64, z:f64) -> Vector3{
		Vector3{
			x,
			y,
			z,
		}
	}

	pub fn norm_squared(&self) -> f64{
		return self.x * self.x + self.y * self.y + self.z * self.z;
	}

	pub fn norm(&self) -> f64{
		return self.norm_squared().sqrt();
	}

	pub fn normalize(&self) -> Vector3{
		let norm:f64 = self.norm();

		return Vector3::new(self.x / norm, self.y / norm, self.z / norm);
	}

	pub fn dot(&self, other:&Vector3) -> f64{
		return self.x * other.x + self.y * other.y + self.z * other.z;
	}

	pub fn mult(&self, other:&Vector3) -> Vector3{
		Vector3{
			x:self.x * other.x,
			y:self.y * other.y,
			z:self.z * other.z,
		}
	}

	pub fn cross(&self, other:&Vector3) -> Vector3{
		Vector3{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
	}
}

impl Default for Vector3{
	fn default() -> Self{
		Vector3{
			x: 0.0, y: 0.0, z: 0.0,
		}
	}
}

impl Display for Vector3{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result{
		write!(f, "{}{}{}", self.x, self.y, self.z)
	}
}

impl ops::Add<Vector3> for Vector3{
	type Output = Vector3;

	#[inline(always)]
	fn add(self, other:Vector3) -> Vector3{
		Vector3{
			x:self.x + other.x,
			y:self.y + other.y,
			z:self.z + other.z,
		}
	}
}

impl ops::Add<Vector3> for &Vector3{
	type Output = Vector3;

	#[inline(always)]
	fn add(self, other:Vector3) -> Vector3{
		Vector3{
			x:self.x + other.x,
			y:self.y + other.y,
			z:self.z + other.z,
		}
	}
}

impl ops::AddAssign for Vector3{
	#[inline(always)]
	fn add_assign(&mut self, other:Self){
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
	}
}

impl ops::Sub<Vector3> for Vector3{
	type Output = Vector3;

	#[inline(always)]
	fn sub(self, other:Vector3) -> Vector3{
		Vector3{
			x:self.x - other.x,
			y:self.y - other.y,
			z:self.z - other.z,
		}
	}
}

impl ops::Sub<Vector3> for &Vector3{
	type Output = Vector3;

	#[inline(always)]
	fn sub(self, other:Vector3) -> Vector3{
		Vector3{
			x:self.x - other.x,
			y:self.y - other.y,
			z:self.z - other.z,
		}
	}
}

impl ops::Neg for Vector3{
	type Output = Vector3;

	#[inline(always)]
	fn neg(self) -> Vector3{
		Vector3{
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}
}

impl ops::Mul<f64> for Vector3{
	type Output = Vector3;

	#[inline(always)]
	fn mul(self, other:f64) -> Vector3{
		Vector3{
			x:self.x * other,
			y:self.y * other,
			z:self.z * other,
		}
	}
}

impl ops::Mul<f64> for &Vector3{
	type Output = Vector3;

	#[inline(always)]
	fn mul(self, other:f64) -> Vector3{
		Vector3{
			x:self.x * other,
			y:self.y * other,
			z:self.z * other,
		}
	}
}

//Maybe impl in the opposite order
impl ops::Mul<Vector3> for f64{
	type Output = Vector3;

	#[inline(always)]
	fn mul(self, other:Vector3) -> Vector3{
		other * self
	}
}

impl ops::Mul<&Vector3> for f64{
	type Output = Vector3;

	#[inline(always)]
	fn mul(self, other:&Vector3) -> Vector3{
		other * self
	}
}

impl ops::Div<f64> for Vector3{
	type Output = Vector3;

	#[inline(always)]
	fn div(self, other:f64) ->Vector3{
		self * (1.0/other)
	}
}

impl ops::Div<f64> for &Vector3{
	type Output = Vector3;

	#[inline(always)]
	fn div(self, other:f64) ->Vector3{
		self * (1.0/other)
	}
}