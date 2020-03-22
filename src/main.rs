#![allow(unused_imports)]
mod states;
use crate::states::MenuState;
use amethyst::{
    core::transform::TransformBundle,
    input::{is_close_requested, is_key_down, InputBundle, StringBindings},
    //ecs::prelude::{ReadExpect, Resources, SystemData},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    //Add Filters for logs to prevent spam:
    amethyst::Logger::from_config(Default::default())
        .level_for("gfx_backend_vulkan", amethyst::LogLevelFilter::Warn)
        .start();

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        //Add Input Bundle:
        .with_bundle(InputBundle::<StringBindings>::new())?
        //Rendering Bundle:
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new("/", MenuState, game_data)?;
    game.run();

    Ok(())
}
