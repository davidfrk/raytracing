#![allow(dead_code)]

use crate::window;
use window::Window;

use crate::scene;
use scene::Scene;

use crate::render;
use crate::render::raytracing_config::RaytracingConfig;

use crate::scene::materials;
use materials::Material;

//extern crate nalgebra as na;
//use na::Vector3;
use crate::vector3::Vector3;
const PI: f64 = 3.14159;

pub fn render_blur_transition(window:&Window, raytracing_config:RaytracingConfig, scene:&mut Scene, frames:u32, focus_dist_start:f64, focus_dist_end:f64, focus_blur_start:f64, focus_blur_end:f64){
	for i in 0..frames{
		let t = i as f64 / frames as f64;

		let focus_dist = (1.0 - t) * focus_dist_start + t * focus_dist_end;
		let focus_blur = (1.0 - t) * focus_blur_start + t * focus_blur_end;
		scene.main_camera.set_focus_blur(focus_dist, focus_blur);
		
		let img = render::render(scene, window.width, window.height, raytracing_config);
		img.save(generate_file_name(i)).unwrap();

		println!("Frame: {}", i.to_string());
	}
}

pub fn render_camera_rotation (window:&Window, raytracing_config:RaytracingConfig, scene:&mut Scene, frames:u32, radius:f64){
	let x = Vector3::new(1.0, 0.0, 0.0);
	let z = Vector3::new(0.0, 0.0, 1.0);

	//let v = scene.main_camera.position - scene.main_camera.target;
	//let radius = (v.x * v.x + v.z * v.z).sqrt();

	for i in 0..frames{
		let mut t = i as f64 / frames as f64;
		t += 0.5;
		t = 2.0 * t * std::f64::consts::PI;

		let mut pos = t.sin() * x + t.cos() * z;
		pos = radius * pos;
		//pos = pos + scene.main_camera.target;
		pos.y = scene.main_camera.position.y;
		pos.x += 100.0;

		scene.main_camera.move_to(&pos);

		let img = render::render(scene, window.width, window.height, raytracing_config);
		img.save(generate_file_name(i)).unwrap();

		println!("Frame: {}", i.to_string());
	}
}

pub fn render_metal_fuzz_animation(window:&Window, raytracing_config:RaytracingConfig, scene:&mut Scene, starting_frame:u32, frames:u32, object_id:u32, fuzz_start:f64, fuzz_end:f64){
	for i in 0..frames{
		//Interpolate fuzz
		let t = i as f64 / frames as f64;
		let fuzz = lerp(fuzz_start, fuzz_end, t);

		//Find material, necessary do in loop to prevent mutable reference from existing
		let material = &mut scene.objects[object_id as usize].material;

		match material{
			Material::Metal(ref mut metal) =>{
				metal.set_fuzz(fuzz);
			},
			_ => {
				eprintln!("Object is not a metal.");
				return;
			}
		}

		//Render image
		let img = render::render(scene, window.width, window.height, raytracing_config);
		img.save(generate_file_name(i + starting_frame) ).unwrap();

		println!("Frame: {}", (i + starting_frame).to_string());
	}
}

pub fn render_light_tunnel_animation(window:&Window, raytracing_config:RaytracingConfig, scene:&mut Scene, starting_frame:u32, frames:u32,
		 oscilation_speed:f64, oscilation_size:f64, cam_start:Vector3, cam_end:Vector3, starting_obj_id:usize){
	
	let distance = oscilation_size + 5.0;// + oscilation_size / 2.0;

	for i in starting_frame..frames{
		let time = i as f64 / frames as f64;

		//Objects animation
		let displacement = (20.0/60.0 * PI * oscilation_speed).cos() * oscilation_size;//change to time to animate positions

		for obj_id in starting_obj_id..scene.objects.len(){
			if obj_id % 2 == 0{
				scene.spheres[obj_id].position.z = displacement + distance;
			} else {
				scene.spheres[obj_id].position.z = - displacement - distance;
			}
		}

		//Camera animation
		let cam_pos = (1.0 - time) * cam_start + time * cam_end;
		scene.lights[0].set_position(cam_pos);

		//Render image
		let img = render::render(scene, window.width, window.height, raytracing_config);
		img.save(generate_file_name(i)).unwrap();

		println!("Frame: {}", i.to_string());
	}
}

fn lerp(a:f64, b:f64, t:f64) -> f64{
	return (1.0 - t) * a + t * b;
}

fn generate_file_name(indice:u32) -> String{
	return format!("{}{}{}", "output_", indice.to_string(), ".png");
}