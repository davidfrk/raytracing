#![allow(dead_code)]

//extern crate nalgebra as na;
//use na::{Vector3};
use crate::vector3::Vector3;
use std::collections::HashMap;

pub mod objects;
pub mod lights;
pub mod materials;

use objects::Object;
use lights::Light;

use self::objects::Sphere;

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

    pub materials:HashMap<String, materials::Material>,
    pub spheres:Vec<Sphere>,
}

impl Scene{
    pub fn new(camera:Camera) -> Scene{
        return Scene{
            main_camera:camera,
            objects:Vec::new(),
            lights:Vec::new(),
            gradient_light_1:Vector3::new(0.67, 0.84, 0.97),
            gradient_light_2:Vector3::new(0.57, 0.63, 0.70),
            materials:HashMap::new(),
            spheres:Vec::new(),
        };
    }

    pub fn add_material(&mut self, name:String, material:materials::Material){
        self.materials.insert(name, material);
    }

    pub fn get_material(&self, material: &String) -> &materials::Material {
        return self.materials.get(material).unwrap_or(&materials::BASE_MATERIAL);
    }

    pub fn create_sphere(&mut self, pos:Vector3, radius:f64, material: &String){
        self.objects.push(objects::Sphere::create(pos, radius, *self.get_material(material)));
        self.spheres.push(Sphere::create_sphere(pos, radius));
    }
}

pub fn load_scene() -> Scene{
    //Set camera
    let main_camera = Camera::new(Vector3::new(10.0, 5.0, 0.0), 
        Vector3::new(01.0, 1.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 60.0);

    //Create Scene
    let mut scene:Scene = Scene::new(main_camera);

    //Create lights
    scene.lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, -4.0), Vector3::new(1.0, 0.0, 0.0)));
    scene.lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, 0.0), Vector3::new(0.0, 1.0, 0.0)));
    scene.lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, 4.0), Vector3::new(0.0, 0.0, 1.0)));

    load_materials(&mut scene);

    //Portals
    let portal_a_pos = Vector3::new(0.0, 2.0, 0.0);
    let portal_b_pos = Vector3::new(100.0, 2.0, 0.0);
    scene.add_material(String::from("portal_a"), materials::Portal::create(Vector3::new(1.0, 1.0, 1.0), portal_a_pos, portal_b_pos));
    scene.add_material(String::from("portal_b"), materials::Portal::create(Vector3::new(1.0, 1.0, 1.0), portal_b_pos, portal_a_pos));

    /////////Create objects

    //Diffuse
    scene.create_sphere(Vector3::new(0.0, -1000.0, 0.0), 1000.0, &String::from("diffuse_white"));
    scene.create_sphere(Vector3::new(100.0, -1000.0, 0.0), 1000.0, &String::from("diffuse_white"));

