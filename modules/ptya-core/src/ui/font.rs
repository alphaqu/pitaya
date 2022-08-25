use std::sync::Arc;
use log::info;
use egui::text::{FontData, FontDefinitions};
use egui::FontFamily;
use anyways::ext::AuditExt;
use crate::{AssetManager, Location};

pub async fn load_fonts(asset: &AssetManager) -> anyways::Result<FontDefinitions> {
	let fonts = vec![
		"Roboto-ThinItalic",
		"Roboto-Thin",
		"Roboto-Italic",
		"Roboto-Regular",
		"Roboto-MediumItalic",
		"Roboto-Medium",
		"Roboto-Light",
		"Roboto-LightItalic",
		"Roboto-BoldItalic",
		"Roboto-Bold",
		"Roboto-BlackItalic",
		"Roboto-Black",
		"Icons",
	];

	let value: Vec<_> = fonts.into_iter().map(|value| {
		let asset_2 = asset.clone();
		tokio::spawn(async {
			let asset = asset_2;
			load_font(&asset, value).await
		})
	}).collect();

	let mut fonts = FontDefinitions::empty();
	for handle in value {
		let (font, name) = handle.await??;
		add_font(&mut fonts, font, &name);
	}

	fonts
		.families
		.insert(FontFamily::Proportional, vec!["Roboto-Regular".to_string()]);
	fonts
		.families
		.insert(FontFamily::Monospace, vec!["Roboto-Regular".to_string()]);

	Ok(fonts)
}

fn add_font(fonts: &mut FontDefinitions, font: FontData, name: &str) {
	fonts.font_data.insert(name.to_owned(), font);
	fonts.families.insert(
		FontFamily::Name(Arc::from(name)),
		vec![name.to_string(), "Roboto-Regular".to_string()],
	);
	info!("Adding font: {name}")
}

async fn load_font(asset: &AssetManager, name: &str) -> anyways::Result<(FontData, String)> {
	info!("Loading font: {name}");
	let data = asset
		.read_file(Location::Assets, format!("fonts/{name}.ttf"))
		.await.wrap_err_with(| | format!( "Failed to load data for font {name}"))?;
	let font = FontData::from_owned(data);
	Ok((font, name.to_string()))
}
