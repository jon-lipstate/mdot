use crate::states::pause::GamePauseState;
use amethyst::{
    audio::output::init_output,
    core::Time,
    ecs::prelude::{Entity, WorldExt},
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{UiCreator, UiFinder, UiText},
    utils::fps_counter::FpsCounter,
    winit::VirtualKeyCode,
};

const TOP_RIGHT: &str = "top_right";
const LOGO: &str = "logo";

#[derive(Default)]
pub struct GameState {
    // If the Game is paused or not
    paused: bool,
    // The UI root entity. Deleting this should remove the complete UI
    ui_root: Option<Entity>,
    top_right: Option<Entity>,
    logo: Option<Entity>,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;
        init_output(&mut world); // needed for registering audio output.

        self.ui_root = //None;
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("game.ron", ())));
    }
    fn on_pause(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.paused = true;
    }

    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.paused = false;
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove Game Screen");
        }
        self.ui_root = None;
        self.logo = None;
        self.top_right = None;
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;
        if self.top_right.is_none() || self.logo.is_none() {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.top_right = ui_finder.find(TOP_RIGHT);
                self.logo = ui_finder.find(LOGO);
            });
        }
        Trans::None
    }
    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Push] Pausing Game!");
                    Trans::Push(Box::new(GamePauseState::default()))
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                if Some(ui_event.target) == self.top_right {
                    log::info!("[UI Event]: {:?} on Top Right", ui_event.event_type);
                } else if Some(ui_event.target) == self.logo {
                    log::info!("[UI Event]: {:?} on Logo", ui_event.event_type);
                }
                Trans::None
            }
            StateEvent::Input(_input) => {
                //log::info!("Input Event detected: {:?}.", input);
                Trans::None
            }
        }
    }
}
