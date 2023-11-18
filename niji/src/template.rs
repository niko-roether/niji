use std::{collections::HashMap, hash::Hash, path::Path};

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
	Failed(String, mustache::Error)
}

#[derive(Debug, Error)]
pub enum RenderError {
	#[error("Failed to render template {0}")]
	Mustache(#[from] mustache::Error)
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

pub trait TemplateData {
	fn to_data(&self, ctx: &TemplateContext) -> FmtResult<mustache::Data>;
}

impl<V> TemplateData for V
where
	V: FmtValue
{
	fn to_data(&self, ctx: &TemplateContext) -> FmtResult<mustache::Data> {
		let fmtstr = ctx
			.fmt
			.get(self.type_name())
			.map(String::as_str)
			.unwrap_or(self.default_fmt());

		self.format(fmtstr).map(mustache::Data::String)
	}
}

impl TemplateData for bool {
	fn to_data(&self, _: &TemplateContext) -> FmtResult<mustache::Data> {
		Ok(mustache::Data::Bool(*self))
	}
}

impl<T> TemplateData for Vec<T>
where
	T: TemplateData
{
	fn to_data(&self, ctx: &TemplateContext) -> FmtResult<mustache::Data> {
		let mut data_vec = Vec::with_capacity(self.len());
		for value in self {
			data_vec.push(value.to_data(ctx)?);
		}
		Ok(mustache::Data::Vec(data_vec))
	}
}

impl<V> TemplateData for HashMap<String, V>
where
	V: TemplateData
{
	fn to_data(&self, ctx: &TemplateContext) -> FmtResult<mustache::Data> {
		let mut data_map = HashMap::new();
		for (key, value) in self {
			data_map.insert(key.to_string(), value.to_data(ctx)?);
		}
		Ok(mustache::Data::Map(data_map))
	}
}

impl TemplateData for Box<dyn TemplateData> {
	fn to_data(&self, ctx: &TemplateContext) -> FmtResult<mustache::Data> {
		self.as_ref().to_data(ctx)
	}
}

#[derive(Debug, Clone)]
pub struct Template {
	ctx: TemplateContext,
	template: mustache::Template
}

impl Template {
	pub fn load<P>(path: P) -> Result<Self, InitError>
	where
		P: AsRef<Path>
	{
		let template = mustache::compile_path(path.as_ref())
			.map_err(|e| InitError::Failed(path.as_ref().to_string_lossy().into_owned(), e))?;

		Ok(Self {
			ctx: TemplateContext::new(),
			template
		})
	}

	pub fn parse(template: &str) -> Result<Self, InitError> {
		let template = mustache::compile_str(template)
			.map_err(|e| InitError::Failed(String::from("inline template"), e))?;

		Ok(Self {
			ctx: TemplateContext::new(),
			template
		})
	}

	pub fn format_as(&mut self, ty: &str, fmtstr: String) -> FmtResult<()> {
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

#[derive(Debug)]
pub struct TemplateRenderer<'a> {
	template: &'a Template,
	data_map: HashMap<String, mustache::Data>
}

impl<'a> TemplateRenderer<'a> {
	pub fn set_value(&mut self, name: String, value: impl TemplateData) -> FmtResult<()> {
		let data = value.to_data(&self.template.ctx)?;
		self.data_map.insert(name, data);

		Ok(())
	}

	pub fn render(self) -> Result<String, RenderError> {
		let string = self
			.template
			.template
			.render_data_to_string(&mustache::Data::Map(self.data_map))?;
		Ok(string)
	}
}
