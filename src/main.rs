extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};

struct Example;

impl SimpleState for Example {}

fn main() -> amethyst::Result<()> {
	amethyst::Logger::from_config(Default::default())
		.level_for("gfx_device_gl", amethyst::LogLevelFilter::Warn)
		.level_for("gfx_glyph", amethyst::LogLevelFilter::Error)
		.start();

	let app_path = application_root_dir();
    let display_path = format!(
        "{}/resources/display_config.ron",
		app_path
    );
    let config = DisplayConfig::load(&display_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let game_data =
        GameDataBuilder::default().with_bundle(RenderBundle::new(pipe, Some(config)))?;
    let mut game = Application::new("./", Example, game_data)?;

    game.run();

    Ok(())
}
