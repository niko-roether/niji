use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use syn::*;

extern crate proc_macro;

#[proc_macro_derive(IntoLua, attributes(lua))]
#[proc_macro_error]
pub fn derive_into_lua(input: TokenStream) -> TokenStream {
	let ast: DeriveInput = parse(input).unwrap();
	let name = ast.ident;

	let lua_attr_ts = ast.attrs.iter().find_map(|a| match &a.meta {
		Meta::List(meta_list) => meta_list
			.path
			.is_ident("lua")
			.then(|| meta_list.tokens.clone()),
		_ => None
	});

	if let Some(lua_attr_ts) = lua_attr_ts {
		let Ok(lua_attr) = syn::parse2::<Ident>(lua_attr_ts.clone()) else {
			abort!(lua_attr_ts, "#[lua()] must contain an identifer");
		};

		if lua_attr != "as_string" {
			abort!(lua_attr, "The only valid use is #[lua(as_string)]");
		}

		quote! {
			impl<'lua> mlua::IntoLua<'lua> for #name {
				fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
					let string = ToString::to_string(&self);
					lua.create_string(string)?.into_lua(lua)
				}
			}
		}
		.into()
	} else {
		let Data::Struct(data_struct) = ast.data else {
			abort_call_site!("Deriving IntoLua is currently only supported for structs");
		};

		let field_names: Vec<&Ident> = data_struct
			.fields
			.iter()
			.map(|f| {
				let Some(field_ident) = &f.ident else {
					abort!(f, "Unnamed struct fields are not supported at the moment");
				};
				field_ident
			})
			.collect();

		quote! {
			impl<'lua> mlua::IntoLua<'lua> for #name {
				fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
					let table = lua.create_table()?;

					#(table.raw_set("#field_names", mlua::IntoLua::into_lua(self.#field_names, lua)?)?;)*

					table.into_lua(lua)
				}
			}
		}
		.into()
	}
}
