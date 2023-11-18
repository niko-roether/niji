use std::{collections::HashMap, fs, io, path::Path};

use strfmt::{strfmt_map, DisplayStr};
use thiserror::Error;

use crate::types::color::Color;

#[derive(Debug, Error)]
pub enum FmtError {
	#[error("{0}")]
	Strfmt(#[from] strfmt::FmtError)
}

#[derive(Debug, Error)]
pub enum InitError {
	#[error("Failed to load {0}: {1}")]
	Load(String, io::Error),

	#[error("Failed to parse the template: {0}")]
	Parse(ramhorns::Error)
}

#[derive(Debug, Error)]
pub enum RenderError {
	#[error("Failed to render template: {0}")]
	Mustache(#[from] ramhorns::Error)
}

pub type FmtResult<T> = Result<T, FmtError>;

pub trait FmtValue {
	fn type_name(&self) -> &'static str;

	fn default_fmt(&self) -> &'static str;

	fn get_placeholder(&self, name: &str) -> Option<Box<dyn DisplayStr>>;

	fn format(&self, fmtstr: &str) -> FmtResult<String> {
		Ok(strfmt_map(fmtstr, |mut fmt| {
			let value = self
				.get_placeholder(fmt.key)
				.ok_or(strfmt::FmtError::KeyError(fmt.key.to_string()))?;
			value.display_str(&mut fmt)?;

			Ok(())
		})?)
	}
}

macro_rules! basic_template_value {
	($name:literal, $($ty:ident),+) => {
        $(
            impl FmtValue for $ty {
                #[inline]
                fn type_name(&self) -> &'static str {
                    $name
                }

                #[inline]
                fn default_fmt(&self) -> &'static str {
                    concat!("{", $name, "}")
                }

                fn get_placeholder(&self, name: &str) -> Option<Box<dyn DisplayStr>> {
                    if name == $name {
                        Some(Box::new(self.clone()))
                    } else {
                        None
                    }
                }
            }
         )+
    };
}

basic_template_value!("string", String);
basic_template_value!("int", i8, u8, i16, u16, i32, u32, i64, u64);
basic_template_value!("float", f32, f64);

impl FmtValue for Color {
	fn type_name(&self) -> &'static str {
		"color"
	}

	fn default_fmt(&self) -> &'static str {
		"#{rx}{gx}{bx}{ax}"
	}

	fn get_placeholder(&self, name: &str) -> Option<Box<dyn DisplayStr>> {
		match name {
			"r" => Some(Box::new(self.r)),
			"g" => Some(Box::new(self.g)),
			"b" => Some(Box::new(self.b)),
			"a" => Some(Box::new(self.a)),
			"rx" => Some(Box::new(format!("{:02x}", self.r))),
			"gx" => Some(Box::new(format!("{:02x}", self.g))),
			"bx" => Some(Box::new(format!("{:02x}", self.b))),
			"ax" => Some(Box::new(format!("{:02x}", self.a))),
			"rf" => Some(Box::new(self.r as f32 / 255.0)),
			"gf" => Some(Box::new(self.g as f32 / 255.0)),
			"bf" => Some(Box::new(self.b as f32 / 255.0)),
			"af" => Some(Box::new(self.a as f32 / 255.0)),
			_ => None
		}
	}
}

#[derive(Debug, Clone)]
pub struct TemplateContext {
	fmt: HashMap<String, String>
}

impl TemplateContext {
	fn new() -> Self {
		Self {
			fmt: HashMap::new()
		}
	}
}

#[derive(Debug, Clone)]
pub enum TemplateData {
	Bool(bool),
	String(String),
	Vec(Vec<TemplateData>),
	Map(HashMap<String, TemplateData>)
}

macro_rules! template_data_delegate {
    ($self:ident.$name:ident($($arg:expr),*)) => {
        match $self {
            TemplateData::Bool(bool) => bool.$name($($arg),*),
            TemplateData::String(string) => string.$name($($arg),*),
            TemplateData::Vec(vec) => vec.$name($($arg),*),
            TemplateData::Map(map) => map.$name($($arg),*)
        }
    };
}

impl ramhorns::Content for TemplateData {
	#[inline]
	fn is_truthy(&self) -> bool {
		template_data_delegate!(self.is_truthy())
	}

	#[inline]
	fn capacity_hint(&self, tpl: &ramhorns::Template) -> usize {
		template_data_delegate!(self.capacity_hint(tpl))
	}

	#[inline]
	fn render_escaped<E: ramhorns::encoding::Encoder>(
		&self,
		encoder: &mut E
	) -> Result<(), E::Error> {
		template_data_delegate!(self.render_escaped(encoder))
	}

	#[inline]
	fn render_section<C, E>(
		&self,
		section: ramhorns::Section<C>,
		encoder: &mut E
	) -> Result<(), E::Error>
	where
		C: ramhorns::traits::ContentSequence,
		E: ramhorns::encoding::Encoder
	{
		template_data_delegate!(self.render_section(section, encoder))
	}

