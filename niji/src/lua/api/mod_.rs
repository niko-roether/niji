use niji_macros::IntoLua;

#[derive(IntoLua)]
pub struct ModApi {
	pub name: String
}
