use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use syn::*;

extern crate proc_macro;

enum LuaAttr {
	With(Path)
}

impl LuaAttr {
	fn parse(attr: &Attribute) -> Option<Self> {
		match &attr.meta {
			Meta::List(meta_list) => {
				if meta_list.path.is_ident("lua_with") {
					let Ok(path) = parse2::<LitStr>(meta_list.tokens.clone())
						.and_then(|lit| lit.parse_with(Path::parse_mod_style))
					else {
						abort!(
							meta_list,
							"Must contain a function in quotes, like \"PathBuf::to_string_lossy\""
						);
					};
					Some(Self::With(path))
				} else {
					None
				}
			}
			_ => None
		}
	}
}

fn get_lua_attr(attrs: &[Attribute]) -> Option<LuaAttr> {
	attrs.iter().find_map(LuaAttr::parse)
}

fn derive_into_lua_with(ast: DeriveInput, path: Path) -> TokenStream {
	let name = ast.ident;

	quote! {
		impl<'lua> mlua::IntoLua<'lua> for #name {
			fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
				mlua::IntoLua::into_lua(#path(&self), lua)
			}
		}
	}
	.into()
}

fn derive_into_lua_table(ast: DeriveInput) -> TokenStream {
	let name = ast.ident;

	let Data::Struct(data_struct) = ast.data else {
		abort_call_site!("Deriving IntoLua directly is currently only supported for structs");
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

	let field_into_lua: Vec<proc_macro2::TokenStream> = data_struct
		.fields
		.iter()
		.map(|f| {
			let lua_attr = get_lua_attr(&f.attrs);
			let name = f.ident.clone().unwrap();

			match lua_attr {
				Some(LuaAttr::With(path)) => quote! {
					mlua::IntoLua::into_lua(#path(&self.#name), lua)
				},
				_ => quote! {
					mlua::IntoLua::into_lua(self.#name, lua)
				}
			}
		})
		.collect();

	quote! {
		impl<'lua> mlua::IntoLua<'lua> for #name {
			fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
				let table = lua.create_table()?;

				#(table.raw_set(stringify!(#field_names), #field_into_lua?)?;)*

				table.into_lua(lua)
			}
		}
	}
	.into()
}

#[proc_macro_derive(IntoLua, attributes(lua_with))]
#[proc_macro_error]
pub fn derive_into_lua(input: TokenStream) -> TokenStream {
	let ast: DeriveInput = parse(input).unwrap();

	let lua_attr = get_lua_attr(&ast.attrs);

	match lua_attr {
		Some(LuaAttr::With(path)) => derive_into_lua_with(ast, path),
		_ => derive_into_lua_table(ast)
	}
}