	#[inline]
	fn render_inverse<C, E>(
		&self,
		section: ramhorns::Section<C>,
		encoder: &mut E
	) -> Result<(), E::Error>
	where
		C: ramhorns::traits::ContentSequence,
		E: ramhorns::encoding::Encoder
	{
		template_data_delegate!(self.render_inverse(section, encoder))
	}

	#[inline]
	fn render_unescaped<E: ramhorns::encoding::Encoder>(
		&self,
		encoder: &mut E
	) -> Result<(), E::Error> {
		template_data_delegate!(self.render_unescaped(encoder))
	}

	#[inline]
	fn render_field_escaped<E: ramhorns::encoding::Encoder>(
		&self,
		hash: u64,
		name: &str,
		encoder: &mut E
	) -> Result<bool, E::Error> {
		template_data_delegate!(self.render_field_escaped(hash, name, encoder))
	}

	#[inline]
	fn render_field_section<C, E>(
		&self,
		hash: u64,
		name: &str,
		section: ramhorns::Section<C>,
		encoder: &mut E
	) -> Result<bool, E::Error>
	where
		C: ramhorns::traits::ContentSequence,
		E: ramhorns::encoding::Encoder
	{
		template_data_delegate!(self.render_field_section(hash, name, section, encoder))
	}

	#[inline]
	fn render_field_inverse<C, E>(
		&self,
		hash: u64,
		name: &str,
		section: ramhorns::Section<C>,
		encoder: &mut E
	) -> Result<bool, E::Error>
	where
		C: ramhorns::traits::ContentSequence,
		E: ramhorns::encoding::Encoder
	{
		template_data_delegate!(self.render_field_inverse(hash, name, section, encoder))
	}

	#[inline]
	fn render_field_unescaped<E: ramhorns::encoding::Encoder>(
		&self,
		hash: u64,
		name: &str,
		encoder: &mut E
	) -> Result<bool, E::Error> {
		template_data_delegate!(self.render_field_unescaped(hash, name, encoder))
	}
}

pub trait ToTemplateData {
	fn to_data(&self, ctx: &TemplateContext) -> FmtResult<TemplateData>;
}

impl<V> ToTemplateData for V
where
	V: FmtValue
{
	fn to_data(&self, ctx: &TemplateContext) -> FmtResult<TemplateData> {
		let fmtstr = ctx
			.fmt
			.get(self.type_name())
			.map(String::as_str)
			.unwrap_or(self.default_fmt());

		self.format(fmtstr).map(TemplateData::String)
	}
}

impl ToTemplateData for bool {
	fn to_data(&self, _: &TemplateContext) -> FmtResult<TemplateData> {
		Ok(TemplateData::Bool(*self))
	}
}

impl<T> ToTemplateData for Vec<T>
where
	T: ToTemplateData
{
	fn to_data(&self, ctx: &TemplateContext) -> FmtResult<TemplateData> {
		let mut data_vec = Vec::with_capacity(self.len());
		for value in self {
			data_vec.push(value.to_data(ctx)?);
		}
		Ok(TemplateData::Vec(data_vec))
	}
}

impl<V> ToTemplateData for HashMap<String, V>
where
	V: ToTemplateData
{
	fn to_data(&self, ctx: &TemplateContext) -> FmtResult<TemplateData> {
		let mut data_map = HashMap::new();
		for (key, value) in self {
			data_map.insert(key.to_string(), value.to_data(ctx)?);
		}
		Ok(TemplateData::Map(data_map))
	}
}

impl ToTemplateData for Box<dyn ToTemplateData> {
	fn to_data(&self, ctx: &TemplateContext) -> FmtResult<TemplateData> {
		self.as_ref().to_data(ctx)
	}
}

pub struct Template {
	ctx: TemplateContext,
	template: ramhorns::Template<'static>
}

impl Template {
	pub fn load<P>(path: P) -> Result<Self, InitError>
	where
		P: AsRef<Path>
	{
		let path_name = path.as_ref().display().to_string();
		let source = fs::read_to_string(path).map_err(|e| InitError::Load(path_name, e))?;

		Self::parse(source)
	}

	pub fn parse(source: String) -> Result<Self, InitError> {
		let template = ramhorns::Template::new(source).map_err(|e| InitError::Parse(e))?;

		Ok(Self {
			ctx: TemplateContext::new(),
			template
		})
	}

	pub fn set_format(&mut self, ty: &str, fmtstr: String) -> FmtResult<()> {
		self.ctx.fmt.insert(ty.to_string(), fmtstr);

		Ok(())
	}

	pub fn renderer(&self) -> TemplateRenderer {
		TemplateRenderer {
			template: self,
			data_map: HashMap::new()
		}
	}
}

pub struct TemplateRenderer<'a> {
	template: &'a Template,
	data_map: HashMap<String, TemplateData>
}

impl<'a> TemplateRenderer<'a> {
	pub fn set_value(&mut self, name: String, value: impl ToTemplateData) -> FmtResult<()> {
		let data = value.to_data(&self.template.ctx)?;
		self.data_map.insert(name, data);

		Ok(())
	}

	pub fn render(self) -> String {
		self.template.template.render(&self.data_map)
	}
}
