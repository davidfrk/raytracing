#![allow(dead_code)]

extern crate nalgebra as na;
//use na::{Vector3};
use crate::vector3::Vector3;
use std::collections::HashMap;

pub mod objects;
pub mod lights;
pub mod materials;

use objects::Object;
use lights::Light;

pub struct Camera{
    pub position:Vector3,
    pub target:Vector3,
    pub forward:Vector3,
    pub right:Vector3,
    pub up:Vector3,
    pub fov:f64,
    pub focus_dist:f64,
    pub focus_blur:f64,
    /*
    aspect_ratio:f64,
    aperture:f64,
    */
}

impl Camera{
    pub fn new(position:Vector3, target:Vector3, up:Vector3, fov:f64) -> Self{
        let forward = (target - position).normalize();
        let right = forward.cross(&up).normalize();
        let up = right.cross(&forward);

        Camera{
            position,
            target,
            forward,
            right,
            up,
            fov,
            focus_dist: (target - position).norm(), //-0.5
            focus_blur: 0.0,
        }
        /*
        //Calculate aspect ratio and field of view
        let image_aspect_ratio = width as f64 / height as f64;
        let camera_height = (scene.main_camera.fov / 2.0 * std::f64::consts::PI / 180.0).tan();
        let camera_width = image_aspect_ratio * camera_height;
        //println!("Height from Fov: {}", camera_height);

        //Camera vectors
        let origin = scene.main_camera.position;
        let forward = (scene.main_camera.target - origin).normalize();
        let right = forward.cross(&scene.main_camera.up).normalize();
        let up = right.cross(&forward);
        */
    }

    pub fn set_focus_blur(&mut self, focus_dist:f64, focus_blur:f64){
        self.focus_dist = focus_dist;
        self.focus_blur = focus_blur;
    }

    pub fn move_to(&mut self, position:&Vector3){
        self.position = *position;
        self.update_camera_vectors();
    }

    fn update_camera_vectors(&mut self){
        self.forward = (self.target - self.position).normalize();
        self.right = self.forward.cross(&Vector3::new(0.0, 1.0, 0.0)).normalize();
        self.up = self.right.cross(&self.forward);
    }
}

pub struct Scene{
    pub main_camera:Camera,
    pub objects:Vec<Object>,
    pub lights:Vec<Light>,
    pub gradient_light_1:Vector3,
    pub gradient_light_2:Vector3,
}

