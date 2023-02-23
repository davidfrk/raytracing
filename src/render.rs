use crate::scene;
use scene::Scene;

use image::{ImageBuffer, RgbImage};
extern crate nalgebra as na;
use na::Vector3;
use rand::Rng;
use crate::intersection::Ray;

mod raytracing;

pub fn render(scene:&Scene, width:u32, height:u32) -> RgbImage{
    let mut img:RgbImage = ImageBuffer::new(width, height);
    //Color correction
    let exposure = 1.0;
    let gamma = 2.2;

    //Anti-aliasing
    let rays_per_pixel = 400;
    let rays = 1;
    let depth = 12;

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

    let mut rng = rand::thread_rng();

    //render it
    for pixel_y in 0..height{
        for pixel_x in 0..width{
            let mut color = Vector3::new(0.0, 0.0, 0.0);

            for _i in 0..rays_per_pixel{
                let mut pixel_x = pixel_x as f64;
                let mut pixel_y = pixel_y as f64;
                
                if rays_per_pixel == 1{
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
                color += raytracing::cast_ray(&scene, &ray, rays, depth);
            }
            
            color = 1.0 / rays_per_pixel as f64 * color;

            //Gamma correction and clamp
            color = exposure * color;
            color.x = (f64::powf(color.x, gamma)).clamp(0.0, 1.0);
            color.y = (f64::powf(color.y, gamma)).clamp(0.0, 1.0);
            color.z = (f64::powf(color.z, gamma)).clamp(0.0, 1.0);

            //Writing pixel
            let r = (color.x * 255.0).floor() as u8;
            let g = (color.y * 255.0).floor() as u8;
            let b = (color.z * 255.0).floor() as u8;
            let rgb = image::Rgb([r, g, b]);
            
            img.put_pixel(pixel_x, pixel_y, rgb);
        }
        println!("Line: {}", pixel_y);
    }

    return img;
}

fn random_in_unit_disk() -> Vector3<f64>{
    let mut rng = rand::thread_rng();
    loop{
        let x = rng.gen_range(-1.0 .. 1.0);
        let y = rng.gen_range(-1.0 .. 1.0);
        if (x*x + y*y) <= 1.0{
            return Vector3::new(x, y, 0.0);
        }
    }
}