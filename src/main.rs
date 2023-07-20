#![allow(dead_code)]
//use std::io;

mod window;
mod scene;
mod render;
mod intersection;
mod animation;
mod vector3;

use std::time::Instant;
use crate::vector3::Vector3;

fn main() {
    let window = window::Window{width:1920, height:1080,};
    let mut main_scene = scene::load_scene_2();
    let now = Instant::now();

    let raytracing_config = render::raytracing_config::RaytracingConfig{
        exposure:1.0, gamma:2.2, rays_per_pixel:0, ray_bounce_max_depth:5, convergence_threshold:0.2, 
        parallel:true, denoise:true, denoise_with_normals:true};

    pub enum RenderOption{
        Image,
        BlurAnimation,
        CameraRotationAnimation,
        MetalFuzzAnimation,
        LightTunnelAnimation,
    }

    let render_option = RenderOption::LightTunnelAnimation;
    match render_option{
        RenderOption::Image =>{
            //Render image
            println!("Rendering image.");
            let img = render::render(&main_scene, window.width, window.height, raytracing_config);
            img.save("output.png").unwrap();
        },
        RenderOption::BlurAnimation =>{
            //Blur animation
            let focus_dist = main_scene.main_camera.focus_dist - 0.5;
            animation::render_blur_transition(&window, raytracing_config, &mut main_scene, 10, focus_dist, focus_dist, 0.5, 0.0);
        },
        RenderOption::CameraRotationAnimation =>{
            //Camera rotation aimation
            animation::render_camera_rotation(&window, raytracing_config, &mut main_scene, 60, 10.0);
        },
        RenderOption::MetalFuzzAnimation =>{
            //Metal fuzz animation
            let obj_id = 8;
            animation::render_metal_fuzz_animation(&window, raytracing_config, &mut main_scene, 10, 1, obj_id , 0.0, 0.0);
        },
        RenderOption::LightTunnelAnimation =>{
            let camera_starting_position = Vector3::new(60.0, 8.0, 0.0);
            let camera_final_position = Vector3::new(5.0, 8.0, 0.0);
            animation::render_light_tunnel_animation(&window, raytracing_config, &mut main_scene, 20, 60,
                 2.0, 5.0, camera_starting_position, camera_final_position, 4)
        },
    }

    let render_time = now.elapsed().as_millis();
    println!("Render time: {}.{} seconds.", render_time / 1000, render_time % 1000);
}