
#[derive(Copy, Clone)]
pub struct RaytracingConfig{
	//Color correction
	pub exposure:f64,
	pub gamma:f64,

	//Anti-aliasing and convergency
	pub rays_per_pixel:u32,
	pub ray_bounce_max_depth:u8,
	pub convergence_threshold:f64,
}