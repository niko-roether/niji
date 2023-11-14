use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{fmt, mem::transmute, num::ParseIntError, str::FromStr};
use thiserror::Error;

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

#[derive(Debug, Clone, Copy, PartialEq, SerializeDisplay, DeserializeFromStr)]
#[repr(C, align(4))]
pub struct Color {
	r: f32,
	g: f32,
	b: f32,
	a: f32
}

impl Color {
	pub fn new_linear_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
		Self { r, g, b, a }
	}

	pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self::new_linear_rgba(
			srgb_eotf(r as f32 / 255.0),
			srgb_eotf(g as f32 / 255.0),
			srgb_eotf(b as f32 / 255.0),
			a as f32 / 255.0
		)
	}

	#[inline]
	pub fn linear_r(self) -> f32 {
		self.r
	}

	#[inline]
	pub fn linear_g(self) -> f32 {
		self.g
	}

	#[inline]
	pub fn linear_b(self) -> f32 {
		self.b
	}

	#[inline]
	pub fn alpha(self) -> f32 {
		self.a
	}

	pub fn r(self) -> u8 {
		(255.0 * srgb_oetf(self.r).clamp(0.0, 1.0)) as u8
	}

	pub fn g(self) -> u8 {
		(255.0 * srgb_oetf(self.g).clamp(0.0, 1.0)) as u8
	}

	pub fn b(self) -> u8 {
		(255.0 * srgb_oetf(self.b).clamp(0.0, 1.0)) as u8
	}

	pub fn a(self) -> u8 {
		(255.0 * self.a).clamp(0.0, 1.0) as u8
	}
}

impl Default for Color {
	fn default() -> Self {
		Self::from(0x000000ff)
	}
}

impl From<u32> for Color {
	fn from(value: u32) -> Self {
		Self::new_rgba(
			(value >> 24) as u8,
			((value >> 16) & 0xff) as u8,
			((value >> 8) & 0xff) as u8,
			(value & 0xff) as u8
		)
	}
}

impl From<Color> for u32 {
	fn from(value: Color) -> Self {
		value.a() as u32
			| (value.b() as u32) << 8
			| (value.g() as u32) << 16
			| (value.r() as u32) << 24
	}
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "#{:08x}", u32::from(*self))
	}
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ColorParseError {
	#[error("Color strings must start with a '#'! (got \"{0}\")")]
	MissingHashtag(String),

	#[error("\"{0}\" is not a valid hexadecimal number: {1}")]
	InvalidHexNumber(String, ParseIntError),

	#[error("Colors must have 3, 6, or 8 hex digits! (got {0})")]
	InvalidLength(usize)
}

impl FromStr for Color {
	type Err = ColorParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let Some(s) = s.strip_prefix('#') else {
			return Err(ColorParseError::MissingHashtag(s.to_string()));
		};

		let parsed_num: u32 = u32::from_str_radix(s, 16)
			.map_err(|err| ColorParseError::InvalidHexNumber(s.to_string(), err))?;

		let col: u32 = match s.len() {
			3 => (parsed_num << 20) | (parsed_num << 8) | 0xff,
			6 => parsed_num << 8 | 0xff,
			8 => parsed_num,
			_ => return Err(ColorParseError::InvalidLength(s.len()))
		};

		Ok(Self::from(col))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn should_construct_from_int() {
		let col = Color::from(0x0a0b0c0d);

		assert_eq!(col.r(), 0x0a);
		assert_eq!(col.g(), 0x0b);
		assert_eq!(col.b(), 0x0c);
		assert_eq!(col.a(), 0x0d);
	}

	#[test]
	fn should_display_correctly() {
		let col = Color::from(0xf10234ff);

		assert_eq!(col.to_string(), String::from("#f10234ff"))
	}

	#[test]
	fn should_parse_3_len() {
		assert_eq!(Color::from_str("#222"), Ok(Color::from(0x222222ff)))
	}

	#[test]
	fn should_parse_6_len() {
		assert_eq!(Color::from_str("#abcdef"), Ok(Color::from(0xabcdefff)));
	}

	#[test]
	fn should_parse_8_len() {
		assert_eq!(Color::from_str("#abcdef80"), Ok(Color::from(0xabcdef80)));
	}
}
