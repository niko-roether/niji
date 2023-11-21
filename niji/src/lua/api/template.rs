use std::collections::HashMap;

use mlua::{IntoLua, Lua, UserData, UserDataMethods};

use crate::{
	template::{Template, ToTemplateData},
	types::color::Color
};

use super::Module;

fn is_array(table: &mlua::Table) -> bool {
	for i in 1..=table.clone().pairs::<mlua::Value, mlua::Value>().count() {
		let Ok(value) = table.raw_get::<_, mlua::Value>(i) else {
			return false;
		};
		if value == mlua::Value::Nil {
			return false;
		}
	}
	true
}

fn to_template_vec(table: mlua::Table) -> mlua::Result<Box<dyn ToTemplateData>> {
	let len = table.raw_len();
	let mut vec = Vec::with_capacity(len);

	for i in 1..=len {
		vec.push(get_template_value(table.raw_get(i)?)?.unwrap());
	}

	Ok(Box::new(vec))
}

fn to_template_map(table: mlua::Table) -> mlua::Result<Box<dyn ToTemplateData>> {
	let mut map = HashMap::new();

	for pair in table.pairs::<String, mlua::Value>() {
		let (key, value) = pair?;
		if let Some(template_value) = get_template_value(value)? {
			map.insert(key, template_value);
		}
	}

	Ok(Box::new(map))
}

fn get_template_value(value: mlua::Value) -> mlua::Result<Option<Box<dyn ToTemplateData>>> {
	let template_val: Box<dyn ToTemplateData> = match value {
		mlua::Value::Number(num) => Box::new(num),
		mlua::Value::Integer(int) => Box::new(int),
		mlua::Value::Boolean(bool) => Box::new(bool),
		mlua::Value::String(string) => Box::new(string.to_string_lossy().into_owned()),
		mlua::Value::UserData(user_data) => {
			if let Ok(color) = user_data.borrow::<Color>() {
				Box::new(*color)
			} else {
				return Err(mlua::Error::runtime(format!(
					"This userdata type isn't supported in templates: {user_data:?}"
				)));
			}
		}
		mlua::Value::Table(table) => {
			if is_array(&table) {
				to_template_vec(table)?
			} else {
				to_template_map(table)?
			}
		}
		mlua::Value::Nil => return Ok(None),
		value => {
			return Err(mlua::Error::runtime(format!(
				"This type isn't supported in templates: {value:?}"
			)))
		}
	};

	Ok(Some(template_val))
}

impl UserData for Template {
	fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
		methods.add_method_mut(
			"set_format",
			|_, this, (ty, fmtstr): (String, String)| -> mlua::Result<()> {
				this.set_format(&ty, fmtstr).map_err(mlua::Error::runtime)
			}
		);

		methods.add_method_mut(
			"render",
			|_, this, values: mlua::Table| -> mlua::Result<String> {
				let mut renderer = this.renderer();
				for pair in values.pairs::<String, mlua::Value>() {
					let (key, value) = pair?;
					if let Some(template_value) = get_template_value(value)? {
						renderer
							.set_value(key, template_value)
							.map_err(mlua::Error::runtime)?;
					}
				}

				Ok(renderer.render())
			}
		)
	}
}

pub struct TemplateApi;

impl TemplateApi {
	fn load(lua: &Lua, path: String) -> mlua::Result<mlua::Value> {
		let template = Template::load(path).map_err(mlua::Error::runtime)?;

		template.into_lua(lua)
	}

	fn parse(lua: &Lua, template: String) -> mlua::Result<mlua::Value> {
		let template = Template::parse(template).map_err(mlua::Error::runtime)?;

		template.into_lua(lua)
	}
}

impl Module for TemplateApi {
	const NAMESPACE: &'static str = "template";

	fn build(lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
		let module = lua.create_table()?;

		module.raw_set("load", lua.create_function(Self::load)?)?;
		module.raw_set("parse", lua.create_function(Self::parse)?)?;

		module.into_lua(lua)
	}
}
