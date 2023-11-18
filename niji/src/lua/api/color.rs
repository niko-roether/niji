use std::str::FromStr;

use mlua::{FromLua, IntoLua, Lua, UserData, UserDataFields, UserDataMethods};

use crate::types::color::Color;

use super::Module;

impl UserData for Color {
	fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
		fields.add_field_method_get("r", |_, this| Ok(this.r));
		fields.add_field_method_get("g", |_, this| Ok(this.g));
		fields.add_field_method_get("b", |_, this| Ok(this.b));
		fields.add_field_method_get("a", |_, this| Ok(this.a));
	}

	fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
		methods.add_method("lighten", |_, this, amount: f32| Ok(this.lighten(amount)));
		methods.add_method("darken", |_, this, amount: f32| Ok(this.darken(amount)));
		methods.add_meta_method("__tostring", |_, this, ()| Ok(this.to_string()));
	}
}

impl<'lua> FromLua<'lua> for Color {
	fn from_lua(value: mlua::Value<'lua>, _: &'lua mlua::Lua) -> mlua::Result<Self> {
		match value {
			mlua::Value::String(str) => {
				Color::from_str(str.to_str()?).map_err(mlua::Error::runtime)
			}
			mlua::Value::UserData(data) => {
				let color_ref = data.borrow::<Color>()?;
				Ok(*color_ref)
			}
			_ => Err(mlua::Error::runtime("Cannot cast this value to a color!"))
		}
	}
}

pub struct ColorApi;

impl ColorApi {
	#[inline]
	#[allow(clippy::new_ret_no_self)]
	pub fn new(_: &Lua, col: Color) -> mlua::Result<Color> {
		Ok(col)
	}

	pub fn blend(_: &Lua, (col1, col2, t): (Color, Color, f32)) -> mlua::Result<Color> {
		Ok(Color::blend(col1, col2, t))
	}

	pub fn mix(_: &Lua, (col1, col2): (Color, Color)) -> mlua::Result<Color> {
		Ok(Color::mix(col1, col2))
	}
}

impl Module for ColorApi {
	const NAMESPACE: &'static str = "col";

	fn build(lua: &Lua) -> mlua::Result<mlua::Value> {
		let module = lua.create_table()?;

		module.raw_set("new", lua.create_function(ColorApi::new)?)?;
		module.raw_set("blend", lua.create_function(ColorApi::blend)?)?;
		module.raw_set("mix", lua.create_function(ColorApi::mix)?)?;

		module.into_lua(lua)
	}
}
