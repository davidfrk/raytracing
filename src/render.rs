use crate::scene;
use scene::Scene;

use image::{ImageBuffer, RgbImage, Rgb};
//extern crate nalgebra as na;
//use na::Vector3;
use crate::vector3::Vector3;
use rand::Rng;
use crate::intersection::Ray;

mod raytracing;
pub mod raytracing_config;
use raytracing_config::RaytracingConfig;

use std::sync::{Arc, Mutex};
use rayon::prelude::*;

extern crate oidn;

pub fn render(scene:&Scene, width:u32, height:u32, raytracing_config:RaytracingConfig) -> RgbImage{
    //ImageBuffer<Rgb<u8>, Vec<u8>>
    let arc_img = Arc::new(Mutex::new(ImageBuffer::<Rgb<f32>, Vec<f32>>::new(width, height)));
    let arc_normal = Arc::new(Mutex::new(ImageBuffer::<Rgb<f32>, Vec<f32>>::new(width, height)));
    let arc_albedo = Arc::new(Mutex::new(ImageBuffer::<Rgb<f32>, Vec<f32>>::new(width, height)));

    //Get camera focus and blur
    let focus_distance = scene.main_camera.focus_dist;
    let focus_blur = scene.main_camera.focus_blur;

    //Calculate aspect ratio and field of view
    let image_aspect_ratio = width as f64 / height as f64;
    let camera_height = (scene.main_camera.fov / 2.0 * std::f64::consts::PI / 180.0).tan();
    let camera_width = image_aspect_ratio * camera_height;

    //Camera vectors
    let origin = scene.main_camera.position;
    let forward = scene.main_camera.forward;
    let right = scene.main_camera.right;
    let up = scene.main_camera.up;

    fn render_line(pixel_y:u32, height: u32, width: u32, origin: Vector3, forward: Vector3, right: Vector3, up: Vector3,
        camera_width: f64, camera_height: f64, focus_distance: f64, focus_blur: f64, scene: &Scene,
        raytracing_config:RaytracingConfig, img: &Arc<Mutex<ImageBuffer<Rgb<f32>, Vec<f32>>>>,
        normals: &Arc<Mutex<ImageBuffer<Rgb<f32>, Vec<f32>>>>, albedos: &Arc<Mutex<ImageBuffer<Rgb<f32>, Vec<f32>>>>){
        
        let mut rng = rand::thread_rng();
        for pixel_x in 0..width{
            let mut color:Vector3 = Vector3::new(0.0, 0.0, 0.0);
            let mut normal:Vector3 = Vector3::default();
            let mut albedo:Vector3 = Vector3::default();
            
            //attention to detail algorithm
            let mut prev_color = color;
            let mut current_ray_count = 0;
            let mut last_pixel_update = 0;
            
            //for _i in 0..rays_per_pixel{ //fixed ray count per pixel
            while !did_converge(&mut prev_color, &color, &mut last_pixel_update, current_ray_count, raytracing_config.convergence_threshold){ //attention to detail algorithm
                let mut pixel_x = pixel_x as f64;
                let mut pixel_y = pixel_y as f64;
                
                if raytracing_config.rays_per_pixel == 1{
                    pixel_x +=  0.5;
                    pixel_y +=  0.5;
                }else{
                    pixel_x +=  rng.gen::<f64>();
                    pixel_y +=  rng.gen::<f64>();
                }

                //Pixel coordinates in NDC space
                let pixel_ndc_x = pixel_x / width as f64;
                let pixel_ndc_y = pixel_y / height as f64;

                //Pixel screen coordinates
                let pixel_screen_x = 2.0 * pixel_ndc_x - 1.0;
                let pixel_screen_y = 1.0 - 2.0 * pixel_ndc_y;

                //Pixel camera coordinates
                let pixel_camera_x = pixel_screen_x * camera_width;
                let pixel_camera_y = pixel_screen_y * camera_height;

                //World space direction
                let mut ray_direction = (forward + pixel_camera_y * up + pixel_camera_x * right).normalize();

                //Focus blur
                let focus_point = focus_distance * ray_direction + origin;
                let mut offset = random_in_unit_disk();
                offset = focus_blur * (offset.x * right + offset.y * up);
                let blur_origin = origin + offset;
                ray_direction = (focus_point - blur_origin).normalize();

                //Create Ray
                let ray = Ray{
                    origin: blur_origin,
                    direction: ray_direction,
                };

                //Cast Ray
                //color += raytracing::cast_ray(&scene, &ray, raytracing_config.ray_bounce_max_depth);

                let mut new_normal:Vector3 = Vector3::default();
                let mut new_albedo:Vector3 = Vector3::default();
                color += raytracing::cast_ray_with_normal_albedo(&scene, &ray, raytracing_config.ray_bounce_max_depth, &mut new_normal, &mut new_albedo);
                normal += new_normal;
                albedo += new_albedo;

                current_ray_count+= 1; //attention to detail algorithm
            }

            //color = 1.0 / rays_per_pixel as f64 * color; // fixed ray count per pixel
            color = 1.0 / current_ray_count as f64 * color; // attention to detail algorithm
            normal = 1.0 / current_ray_count as f64 * normal;
            albedo = 1.0 / current_ray_count as f64 * albedo;

            //Gamma correction and clamp
            color = raytracing_config.exposure * color;
            color.x = (f64::powf(color.x, raytracing_config.gamma)).clamp(0.0, 1.0);
            color.y = (f64::powf(color.y, raytracing_config.gamma)).clamp(0.0, 1.0);
            color.z = (f64::powf(color.z, raytracing_config.gamma)).clamp(0.0, 1.0);

            //Writing pixel
            //let r = (color.x * 255.0).floor() as u8;
            //let g = (color.y * 255.0).floor() as u8;
            //let b = (color.z * 255.0).floor() as u8;
            //let rgb = image::Rgb([r, g, b]);
            let rgb = image::Rgb([color.x as f32, color.y as f32, color.z as f32]);
            let normal_rgb = image::Rgb([normal.x as f32, normal.y as f32, normal.z as f32]);
            let albedo_rgb = image::Rgb([albedo.x as f32, albedo.y as f32, albedo.z as f32]);
            
            img.lock().unwrap().put_pixel(pixel_x, pixel_y, rgb);
            normals.lock().unwrap().put_pixel(pixel_x, pixel_y, normal_rgb);
            albedos.lock().unwrap().put_pixel(pixel_x, pixel_y, albedo_rgb);
        }
        if pixel_y % 50 == 0 {
            println!("Line: {}", pixel_y);
        }
    }

    //Render it
    if raytracing_config.parallel{
        //Render in parallel with rayon
        (0..height).into_par_iter().for_each( | line | render_line(line, height, width, origin, forward, right, up, camera_width, camera_height,
            focus_distance, focus_blur, scene, raytracing_config, &arc_img, &arc_normal, &arc_albedo));
    }else{
        for line in 0..height{
            render_line(line, height, width, origin, forward, right, up, camera_width, camera_height,
                focus_distance, focus_blur, scene, raytracing_config, &arc_img, &arc_normal, &arc_albedo);
        }
    }

    let img = &*arc_img.lock().unwrap();
    let mut final_image = img.clone();

    if raytracing_config.denoise{
        let normals = (&*arc_normal.lock().unwrap()).clone();
        let albedos = (&*arc_albedo.lock().unwrap()).clone();

        to_rgb(&normals).save("normal.png").unwrap();
        to_rgb(&albedos).save("albedo.png").unwrap();

        denoise(&mut final_image, normals, albedos, raytracing_config);
    }
    
    return to_rgb(&final_image);
}

