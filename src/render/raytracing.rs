#![allow(dead_code,unused_variables)]

use crate::scene;
use scene::Scene;
use scene::materials::Material;
use crate::intersection;
use intersection::Ray;
use intersection::Hit;
use intersection::HitData;

//extern crate nalgebra as na;
//use na::Vector3;
use crate::vector3::Vector3;

static DISPLACEMENT_DISTANCE:f64 = 0.0000001;

pub fn cast_ray_with_normal_albedo(scene:&Scene, ray:&Ray, depth:u8, normal:&mut Vector3, albedo:&mut Vector3) -> Vector3{
	let intersection = intersection::raycast(scene, ray);

	match intersection{
		Hit::Nothing => {
			//Skybox
			*normal = ray.direction;
			*albedo = skybox(scene, ray);
			return *albedo;
		},
		Hit::Something(ref hit_data) => {
			*normal = hit_data.norm;
			*albedo = hit_data.object.material.attenuation();
			return //color_mult(&scene.ambient_light, &hit_data.object.material.color)
				//hit_data.object.material.emission
				compute_direct_illumination(scene, &ray.direction, &hit_data) +
				compute_indirect_illumination(scene, ray, &hit_data, depth);
		},
	}
}

pub fn cast_ray(scene:&Scene, ray:&Ray, depth:u8) -> Vector3{
	let intersection = intersection::raycast(scene, ray);

	match intersection{
		Hit::Nothing => {
			return skybox(scene, ray);
		},
		Hit::Something(ref hit_data) => {
			return //color_mult(&scene.ambient_light, &hit_data.object.material.color)
				//hit_data.object.material.emission
				compute_direct_illumination(scene, &ray.direction, &hit_data) +
				compute_indirect_illumination(scene, ray, &hit_data, depth);
		},
	}
}

fn skybox(scene:&Scene, ray:&Ray) -> Vector3{
	let t = ray.direction.y.abs(); //0.5 * (ray.direction.y + 1.0);
	return t * scene.gradient_light_1 + (1.0 - t) * scene.gradient_light_2;
}

fn compute_direct_illumination(scene:&Scene, direction:&Vector3, hit_data:&HitData) -> Vector3{
	let mut color = Vector3::new(0.0, 0.0, 0.0);

	match hit_data.object.material{
		Material::Glass(_) | Material::Metal(_) | Material::Portal(_) => {
			return color;
		}
		_ => {}
	}

	let effective_norm:Vector3;

	if hit_data.inside{
		//If they are in the same direction reflection/surface norm must be inverted.
		//Inside object, effective norm is flipped of surface norm
		effective_norm = -hit_data.norm;
	}else{
		effective_norm = hit_data.norm;
	}

	let displacement_point = hit_data.point + DISPLACEMENT_DISTANCE * effective_norm;

	//Direct light
	for light in &scene.lights{
		//Compute distance and direction to light
		let mut light_dir = light.get_position() - hit_data.point;
		let squared_light_distance = light_dir.dot(&light_dir);
		light_dir = light_dir.normalize();

		//Cos between norm and light
		let cos = hit_data.norm.dot(&light_dir);

		let ray = Ray{
			origin: displacement_point,
			direction: light_dir,
		};

		let intersection = intersection::raycast(scene, &ray);

		match intersection{
			Hit::Nothing => {
				color += cos.abs() * light.get_color();
			},
			Hit::Something(ref light_hit_data) => {
				if light_hit_data.distance * light_hit_data.distance >= squared_light_distance{
					color += cos.abs() * light.get_color();
				}
			},
		}
	}

	color = color.mult(&hit_data.object.material.attenuation());
	return color;
}

fn compute_indirect_illumination(scene:&Scene, in_ray:&Ray, hit_data:&HitData, depth:u8) -> Vector3{
	let mut color = Vector3::new(0.0, 0.0, 0.0);

	if depth > 0 {
		let mut out_ray = Ray{
			origin: Vector3::new(0.0, 0.0, 0.0),
			direction: Vector3::new(0.0, 0.0, 0.0),
		};

		out_ray.origin = hit_data.point;
		
		if hit_data.object.material.scatter(&in_ray.direction, &hit_data, &mut out_ray){
			//Computing displacement point to prevent point float errors
			if hit_data.norm.dot(&out_ray.direction) >= 0.0{
				out_ray.origin +=  DISPLACEMENT_DISTANCE * hit_data.norm;
			}else{
				out_ray.origin += - DISPLACEMENT_DISTANCE * hit_data.norm;
			}

			//The math is with effective_norm instead of norm, however, we do a cos.abs() anyway
			//let cos = hit_data.norm.dot(&in_ray.direction);
			color +=  /* cos.abs() * */  cast_ray(scene, &out_ray, depth - 1);
		}

		color = color.mult(&hit_data.object.material.attenuation());
	}

	if let Material::Emission(m) = hit_data.object.material{
		color += m.emission;
	}

	return color;
}