#![allow(unused_imports)]
use crate::states::{game::GameState, splash::SplashState};
use amethyst::{
    core::Hidden,
    ecs::{Entity, WorldExt},
    input::{is_close_requested, is_key_down, InputBundle, InputEvent, StringBindings},
    prelude::*,
    ui::{RenderUi, UiBundle, UiCreator, UiEvent, UiEventType, UiFinder, UiText},
    winit::VirtualKeyCode,
    State, StateData, StateEvent, Trans,
};

const BUTTON_START: &str = "start";
const BUTTON_LOAD: &str = "load";
const BUTTON_OPTIONS: &str = "options";
const BUTTON_CREDITS: &str = "credits";

#[derive(Default, Debug)]
pub struct MenuState {
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_load: Option<Entity>,
    button_options: Option<Entity>,
    button_credits: Option<Entity>,
}

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        //Instantiate the Prefabs:
        let world = data.world;
        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("menu.ron", ())));
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.button_start.is_none()
            || self.button_load.is_none()
            || self.button_options.is_none()
            || self.button_credits.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_start = ui_finder.find(BUTTON_START);
                self.button_load = ui_finder.find(BUTTON_LOAD);
                self.button_options = ui_finder.find(BUTTON_OPTIONS);
                self.button_credits = ui_finder.find(BUTTON_CREDITS);
            });
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            //Quit & Return to Splash:
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Switch] Switching back to Splash!");
                    Trans::Switch(Box::new(SplashState::default()))
                } else {
                    Trans::None
                }
            }
            //UI Events:
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_start {
                    log::info!("[Trans::Switch] Switching to Game!");
                    return Trans::Switch(Box::new(GameState::default()));
                }
                if Some(target) == self.button_load
                    || Some(target) == self.button_options
                    || Some(target) == self.button_credits
                {
                    log::info!("This Buttons functionality is not yet implemented!");
                }
                // if Some(target) == self.button_credits {
                //     log::info!("[Trans::Switch] Switching to CreditsScreen!");
                //     return Trans::Switch(Box::new(CreditsScreen::default()));
                // }
                Trans::None
            }
            //Logging to Console:
            StateEvent::Input(input) => {
                //https://docs-src.amethyst.rs/stable/amethyst_input/enum.InputEvent.html
                match input {
                    InputEvent::KeyPressed {
                        key_code: kc,
                        scancode: _sc,
                    } => log::info!("KeyPressed: {:?}", kc),
                    //OUTSIDE of Game Window
                    InputEvent::MouseMoved {
                        delta_x: _dx,
                        delta_y: _dy,
                    } => {}
                    //INSIDE of Game Window
                    InputEvent::CursorMoved {
                        delta_x: _dx,
                        delta_y: _dy,
                    } => {
                        //log::info!("Cursor Delta: ({:?},{:?}).", _dx, _dy);
                    }
                    InputEvent::MouseWheelMoved(scroll_direction) => {
                        log::info!("MouseWheelMoved: {:?}", scroll_direction)
                    }
                    InputEvent::MouseButtonPressed(mouse_button) => {
                        log::info!("MouseButtonPressed: {:?}", mouse_button)
                    }
                    _ => (), //log::info!("Input Event detected: {:?}.", input),
                }
                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        // after destroying the current UI, invalidate references as well (makes things cleaner)
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove Menu");
        }

        self.ui_root = None;
        self.button_start = None;
        self.button_load = None;
        self.button_options = None;
        self.button_credits = None;
    }
}
