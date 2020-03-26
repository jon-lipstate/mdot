#![allow(unused_imports)]
mod states;
use crate::states::{GameState, MenuState, SplashState};
use amethyst::{
    assets::HotReloadBundle,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{is_close_requested, is_key_down, InputBundle, StringBindings},
    //ecs::prelude::{ReadExpect, Resources, SystemData},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};
use std::time::Duration;

fn main() -> amethyst::Result<()> {
    //Add Filters for logs to prevent spam:
    amethyst::Logger::from_config(Default::default())
        .level_for("gfx_backend_vulkan", amethyst::LogLevelFilter::Warn)
        .start();

    let app_root = application_root_dir()?; //Cargo.toml level

    let config_dir = app_root.join("config");
    let assets_dir = app_root.join("assets");
    let display_config_path = config_dir.join("display.ron"); //display config path

    let game_data = GameDataBuilder::default()
        //Transform Bundle 1st:
        .with_bundle(TransformBundle::new())?
        //Add Input Bundle:
        .with_bundle(InputBundle::<StringBindings>::new())?
        //UI Bundle:
        .with_bundle(UiBundle::<StringBindings>::new())?
        //Hot Reloading Bundle:
        .with_bundle(HotReloadBundle::default())?
        //Rendering Bundle:
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.3, 0.3, 0.35, 1.0]),
                )
                .with_plugin(RenderUi::default()), // required for UI
                                                   //.with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::build(assets_dir, GameState::default())? //SplashState::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_micros(1)),
            60,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