fn denoise(img: &mut ImageBuffer::<Rgb<f32>, Vec<f32>>, normal: ImageBuffer::<Rgb<f32>, Vec<f32>>, albedo: ImageBuffer::<Rgb<f32>, Vec<f32>>, raytracing_config: RaytracingConfig){
    let (width, height) = img.dimensions();
    let num_pixels = (width * height) as usize;

    let mut converted_pixels: Vec<f32> = Vec::with_capacity(num_pixels * 3); // 3 channels (R, G, B) per pixel
    let mut converted_normals: Vec<f32> = Vec::with_capacity(num_pixels * 3);
    let mut converted_albedos: Vec<f32> = Vec::with_capacity(num_pixels * 3);

    for pixel in img.pixels() {
        converted_pixels.push(pixel[0]);
        converted_pixels.push(pixel[1]);
        converted_pixels.push(pixel[2]);
    }

    for pixel in normal.pixels() {
        converted_normals.push(pixel[0]);
        converted_normals.push(pixel[1]);
        converted_normals.push(pixel[2]);
    }

    for pixel in albedo.pixels() {
        converted_albedos.push(pixel[0]);
        converted_albedos.push(pixel[1]);
        converted_albedos.push(pixel[2]);
    }

    // Ensure the capacity matches the actual number of converted pixels
    converted_pixels.shrink_to_fit();
    converted_normals.shrink_to_fit();
    converted_albedos.shrink_to_fit();

    let mut filter_output = vec![0.0f32; converted_pixels.len()];
    
    let device = oidn::Device::new();
    
    let mut filter = oidn::RayTracing::new(&device);
    
    filter
        //.srgb(true)
        .image_dimensions(width as usize, height as usize);
        //.input_scale(1.0_f32);

    if raytracing_config.denoise_with_normals{
        filter.albedo_normal(&converted_albedos, &converted_normals);
    }
    
    filter
        .filter(&converted_pixels[..], &mut filter_output[..])
        .expect("Invalid input image dimensions?");
    

    if let Err(e) = device.get_error() {
        println!("Error denosing image: {}", e.1);
    }

    //To do, transfer filter_output back to img
    for i in (0..filter_output.len()).filter(|x| (x % 3) == 0) {
        let rgb = image::Rgb([filter_output[i], filter_output[i + 1], filter_output[i + 2]]);

        let pixel = i / 3;
        let x = (pixel % width as usize) as u32;
        let y = (pixel / width as usize) as u32;
        img.put_pixel(x, y, rgb);
    }
}

