use crate::scene;
use scene::Scene;
use scene::objects::Object;
use scene::objects::Shape;
use scene::objects::Sphere;

//extern crate nalgebra as na;
//use na::Vector3;
use crate::vector3::Vector3;

pub struct Ray{
	pub origin:Vector3,
	pub direction:Vector3,
}

pub enum Hit<'a>{
	Nothing,
	Something(HitData<'a>),
}

pub struct HitData<'a>{
	pub point:Vector3,
	pub norm:Vector3,
	pub inside:bool,
	pub distance:f64,
	pub object:&'a Object,
}

fn test(scene:&Scene) -> bool{
	return scene.objects.len() == scene.spheres.len();
}

pub fn raycast<'a>(scene:&'a Scene, ray:&'a Ray) -> Hit<'a>{
	//let mut closest_hit = Hit::Nothing;
	let mut closest_distance = std::f64::MAX;
	let mut closest_id: i32 = -1;

	//assert!(test(scene), "Different number of spheres and objects");
	/*
	for obj in &scene.objects{
		let hit = obj.intersection(ray);
		
		match hit{
			Hit::Nothing => {continue;},
			Hit::Something(ref hit_data) => {
				
				if hit_data.distance < closest_distance{
					closest_distance = hit_data.distance;
					closest_hit = hit;
				}
			},
		}
	}

	return closest_hit;
	*/
	
	for i in 0..scene.spheres.len() {
		let distance = scene.spheres[i].intersection_distance(ray);

		if distance < closest_distance {
			closest_distance = distance;
			closest_id = i as i32;
		}
	}

	if closest_id >= 0 {
		let id = closest_id as usize;
		return scene.spheres[id].get_hit_data(&scene.objects[id], closest_distance, ray);
	}
	
	return Hit::Nothing;
}

impl Object{
	pub fn intersection(&self, ray:&Ray) -> Hit{
		match self.shape{
			Shape::Sphere(ref s) => {
				return s.intersection(self, ray);
			},
		}
	}
}


impl Sphere{
	pub fn intersection<'a>(&self, object:&'a Object, ray:&Ray) -> Hit<'a>{
			let origin_to_center = self.position - ray.origin;
			let proj_length = origin_to_center.dot(&ray.direction);

			//Test if sphere is in the opposite direction of ray.
			let square_distance_sphere_origin = origin_to_center.norm_squared();
			let square_radius = self.radius*self.radius;
			if proj_length <= 0.0 && square_radius < square_distance_sphere_origin {return Hit::Nothing;}

			let center_to_direction = origin_to_center - proj_length * ray.direction;
			let square_distance = center_to_direction.norm_squared();

			if square_distance <= square_radius{
				let displacement = (square_radius - square_distance).sqrt();

				//Points of intersection
				//p1 = origin + (proj_length - displacement) * direction;
				//p2 = origin + (proj_length + displacement) * direction;

				//Distance from camera
				let distance:f64;
				let inside:bool;

				//Finding if we are inside or outside the sphere and choosing adequate point
				if proj_length > displacement{
					inside = false;
					distance = proj_length - displacement;
				}else{
					inside = true;
					distance = proj_length + displacement;
				}

				//Distance from camera and surface norm
				let point = ray.origin + distance * ray.direction;
				let norm = (point - self.position).normalize();

				return Hit::Something(HitData{
					point:point,
					norm:norm,
					inside:inside,
					distance:distance,
					object:object,
				});
			}

			return Hit::Nothing;
	}

	pub fn intersection_distance(&self, ray:&Ray) -> f64{
		let origin_to_center = self.position - ray.origin;
		let proj_length = origin_to_center.dot(&ray.direction);

		//Test if sphere is in the opposite direction of ray.
		let square_distance_sphere_origin = origin_to_center.norm_squared();
		let square_radius = self.radius*self.radius;
		if proj_length <= 0.0 && square_radius < square_distance_sphere_origin {return f64::MAX;}

		let center_to_direction = origin_to_center - proj_length * ray.direction;
		let square_distance = center_to_direction.norm_squared();

		if square_distance <= square_radius{
			let displacement = (square_radius - square_distance).sqrt();

			//Points of intersection
			//p1 = origin + (proj_length - displacement) * direction;
			//p2 = origin + (proj_length + displacement) * direction;

			//Distance from camera
			let distance:f64;

			//Finding if we are inside or outside the sphere and choosing adequate point
			if proj_length > displacement{
				distance = proj_length - displacement;
			}else{
				distance = proj_length + displacement;
			}

			//Distance from camera to surface
			return distance;
		}

		return f64::MAX;
	}

	pub fn get_hit_data<'a>(&self, object:&'a Object, distance:f64, ray:&Ray) -> Hit<'a> {
		let point = ray.origin + distance * ray.direction;
		let norm = (point - self.position).normalize();

		//Deciding if ray started from inside
		let to_origin = ray.origin - self.position;
		let proj = to_origin.dot(&to_origin);
		let inside = proj < self.radius * self.radius;

		return Hit::Something(HitData{
			point:point,
			norm:norm,
			inside:inside,
			distance:distance,
			object:object,
		});
	}
}