use std::{any::type_name, collections::HashMap, marker::PhantomData};

use mlua::{IntoLua, Lua, UserData, UserDataMethods};
use thiserror::Error;

use crate::{
	template::{Template, ToTemplateData},
	types::color::Color
};

use super::Module;

fn get_template_value(value: mlua::Value) -> mlua::Result<Box<dyn ToTemplateData>> {
	let template_val: Box<dyn ToTemplateData> = match value {
		mlua::Value::Number(num) => Box::new(num),
		mlua::Value::Integer(int) => Box::new(int),
		mlua::Value::Boolean(bool) => Box::new(bool),
		mlua::Value::String(string) => Box::new(string.to_string_lossy().into_owned()),
		mlua::Value::UserData(user_data) => {
			if let Ok(color) = user_data.borrow::<Color>() {
				Box::new(*color)
			} else {
				return Err(mlua::Error::runtime(
					"This userdata type isn't supported in templates!"
				));
			}
		}
		mlua::Value::Table(table) => {
			let mut is_vec = true;
			let mut vec = Vec::<Box<dyn ToTemplateData>>::new();
			let mut map = HashMap::<String, Box<dyn ToTemplateData>>::new();

			for pair in table.pairs::<mlua::Value, mlua::Value>() {
				let (key, value) = pair?;
				match (is_vec, key) {}
			}

			todo!()
		}
		_ => {
			return Err(mlua::Error::runtime(
				"This type isn't supported in templates!"
			))
		}
	};

	Ok(template_val)
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
					let template_value = get_template_value(value)?;
					renderer
						.set_value(key, template_value)
						.map_err(mlua::Error::runtime)?;
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
