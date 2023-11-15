pub mod fs;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
	a + t * (b - a)
}
