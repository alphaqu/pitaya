use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Theme {
	pub id: String,
	pub name: String,
	pub primary: ThemeColors,
	pub secondary: ThemeColors,
	pub tertiary: ThemeColors,
	pub neutral: ThemeColors,
	pub base: ThemeColors,
}

impl Theme {
	
	pub fn fallback() -> Theme {
		serde_json::from_str(include_str!("../../../../assets/theme/pitaya-dark.json")).unwrap()
	}
}

#[derive(Serialize, Deserialize)]
pub struct ThemeColors {
	pub bg: String,
	pub fg: String,
	pub c_bg: String,
	pub c_fg: String,
}