pub fn load_scene() -> Scene{
    //Set camera
    let main_camera = Camera::new(Vector3::new(10.0, 5.0, 0.0), 
        Vector3::new(01.0, 1.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 60.0);

    //Create materials
    let mut map = HashMap::new();
    let base = materials::Diffuse::create(Vector3::new(0.0, 0.0, 0.0));

    //Emission
    map.insert(String::from("emission_1"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.4, 0.4)));
    map.insert(String::from("emission_2"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.8, 0.4)));
    map.insert(String::from("emission_3"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.4, 0.8)));
    map.insert(String::from("emission_white"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.8, 0.8)));
    map.insert(String::from("emission_red"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.95, 0.1, 0.1)));
    map.insert(String::from("emission_green"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.1, 0.95, 0.1)));
    map.insert(String::from("emission_blue"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.1, 0.1, 0.95)));

    //Diffuse
    map.insert(String::from("diffuse_white"), materials::Diffuse::create(Vector3::new(0.9, 0.9, 0.9)));
    map.insert(String::from("diffuse_red"), materials::Diffuse::create(Vector3::new(0.9, 0.1, 0.1)));
    map.insert(String::from("diffuse_green"), materials::Diffuse::create(Vector3::new(0.1, 0.9, 0.1)));
    map.insert(String::from("diffuse_blue"), materials::Diffuse::create(Vector3::new(0.1, 0.1, 0.9)));
    map.insert(String::from("diffuse_yellow"), materials::Diffuse::create(Vector3::new(0.9, 0.9, 0.0)));
    map.insert(String::from("diffuse_black"), materials::Diffuse::create(Vector3::new(0.1, 0.1, 0.1)));
    map.insert(String::from("diffuse_dark_gray"), materials::Diffuse::create(Vector3::new(0.3, 0.3, 0.3)));

    //Metal
    map.insert(String::from("metal_red_fuzz"), materials::Metal::create(Vector3::new(1.0, 0.45, 0.45), 0.7));
    map.insert(String::from("metal_gray_fuzz"), materials::Metal::create(Vector3::new(0.7, 0.7, 0.7), 0.3));
    map.insert(String::from("metal_silver"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.05));
    map.insert(String::from("metal_silver_fuzz_0.2"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.2));
    map.insert(String::from("metal_silver_fuzz_0.4"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.4));
    map.insert(String::from("metal_silver_fuzz_0.6"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.6));
    map.insert(String::from("metal_silver_fuzz_0.8"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.8));

    //Glass
    map.insert(String::from("glass_diamond"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 2.4));
    map.insert(String::from("glass_glass"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 1.8));
    map.insert(String::from("glass_r_1.0"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 1.0));
    map.insert(String::from("glass_r_1.4"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 1.4));
    map.insert(String::from("glass_r_1.8"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 1.8));
    map.insert(String::from("glass_r_2.2"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 2.2));
    map.insert(String::from("glass_r_2.6"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 2.6));

    //Portals
    let portal_a_pos = Vector3::new(0.0, 2.0, 0.0);
    let portal_b_pos = Vector3::new(100.0, 2.0, 0.0);
    map.insert(String::from("portal_a"), materials::Portal::create(Vector3::new(1.0, 1.0, 1.0), portal_a_pos, portal_b_pos));
    map.insert(String::from("portal_b"), materials::Portal::create(Vector3::new(1.0, 1.0, 1.0), portal_b_pos, portal_a_pos));

    /////////Create objects
    let mut objects:Vec<Object> = Vec::new();

    //Diffuse
    objects.push(objects::Sphere::create(Vector3::new(0.0, -1000.0, 0.0), 1000.0, *map.get(&String::from("diffuse_white")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(100.0, -1000.0, 0.0), 1000.0, *map.get(&String::from("diffuse_white")).unwrap_or(&base) ));

/*
    objects.push(objects::Sphere::create(Vector3::new(-5.0, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_white")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(-2.5, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_red")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(2.5, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_green")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(0.0, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_blue")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(5.0, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_dark_gray")).unwrap_or(&base) ));
*/

    let r = 3.05;
    //Glass
    objects.push(objects::Sphere::create(Vector3::new(100.0 + r * 0.5, 1.0, r * 0.866), 1.0, *map.get(&String::from("glass_r_1.0")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(100.0 + r * 1.0, 1.0, r * 0.0), 1.0, *map.get(&String::from("glass_r_1.4")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(100.0 + r * 0.5, 1.0, r * -0.866), 1.0, *map.get(&String::from("glass_r_1.8")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(100.0 + r * -0.5, 1.0, r * -0.866), 1.0, *map.get(&String::from("glass_r_2.2")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(100.0 + r * -1.0, 1.0, r * 0.0), 1.0, *map.get(&String::from("glass_r_2.6")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(100.0 + r * -0.5, 1.0, r * 0.866), 1.0, *map.get(&String::from("glass_r_2.6")).unwrap_or(&base) ));
    
    //Metal
    objects.push(objects::Sphere::create(Vector3::new(r * 0.5, 1.0, r * 0.866), 1.0, *map.get(&String::from("metal_silver")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(r * 1.0, 1.0, r * 0.0), 1.0, *map.get(&String::from("metal_silver_fuzz_0.2")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(r * 0.5, 1.0, r * -0.866), 1.0, *map.get(&String::from("metal_silver_fuzz_0.4")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(r * -0.5, 1.0, r * -0.866), 1.0, *map.get(&String::from("metal_silver_fuzz_0.6")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(r * -1.0, 1.0, r * 0.0), 1.0, *map.get(&String::from("metal_silver_fuzz_0.8")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(r * -0.5, 1.0, r * 0.866), 1.0, *map.get(&String::from("metal_silver_fuzz_0.8")).unwrap_or(&base) ));

    //Emission
    
    objects.push(objects::Sphere::create(Vector3::new(0.0, 10.0, -5.0), 5.0, *map.get(&String::from("emission_red")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(0.0, 10.0, 0.0), 5.0, *map.get(&String::from("emission_green")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(0.0, 10.0, 5.0), 5.0, *map.get(&String::from("emission_blue")).unwrap_or(&base) ));

    objects.push(objects::Sphere::create(Vector3::new(100.0, 10.0, -5.0), 5.0, *map.get(&String::from("emission_1")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(100.0, 10.0, 0.0), 5.0, *map.get(&String::from("emission_2")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(100.0, 10.0, 5.0), 5.0, *map.get(&String::from("emission_3")).unwrap_or(&base) ));
    
    //Portals
    objects.push(objects::Sphere::create(portal_a_pos, 2.0, *map.get(&String::from("portal_a")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(portal_b_pos, 2.0, *map.get(&String::from("portal_b")).unwrap_or(&base) ));

    //Create lights
    let mut lights:Vec<Light> = Vec::new();
    lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, -4.0), Vector3::new(1.0, 0.0, 0.0)));
    lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, 0.0), Vector3::new(0.0, 1.0, 0.0)));
    lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, 4.0), Vector3::new(0.0, 0.0, 1.0)));

    let gradient_light_1 = Vector3::new(0.67, 0.84, 0.97);
    //let gradient_light_2 = Vector3::new(0.63, 0.74, 0.90);
    let gradient_light_2 = Vector3::new(0.57, 0.63, 0.70);

    return Scene{
        main_camera,
        objects,
        lights,
        gradient_light_1,
        gradient_light_2,
    };
}


pub fn load_scene_2() -> Scene{
    //Set camera
    let main_camera = Camera::new(Vector3::new(10.0, 7.0, 0.0), 
        Vector3::new(1.0, 1.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 60.0);

    //Create materials
    let mut map = HashMap::new();
    let base = materials::Diffuse::create(Vector3::new(0.0, 0.0, 0.0));

    //Emission
    map.insert(String::from("emission_1"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.4, 0.4)));
    map.insert(String::from("emission_2"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.8, 0.4)));
    map.insert(String::from("emission_3"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.4, 0.8)));
    map.insert(String::from("emission_white"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.8, 0.8)));
    map.insert(String::from("emission_red"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.95, 0.1, 0.1)));
    map.insert(String::from("emission_green"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.1, 0.95, 0.1)));
    map.insert(String::from("emission_blue"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.1, 0.1, 0.95)));

    //Diffuse
    map.insert(String::from("diffuse_white"), materials::Diffuse::create(Vector3::new(0.9, 0.9, 0.9)));
    map.insert(String::from("diffuse_red"), materials::Diffuse::create(Vector3::new(0.9, 0.1, 0.1)));
    map.insert(String::from("diffuse_green"), materials::Diffuse::create(Vector3::new(0.1, 0.9, 0.1)));
    map.insert(String::from("diffuse_blue"), materials::Diffuse::create(Vector3::new(0.1, 0.1, 0.9)));
    map.insert(String::from("diffuse_yellow"), materials::Diffuse::create(Vector3::new(0.9, 0.9, 0.0)));
    map.insert(String::from("diffuse_black"), materials::Diffuse::create(Vector3::new(0.1, 0.1, 0.1)));
    map.insert(String::from("diffuse_dark_gray"), materials::Diffuse::create(Vector3::new(0.3, 0.3, 0.3)));

    //Metal
    map.insert(String::from("metal_red_fuzz"), materials::Metal::create(Vector3::new(1.0, 0.45, 0.45), 0.7));
    map.insert(String::from("metal_gray_fuzz"), materials::Metal::create(Vector3::new(0.7, 0.7, 0.7), 0.3));
    map.insert(String::from("metal_silver"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.05));
    map.insert(String::from("metal_silver_fuzz_0.2"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.2));
    map.insert(String::from("metal_silver_fuzz_0.4"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.4));
    map.insert(String::from("metal_silver_fuzz_0.6"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.6));
    map.insert(String::from("metal_silver_fuzz_0.8"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.8));

    //Glass
    map.insert(String::from("glass_diamond"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 2.4));
    map.insert(String::from("glass_glass"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 1.8));
    map.insert(String::from("glass_r_1.0"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 1.0));
    map.insert(String::from("glass_r_1.4"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 1.4));
    map.insert(String::from("glass_r_1.8"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 1.8));
    map.insert(String::from("glass_r_2.2"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 2.2));
    map.insert(String::from("glass_r_2.6"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 2.6));

    //Portals
    let portal_a_pos = Vector3::new(0.0, 1.0, 2.0);
    let portal_b_pos = Vector3::new(0.0, 5.0, 2.0);
    map.insert(String::from("portal_a"), materials::Portal::create(Vector3::new(1.0, 1.0, 1.0), portal_a_pos, portal_b_pos));
    map.insert(String::from("portal_b"), materials::Portal::create(Vector3::new(1.0, 1.0, 1.0), portal_b_pos, portal_a_pos));

    /////////Create objects
    let mut objects:Vec<Object> = Vec::new();

    //Diffuse
    objects.push(objects::Sphere::create(Vector3::new(0.0, -1000.0, 0.0), 1000.0, *map.get(&String::from("diffuse_white")).unwrap_or(&base) ));

    objects.push(objects::Sphere::create(Vector3::new(-5.0, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_white")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(-2.5, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_red")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(2.5, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_green")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(0.0, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_blue")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(5.0, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_dark_gray")).unwrap_or(&base) ));

    //Glass
    objects.push(objects::Sphere::create(Vector3::new(-5.0, 1.0, 0.0), 1.0, *map.get(&String::from("glass_r_1.0")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(-2.5, 1.0, 0.0), 1.0, *map.get(&String::from("glass_r_1.4")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(0.0, 1.0, 0.0), 1.0, *map.get(&String::from("glass_r_1.8")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(2.5, 1.0, 0.0), 1.0, *map.get(&String::from("glass_r_2.2")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(5.0, 1.0, 0.0), 1.0, *map.get(&String::from("glass_r_2.6")).unwrap_or(&base) ));
    
    //Metal
    objects.push(objects::Sphere::create(Vector3::new(-5.0, 1.0, -2.0), 1.0, *map.get(&String::from("metal_silver")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(-2.5, 1.0, -2.0), 1.0, *map.get(&String::from("metal_silver_fuzz_0.2")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(0.0, 1.0, -2.0), 1.0, *map.get(&String::from("metal_silver_fuzz_0.4")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(2.5, 1.0, -2.0), 1.0, *map.get(&String::from("metal_silver_fuzz_0.6")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(5.0, 1.0, -2.0), 1.0, *map.get(&String::from("metal_silver_fuzz_0.8")).unwrap_or(&base) ));

    //Emission
    objects.push(objects::Sphere::create(Vector3::new(0.0, 10.0, -5.0), 5.0, *map.get(&String::from("emission_red")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(0.0, 10.0, 0.0), 5.0, *map.get(&String::from("emission_green")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(0.0, 10.0, 5.0), 5.0, *map.get(&String::from("emission_blue")).unwrap_or(&base) ));

    //Portals
    //objects.push(objects::Sphere::create(portal_a_pos, 2.0, *map.get(&String::from("portal_a")).unwrap_or(&base) ));
    //objects.push(objects::Sphere::create(portal_b_pos, 2.0, *map.get(&String::from("portal_b")).unwrap_or(&base) ));

    //Create lights
    let mut lights:Vec<Light> = Vec::new();
    lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, -4.0), Vector3::new(1.0, 0.0, 0.0)));
    lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, 0.0), Vector3::new(0.0, 1.0, 0.0)));
    lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, 4.0), Vector3::new(0.0, 0.0, 1.0)));

    let gradient_light_1 = Vector3::new(0.67, 0.84, 0.97);
    //let gradient_light_2 = Vector3::new(0.63, 0.74, 0.90);
    let gradient_light_2 = Vector3::new(0.3, 0.3, 0.3);

    return Scene{
        main_camera,
        objects,
        lights,
        gradient_light_1,
        gradient_light_2,
    };
}


pub fn load_scene_3() -> Scene{
    //Set camera
    /*
    let main_camera = Camera {
        position: Vector3::new(-3.5, 4.9, 3.0),
        //position: Vector3::new(-3.0, 2.0, 4.0),
        target: Vector3::new(0.0, 0.0, 1.0),
        up: Vector3::new(0.0, 1.0, 0.0),
        fov: 90.0,
    };*/
    let main_camera = Camera::new(Vector3::new(-3.5, 4.9, 3.0), 
        Vector3::new(0.0, 1.0, 2.0), Vector3::new(0.0, 1.0, 0.0), 90.0);

    //let main_camera = Camera::new(Vector3::new(-3.5, 1.0, 4.0), 
    //    Vector3::new(-0.5, 1.0, 4.0), Vector3::new(0.0, 1.0, 0.0), 90.0);

    //Create materials
    let mut map = HashMap::new();
    let base = materials::Diffuse::create(Vector3::new(0.0, 0.0, 0.0));

    //Emission
    map.insert(String::from("emission_1"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.3, 0.3)));
    map.insert(String::from("emission_2"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.8, 0.3)));
    map.insert(String::from("emission_3"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.1, 0.8)));
/*
    map.insert(String::from("emission_1"), materials::Emission::create(Vector3::new(1.0, 0.4, 0.4)));
    map.insert(String::from("emission_2"), materials::Emission::create(Vector3::new(0.4, 1.0, 0.4)));
    map.insert(String::from("emission_3"), materials::Emission::create(Vector3::new(0.4, 0.0, 1.4)));
*/
    //Diffuse
    map.insert(String::from("diffuse_white"), materials::Diffuse::create(Vector3::new(0.9, 0.9, 0.9)));
    map.insert(String::from("diffuse_red"), materials::Diffuse::create(Vector3::new(0.9, 0.1, 0.1)));
    map.insert(String::from("diffuse_blue"), materials::Diffuse::create(Vector3::new(0.1, 0.1, 0.9)));
    map.insert(String::from("diffuse_yellow"), materials::Diffuse::create(Vector3::new(0.9, 0.9, 0.0)));
    //Metal
    map.insert(String::from("metal_red_fuzz"), materials::Metal::create(Vector3::new(1.0, 0.45, 0.45), 0.7));
    map.insert(String::from("metal_gray_fuzz"), materials::Metal::create(Vector3::new(0.7, 0.7, 0.7), 0.3));
    map.insert(String::from("metal_silver"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.05));

    //Glass
    map.insert(String::from("glass_diamond"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 2.4));
    map.insert(String::from("glass_glass"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 1.8));

    //Portals
    let portal_a_pos = Vector3::new(0.0, 1.0, 2.0);
    let portal_b_pos = Vector3::new(0.0, 5.0, 2.0);
    map.insert(String::from("portal_a"), materials::Portal::create(Vector3::new(1.0, 1.0, 1.0), portal_a_pos, portal_b_pos));
    map.insert(String::from("portal_b"), materials::Portal::create(Vector3::new(1.0, 1.0, 1.0), portal_b_pos, portal_a_pos));

    //Create objects
    let mut objects:Vec<Object> = Vec::new();

    objects.push(objects::Sphere::create(Vector3::new(0.0, 1.0, 0.0), 1.0, *map.get(&String::from("emission_1")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(2.0, 1.0, 0.0), 1.0, *map.get(&String::from("diffuse_white")).unwrap_or(&base) ));
/*
    //Glass
    objects.push(objects::Sphere::create(Vector3::new(0.0, 1.0, 2.0), 1.0, *map.get(&String::from("glass_diamond")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(4.0, 1.0, 2.0), 1.0, *map.get(&String::from("glass_glass")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(-1.5, 0.5, 5.2), 0.5, *map.get(&String::from("glass_glass")).unwrap_or(&base) ));
    //Metal
    objects.push(objects::Sphere::create(Vector3::new(-2.5, 1.5, 0.0), 1.5, *map.get(&String::from("metal_red_fuzz")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(2.5, 1.0, 4.0), 1.0, *map.get(&String::from("metal_red_fuzz")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(-0.5, 1.0, 4.0), 1.0, *map.get(&String::from("metal_silver")).unwrap_or(&base) ));
*/
    /////////Version Diffuse
    //Glass
    //objects.push(objects::Sphere::create(Vector3::new(0.0, 1.0, 2.0), 1.0, *map.get(&String::from("glass_diamond")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(4.0, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_red")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(-1.5, 0.5, 5.2), 0.5, *map.get(&String::from("diffuse_white")).unwrap_or(&base) ));
    //Metal
    objects.push(objects::Sphere::create(Vector3::new(-2.5, 1.5, 0.0), 1.5, *map.get(&String::from("diffuse_white")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(2.5, 1.0, 4.0), 1.0, *map.get(&String::from("metal_red_fuzz")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(-0.5, 1.0, 4.0), 1.0, *map.get(&String::from("metal_silver")).unwrap_or(&base) ));
    
    objects.push(objects::Sphere::create(Vector3::new(0.0, -50.0, 0.0), 50.0, *map.get(&String::from("metal_silver")).unwrap_or(&base) ));
    //objects.push(objects::Sphere::create(Vector3::new(0.0, 0.0, 0.0), 10.0, (materials[3]) ));

    //Emission
    objects.push(objects::Sphere::create(Vector3::new(-3.0, 10.0, 0.0), 5.0, *map.get(&String::from("emission_2")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(3.0, 10.0, 0.0), 5.0, *map.get(&String::from("emission_3")).unwrap_or(&base) ));

    //Portals
    objects.push(objects::Sphere::create(portal_a_pos, 1.0, *map.get(&String::from("portal_a")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(portal_b_pos, 1.0, *map.get(&String::from("portal_b")).unwrap_or(&base) ));

    //Create lights
    let mut lights:Vec<Light> = Vec::new();
    lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, -4.0), Vector3::new(1.0, 0.0, 0.0)));
    lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, 0.0), Vector3::new(0.0, 1.0, 0.0)));
    lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, 4.0), Vector3::new(0.0, 0.0, 1.0)));

    let gradient_light_1 = Vector3::new(0.67, 0.84, 0.97);
    //let gradient_light_2 = Vector3::new(0.63, 0.74, 0.90);
    let gradient_light_2 = Vector3::new(0.3, 0.3, 0.3);

    return Scene{
        main_camera,
        objects,
        lights,
        gradient_light_1,
        gradient_light_2,
    };
}