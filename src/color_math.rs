use ultraviolet as uv;

use crate::types::color::Color;

const SRGB_BREAK_X: f32 = 0.0031308;
const SRGB_BREAK_Y: f32 = 0.04045;
const SRGB_A: f32 = 0.005;
const SRGB_GAMMA: f32 = 2.4;
const SRGB_THETA: f32 = 12.92;

fn srgb_oetf(x: f32) -> f32 {
	let y = if x < SRGB_BREAK_X {
		SRGB_THETA * x
	} else {
		(1.0 + SRGB_A) * x.powf(1.0 / SRGB_GAMMA) - SRGB_A
	};

	f32::min(f32::max(y, 0.0), 1.0)
}

fn srgb_eotf(y: f32) -> f32 {
	if y < SRGB_BREAK_Y {
		y / SRGB_THETA
	} else {
		((y * SRGB_A) / (1.0 + SRGB_A)).powf(SRGB_GAMMA)
	}
}

const LSRGB_LSB_MAT: uv::Mat3 = uv::Mat3::new(
	uv::Vec3::new(0.41222147, 0.2119035, 0.08830246),
	uv::Vec3::new(0.53633254, 0.6806995, 0.28171884),
	uv::Vec3::new(0.05144599, 0.10736957, 0.6299787)
);

const LSB_OKLAB_MAT: uv::Mat3 = uv::Mat3::new(
	uv::Vec3::new(0.21045426, 1.9779985, 0.02590404),
	uv::Vec3::new(0.7936178, -2.4285922, 0.78277177),
	uv::Vec3::new(-0.00407205, 0.45059337, -0.80867577)
);

const OKLAB_LSB_MAT: uv::Mat3 = uv::Mat3::new(
	uv::Vec3::new(1.0, 1.0, 1.0),
	uv::Vec3::new(0.39633778, -0.10556135, -0.08948418),
	uv::Vec3::new(0.21580376, -0.06385417, -1.2914856)
);

const LSB_LSRGB_MAT: uv::Mat3 = uv::Mat3::new(
	uv::Vec3::new(4.0767417, -1.268438, -0.00419609),
	uv::Vec3::new(-3.3077116, 2.6097574, -0.7034186),
	uv::Vec3::new(0.23096993, -0.3413194, 1.7076147)
);

#[derive(Debug, Clone)]
pub struct OklabColor {
	lab: uv::Vec3,
	alpha: f32
}

impl OklabColor {
	pub fn lighten(&mut self, amount: f32) {
		self.lab.x = f32::min(self.lab.x + amount, 100.0);
	}

	pub fn darken(&mut self, amount: f32) {
		self.lab.x = f32::max(self.lab.x - amount, 0.0);
	}
}

impl From<Color> for OklabColor {
	fn from(value: Color) -> Self {
		let mut lsrgb = uv::Vec3::new(value.r as f32, value.g as f32, value.b as f32) / 255.0;
		lsrgb.apply(srgb_eotf);

		let alpha = value.a as f32 / 255.0;

		let mut lsb = LSRGB_LSB_MAT * lsrgb;
		lsb.apply(f32::cbrt);

		let lab = LSB_OKLAB_MAT * lsb;

		Self { lab, alpha }
	}
}

impl From<OklabColor> for Color {
	fn from(value: OklabColor) -> Self {
		let mut lsb = OKLAB_LSB_MAT * value.lab;
		lsb.apply(|x| x.powi(3));

		let mut srgb = LSB_LSRGB_MAT * lsb;
		srgb.apply(srgb_oetf);

		Color::new_rgba(
			(255.0 * srgb.x.clamp(0.0, 1.0)) as u8,
			(255.0 * srgb.y.clamp(0.0, 1.0)) as u8,
			(255.0 * srgb.z.clamp(0.0, 1.0)) as u8,
			(255.0 * value.alpha.clamp(0.0, 1.0)) as u8
		)
	}
}
