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

const LSRGB_LMS_MAT: uv::Mat3 = uv::Mat3::new(
	uv::Vec3::new(0.41222147, 0.2119035, 0.08830246),
	uv::Vec3::new(0.53633254, 0.6806995, 0.28171884),
	uv::Vec3::new(0.05144599, 0.10736957, 0.6299787)
);

const LMS_OKLAB_MAT: uv::Mat3 = uv::Mat3::new(
	uv::Vec3::new(0.21045426, 1.9779985, 0.02590404),
	uv::Vec3::new(0.7936178, -2.4285922, 0.78277177),
	uv::Vec3::new(-0.00407205, 0.45059337, -0.80867577)
);

const OKLAB_LMS_MAT: uv::Mat3 = uv::Mat3::new(
	uv::Vec3::new(1.0, 1.0, 1.0),
	uv::Vec3::new(0.39633778, -0.10556135, -0.08948418),
	uv::Vec3::new(0.21580376, -0.06385417, -1.2914856)
);

const LMS_LSRGB_MAT: uv::Mat3 = uv::Mat3::new(
	uv::Vec3::new(4.0767417, -1.268438, -0.00419609),
	uv::Vec3::new(-3.3077116, 2.6097574, -0.7034186),
	uv::Vec3::new(0.23096993, -0.3413194, 1.7076147)
);

#[derive(Debug, Clone, Copy)]
pub struct OklchColor {
	l: f32,
	c: f32,
	h: f32
}

impl OklchColor {
	pub fn new(l: f32, c: f32, h: f32) -> Self {
		Self { l, c, h }
	}

	pub fn from_srgb(r: f32, g: f32, b: f32) -> Self {
		let mut rgb = uv::Vec3::new(r, g, b);
		rgb.apply(srgb_eotf); // linearize components

		let mut lms = LSRGB_LMS_MAT * rgb;
		lms.apply(f32::cbrt); // nonlinear transform

		let lab = LMS_OKLAB_MAT * lms;

		// Convert to polar representation
		let c = f32::sqrt(lab.y.powi(2) + lab.z.powi(2));
		let h = f32::atan2(lab.y, lab.z);

		Self::new(lab.x, c, h)
	}

	pub fn into_srgb(self) -> (f32, f32, f32) {
		// Convert to cartesian representation
		let a = self.c * f32::cos(self.h);
		let b = self.c * f32::sin(self.h);
		let lab = uv::Vec3::new(self.l, a, b);

		let mut lms = OKLAB_LMS_MAT * lab;
		lms.apply(|c| c.powi(3)); // Inverse nonlinear transform

		let mut rgb = LMS_LSRGB_MAT * lms;
		rgb.apply(srgb_oetf); // Gamma-compress components
		rgb.apply(|c| c.clamp(0.0, 1.0)); // Clamp to fit into sRGB gamut

		(rgb.x, rgb.y, rgb.z)
	}

	#[inline]
	pub fn lightness(self) -> f32 {
		self.l
	}

	#[inline]
	pub fn chroma(self) -> f32 {
		self.c
	}

	#[inline]
	pub fn hue(self) -> f32 {
		self.h
	}

	pub fn shade(self, lightness: f32) -> Self {
		let mut result = self;
		result.l = lightness;
		result
	}

	pub fn lighten(self, amount: f32) -> Self {
		self.shade(self.lightness() + amount)
	}

	pub fn darken(self, amount: f32) -> Self {
		self.shade(self.lightness() - amount)
	}

	pub fn blend(col1: Self, col2: Self, t: f32) -> Self {
		Self::new(
			lerp(col1.lightness(), col2.lightness(), t),
			lerp(col1.chroma(), col2.chroma(), t),
			lerp(col1.hue(), col2.hue(), t)
		)
	}
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
	a + t * (b - a)
}
