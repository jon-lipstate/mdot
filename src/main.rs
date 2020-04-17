#![allow(unused_imports, dead_code)]
use amethyst::{
    assets::{Prefab, PrefabData},
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    derive::PrefabData,
    ecs::{storage::DenseVecStorage, Component},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
};
mod components;
mod constants;
mod defintions;
mod events;
mod key_bindings;
mod resources;
mod states;
mod systems;
use key_bindings::UserInputBindingTypes;
use serde::{Deserialize, Serialize};
use states::PlayState;
use std::time::Duration;

// #[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
// #[prefab(Component)]
// #[serde(deny_unknown_fields)]
// pub struct Position(pub f32, pub f32, pub f32);

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        .level_for("gfx_backend_vulkan", amethyst::LogLevelFilter::Warn)
        .start();

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config = config_dir.join("display.ron");
    let key_bindings_config_path = config_dir.join("bindings.ron");
    let input_bundle = InputBundle::<UserInputBindingTypes>::new()
        .with_bindings_from_file(key_bindings_config_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        // .with_bundle(
        //     InputBundle::<StringBindings>::new()
        //         .with_bindings_from_file(&key_bindings_config_path)?,
        // )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(FpsCounterBundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        //.with(systems::MapLoader, "map_system", &[])
        .with(systems::MotionSystem, "motion_system", &[])
        //.with(systems::InputSystem::default(), "input_system2", &[]) //cant use 'input_system' it is reserved by bundle i think
        .with_bundle(systems::InputSystemBundle)?;

    //let mut game = Application::new(assets_dir, PlayState::default(), game_data)?;

    let mut game = Application::build(assets_dir, PlayState::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_micros(1)),
            120,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