/*
    objects.push(objects::Sphere::create(Vector3::new(-5.0, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_white")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(-2.5, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_red")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(2.5, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_green")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(0.0, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_blue")).unwrap_or(&base) ));
    objects.push(objects::Sphere::create(Vector3::new(5.0, 1.0, 2.0), 1.0, *map.get(&String::from("diffuse_dark_gray")).unwrap_or(&base) ));
*/

    let r = 3.05;
    //Glass
    scene.create_sphere(Vector3::new(100.0 + r * 0.5, 1.0, r * 0.866), 1.0, &String::from("glass_r_1.0"));
    scene.create_sphere(Vector3::new(100.0 + r * 1.0, 1.0, r * 0.0), 1.0, &String::from("glass_r_1.4"));
    scene.create_sphere(Vector3::new(100.0 + r * 0.5, 1.0, r * -0.866), 1.0, &String::from("glass_r_1.8"));
    scene.create_sphere(Vector3::new(100.0 + r * -0.5, 1.0, r * -0.866), 1.0, &String::from("glass_r_2.2"));
    scene.create_sphere(Vector3::new(100.0 + r * -1.0, 1.0, r * 0.0), 1.0, &String::from("glass_r_2.6"));
    scene.create_sphere(Vector3::new(100.0 + r * -0.5, 1.0, r * 0.866), 1.0, &String::from("glass_r_2.6"));
    
    //Metal
    scene.create_sphere(Vector3::new(r * 0.5, 1.0, r * 0.866), 1.0, &String::from("metal_silver"));
    scene.create_sphere(Vector3::new(r * 1.0, 1.0, r * 0.0), 1.0, &String::from("metal_silver_fuzz_0.2"));
    scene.create_sphere(Vector3::new(r * 0.5, 1.0, r * -0.866), 1.0, &String::from("metal_silver_fuzz_0.4"));
    scene.create_sphere(Vector3::new(r * -0.5, 1.0, r * -0.866), 1.0, &String::from("metal_silver_fuzz_0.6"));
    scene.create_sphere(Vector3::new(r * -1.0, 1.0, r * 0.0), 1.0, &String::from("metal_silver_fuzz_0.8"));
    scene.create_sphere(Vector3::new(r * -0.5, 1.0, r * 0.866), 1.0, &String::from("metal_silver_fuzz_0.8"));

    //Emission
    scene.create_sphere(Vector3::new(0.0, 10.0, -5.0), 5.0, &String::from("emission_red"));
    scene.create_sphere(Vector3::new(0.0, 10.0, 0.0), 5.0, &String::from("emission_green"));
    scene.create_sphere(Vector3::new(0.0, 10.0, 5.0), 5.0, &String::from("emission_blue"));
    scene.create_sphere(Vector3::new(100.0, 10.0, -5.0), 5.0, &String::from("emission_1"));
    scene.create_sphere(Vector3::new(100.0, 10.0, 0.0), 5.0, &String::from("emission_2"));
    scene.create_sphere(Vector3::new(100.0, 10.0, 5.0), 5.0, &String::from("emission_3"));
    
    //Portals
    scene.create_sphere(portal_a_pos, 2.0, &String::from("portal_a"));
    scene.create_sphere(portal_b_pos, 2.0, &String::from("portal_b"));

    return scene;
}

pub fn load_scene_2() -> Scene{
    //Set camera
    let main_camera = Camera::new(Vector3::new(60.0, 8.0, 0.0), 
        Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 60.0);

    //Create Scene
    let mut scene:Scene = Scene::new(main_camera);
    load_materials(&mut scene);

    //Create lights
    scene.lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, 0.0), Vector3::new(30.3, 10.3, 10.3)));
    //scene.lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, 0.0), Vector3::new(0.0, 1.0, 0.0)));
    //scene.lights.push(lights::PointLight::create(Vector3::new(0.0, 5.0, 4.0), Vector3::new(0.0, 0.0, 1.0)));

    //Portals
    //let portal_a_pos = Vector3::new(0.0, 2.0, 0.0);
    //let portal_b_pos = Vector3::new(100.0, 2.0, 0.0);
    //scene.add_material(String::from("portal_a"), materials::Portal::create(Vector3::new(1.0, 1.0, 1.0), portal_a_pos, portal_b_pos));
    //scene.add_material(String::from("portal_b"), materials::Portal::create(Vector3::new(1.0, 1.0, 1.0), portal_b_pos, portal_a_pos));

    /////////Create objects

    //Diffuse
    scene.create_sphere(Vector3::new(0.0, -10000.0, 0.0), 10000.0, &String::from("diffuse_white"));
    scene.create_sphere(Vector3::new(0.0, 10010.0, 0.0), 10000.0, &String::from("diffuse_white"));
    scene.create_sphere(Vector3::new(0.0, 0.0, 10010.0), 10000.0, &String::from("diffuse_white"));
    scene.create_sphere(Vector3::new(0.0, 0.0, -10010.0), 10000.0, &String::from("diffuse_white"));
    scene.create_sphere(Vector3::new(-10010.0, 0.0, 0.0), 10000.0, &String::from("diffuse_white"));

    //Emission
    //scene.create_sphere(Vector3::new(0.0, 5.0, 0.0), 4.0, &String::from("emission_white"));

    //Diffuse
    scene.create_sphere(Vector3::new(05.0, 2.0, 5.0), 5.0, &String::from("diffuse_white"));
    scene.create_sphere(Vector3::new(10.0, 2.0, -5.0), 5.0, &String::from("diffuse_white"));
    scene.create_sphere(Vector3::new(15.0, 2.0, 5.0), 5.0, &String::from("diffuse_white"));
    scene.create_sphere(Vector3::new(20.0, 2.0, -5.0), 5.0, &String::from("diffuse_white"));
    scene.create_sphere(Vector3::new(25.0, 2.0, 5.0), 5.0, &String::from("diffuse_yellow"));
    scene.create_sphere(Vector3::new(30.0, 2.0, -5.0), 5.0, &String::from("diffuse_red"));
    scene.create_sphere(Vector3::new(35.0, 2.0, 5.0), 5.0, &String::from("diffuse_green"));
    scene.create_sphere(Vector3::new(40.0, 2.0, -5.0), 5.0, &String::from("diffuse_blue"));
    scene.create_sphere(Vector3::new(45.0, 2.0, 5.0), 5.0, &String::from("diffuse_yellow"));
    scene.create_sphere(Vector3::new(50.0, 2.0, -5.0), 5.0, &String::from("diffuse_red"));
    scene.create_sphere(Vector3::new(55.0, 2.0, 5.0), 5.0, &String::from("diffuse_green"));
    scene.create_sphere(Vector3::new(60.0, 2.0, -5.0), 5.0, &String::from("diffuse_blue"));
    
    //Portals
    //scene.create_sphere(portal_a_pos, 2.0, &String::from("portal_a"));
    //scene.create_sphere(portal_b_pos, 2.0, &String::from("portal_b"));

    return scene;

}

