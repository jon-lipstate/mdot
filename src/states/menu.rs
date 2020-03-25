#![allow(unused_imports)]
use crate::states::{game::GameState, splash::SplashState};
use amethyst::{
    core::Hidden,
    ecs::{Entity, WorldExt},
    input::{is_close_requested, is_key_down, InputBundle, InputEvent, StringBindings},
    prelude::*,
    ui::{RenderUi, UiBundle, UiCreator, UiEvent, UiFinder, UiText},
    winit::VirtualKeyCode,
    State, StateData, StateEvent, Trans,
};

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
        let world = data.world;

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("menu.ron", ())));
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                log::info!(
                    "[HANDLE_EVENT] You just interacted with a ui element: {:?}",
                    ui_event
                );
                Trans::None
            }
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
        }
    }
}
