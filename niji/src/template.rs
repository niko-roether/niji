use std::{collections::HashMap, path::Path};

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

pub trait TemplateValue {
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

impl TemplateValue for Box<dyn TemplateValue> {
	fn type_name(&self) -> &'static str {
		self.as_ref().type_name()
	}

	fn default_fmt(&self) -> &'static str {
		self.as_ref().default_fmt()
	}

	fn get_placeholder(&self, name: &str) -> Option<Box<dyn DisplayStr>> {
		self.as_ref().get_placeholder(name)
	}
}

macro_rules! basic_template_value {
	($name:literal, $($ty:ident),+) => {
        $(
            impl TemplateValue for $ty {
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

impl TemplateValue for bool {
	fn type_name(&self) -> &'static str {
		"bool"
	}

	fn default_fmt(&self) -> &'static str {
		"{true/false}"
	}

	fn get_placeholder(&self, name: &str) -> Option<Box<dyn DisplayStr>> {
		let (when_true, when_false) = name.split_once('/')?;

		if *self {
			Some(Box::new(when_true.to_string()))
		} else {
			Some(Box::new(when_false.to_string()))
		}
	}
}

impl TemplateValue for Color {
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
pub struct Template {
	fmt: HashMap<String, String>,
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
			fmt: HashMap::new(),
			template
		})
	}

	pub fn parse(template: &str) -> Result<Self, InitError> {
		let template = mustache::compile_str(template)
			.map_err(|e| InitError::Failed(String::from("inline template"), e))?;

		Ok(Self {
			fmt: HashMap::new(),
			template
		})
	}

	pub fn format_as(&mut self, ty: &str, fmtstr: String) -> FmtResult<()> {
		self.fmt.insert(ty.to_string(), fmtstr);

		Ok(())
	}

	pub fn renderer(&self) -> TemplateRenderer {
		TemplateRenderer {
			template: self,
			values: HashMap::new()
		}
	}
}

#[derive(Debug, Clone)]
pub struct TemplateRenderer<'a> {
	template: &'a Template,
	values: HashMap<String, String>
}

impl<'a> TemplateRenderer<'a> {
	pub fn set_value(&mut self, name: String, value: impl TemplateValue) -> FmtResult<()> {
		let fmtstr = self
			.template
			.fmt
			.get(value.type_name())
			.map(String::as_str)
			.unwrap_or(value.default_fmt());

		let val_string = value.format(fmtstr)?;
		self.values.insert(name, val_string);

		Ok(())
	}

	pub fn render(self) -> Result<String, RenderError> {
		let string = self.template.template.render_to_string(&self.values)?;
		Ok(string)
	}
}