fn load_materials(scene: &mut Scene){
    ////////Create Materials

    //Emission
    scene.add_material(String::from("emission_1"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.4, 0.4)));
    scene.add_material(String::from("emission_2"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.8, 0.4)));
    scene.add_material(String::from("emission_3"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.4, 0.8)));
    scene.add_material(String::from("emission_white"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.8, 0.8, 0.8)));
    scene.add_material(String::from("emission_red"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.95, 0.1, 0.1)));
    scene.add_material(String::from("emission_green"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.1, 0.95, 0.1)));
    scene.add_material(String::from("emission_blue"), materials::Emission::create(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.1, 0.1, 0.95)));

    //Diffuse
    scene.add_material(String::from("diffuse_white"), materials::Diffuse::create(Vector3::new(0.9, 0.9, 0.9)));
    scene.add_material(String::from("diffuse_red"), materials::Diffuse::create(Vector3::new(0.9, 0.1, 0.1)));
    scene.add_material(String::from("diffuse_green"), materials::Diffuse::create(Vector3::new(0.1, 0.9, 0.1)));
    scene.add_material(String::from("diffuse_blue"), materials::Diffuse::create(Vector3::new(0.1, 0.1, 0.9)));
    scene.add_material(String::from("diffuse_yellow"), materials::Diffuse::create(Vector3::new(0.9, 0.9, 0.0)));
    scene.add_material(String::from("diffuse_black"), materials::Diffuse::create(Vector3::new(0.1, 0.1, 0.1)));
    scene.add_material(String::from("diffuse_dark_gray"), materials::Diffuse::create(Vector3::new(0.3, 0.3, 0.3)));

    //Metal
    scene.add_material(String::from("metal_red_fuzz"), materials::Metal::create(Vector3::new(1.0, 0.45, 0.45), 0.7));
    scene.add_material(String::from("metal_gray_fuzz"), materials::Metal::create(Vector3::new(0.7, 0.7, 0.7), 0.3));
    scene.add_material(String::from("metal_silver"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.05));
    scene.add_material(String::from("metal_silver_fuzz_0.2"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.2));
    scene.add_material(String::from("metal_silver_fuzz_0.4"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.4));
    scene.add_material(String::from("metal_silver_fuzz_0.6"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.6));
    scene.add_material(String::from("metal_silver_fuzz_0.8"), materials::Metal::create(Vector3::new(0.9, 0.9, 0.9), 0.8));

    //Glass
    scene.add_material(String::from("glass_diamond"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 2.4));
    scene.add_material(String::from("glass_glass"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 1.8));
    scene.add_material(String::from("glass_r_1.0"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 1.0));
    scene.add_material(String::from("glass_r_1.4"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 1.4));
    scene.add_material(String::from("glass_r_1.8"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 1.8));
    scene.add_material(String::from("glass_r_2.2"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 2.2));
    scene.add_material(String::from("glass_r_2.6"), materials::Glass::create(Vector3::new(1.0, 1.0, 1.0), 2.6));
}