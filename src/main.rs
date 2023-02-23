//use std::io;

mod window;
mod scene;
mod render;
mod intersection;
mod animation;

use std::time::{Instant};

fn main() {
    let window = window::Window{width:800, height:800,};
    let main_scene = scene::load_scene();
    let now = Instant::now();

    //Render image
    println!("Rendering image.");
    let img = render::render(&main_scene, window.width, window.height);
    img.save("output.png").unwrap();

    //Blur animation
    //let focus_dist = main_scene.main_camera.focus_dist - 0.5;
    //animation::render_blur_transition(&window, &mut main_scene, 10, focus_dist, focus_dist, 0.5, 0.0);

    //Camera rotation aimation
    //animation::render_camera_rotation(&window, &mut main_scene, 60, 10.0);

    //Metal fuzz animation
    //let obj_id = 8;
    //animation::render_metal_fuzz_animation(&window, &mut main_scene, 10, 1, obj_id , 0.0, 0.0);

    let render_time = now.elapsed().as_secs();
    println!("Render time: {}", render_time);
}