fn to_rgb(img: &ImageBuffer::<Rgb<f32>, Vec<f32>>) -> ImageBuffer::<Rgb<u8>, Vec<u8>>{
    let mut rgb_image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(img.width(), img.height());

    for y in 0..img.height(){
        for x in 0..img.width(){
            let pixel = img.get_pixel(x, y);

            let r = (pixel[0] * 255.0).floor() as u8;
            let g = (pixel[1] * 255.0).floor() as u8;
            let b = (pixel[2] * 255.0).floor() as u8;
            let rgb = image::Rgb([r, g, b]);

            rgb_image.put_pixel(x, y, rgb);
        }
    }

    return rgb_image;
}

fn did_converge(last_color:&mut Vector3, color:& Vector3, last_update:&mut u32, current_count:u32, convergence_threshold:f64) -> bool{
    if *last_update + 20 > current_count{
        return false;
    }
    let diff = *last_color / *last_update as f64 - color / current_count as f64;
    let abs_diff = diff.x.abs() + diff.y.abs() + diff.z.abs();
    //let norm = diff.norm();

    //if norm < 0.001{
    if abs_diff < convergence_threshold{
        return true;
    }else{
        *last_color = *color;
        *last_update = current_count;
        /*
        if current_count > 100{
            println!("{}", current_count);
        }*/
        return false;
    }
}

fn random_in_unit_disk() -> Vector3{
    let mut rng = rand::thread_rng();
    loop{
        let x = rng.gen_range(-1.0 .. 1.0);
        let y = rng.gen_range(-1.0 .. 1.0);
        if (x*x + y*y) <= 1.0{
            return Vector3::new(x, y, 0.0);
        }
    }
}