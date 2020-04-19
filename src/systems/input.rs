use crate::{
    components::{InputComponent, Moving, PlayerComponent, TilePosition},
    constants,
    defintions::{Direction, UserActions},
    key_bindings::UserInputBindingTypes,
};

use amethyst::{
    core::{bundle::SystemBundle, timing::Time, transform::Transform, SystemDesc},
    derive::SystemDesc,
    ecs::prelude::*,
    ecs::{DispatcherBuilder, Read, ReadStorage, System, SystemData, World, Write, WriteStorage},
    input::{InputEvent, InputHandler, VirtualKeyCode},
    shrev::{EventChannel, ReaderId},
};
use std::time::Duration;

pub struct InputSystemBundle;
impl<'a, 'b> SystemBundle<'a, 'b> for InputSystemBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), amethyst::Error> {
        builder.add(InputSystemDesc::default().build(world), "inp_system", &[]);
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct InputSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, InputSystem> for InputSystemDesc {
    fn build(self, world: &mut World) -> InputSystem {
        <InputSystem as System<'_>>::SystemData::setup(world);
        let reader = world
            .fetch_mut::<EventChannel<InputEvent<UserInputBindingTypes>>>()
            .register_reader();
        InputSystem::new(reader)
    }
}

#[derive(Debug, SystemDesc)]
pub struct InputSystem {
    reader: ReaderId<InputEvent<UserInputBindingTypes>>,
    latched_actions: Vec<(UserActions, f64)>, //expiry time in f64
    latched_keys: Vec<(VirtualKeyCode, f64)>, //expiry time in f64
    typing_mode: bool,
    typed_input: Vec<char>,
}

impl InputSystem {
    pub fn new(reader: ReaderId<InputEvent<UserInputBindingTypes>>) -> Self {
        Self {
            latched_actions: Vec::new(),
            latched_keys: Vec::new(),
            typing_mode: false,
            typed_input: Vec::new(),
            reader,
        }
    }
    ///Iterates on latched_actions & removes any expired action
    fn remove_expired_latches(&mut self, current_time: f64) {
        self.latched_actions.retain(|(_action, expiry_time)| {
            return current_time < *expiry_time;
        });
        self.latched_keys.retain(|(_action, expiry_time)| {
            return current_time < *expiry_time;
        });
    }
}

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        Read<'s, InputHandler<UserInputBindingTypes>>,
        Read<'s, Time>,
        Read<'s, EventChannel<InputEvent<UserInputBindingTypes>>>,
        ReadStorage<'s, PlayerComponent>,
        WriteStorage<'s, InputComponent>,
    );

    fn run(
        &mut self,
        (input, time, input_event_channel, pcs, mut input_components): Self::SystemData,
    ) {
        let t = time.absolute_real_time().as_secs_f64();
        let action_expiry = t + constants::ACTION_DELAY_MS as f64 / 1000.;
        let typing_expiry = t + constants::TYPING_DELAY_MS as f64 / 1000.;
        self.remove_expired_latches(t);
        let mut cmd_action = None::<UserActions>;
        let user_actions = vec![
            UserActions::TypingMode,
            UserActions::Move(Direction::North),
            UserActions::Move(Direction::East),
            UserActions::Move(Direction::South),
            UserActions::Move(Direction::West),
            UserActions::Melee,
        ];

        let input_events = input_event_channel.read(&mut self.reader);
        if self.typing_mode {
            let return_key = 0x0d as char;
            let escape_key = 0x1b as char;
            let backspace_key = 0x08 as char;
            let del_key = 0x7f as char;
            for e in input_events {
                match e {
                    InputEvent::KeyTyped(key) => {
                        if key == &return_key {
                            let data = self.typed_input.iter().collect::<String>();
                            if data.len() > 0 {
                                log::info!("Action: TypedData({:?})", data);
                                //command_queue.add(Command::TypedData(data));
                            }
                            self.typed_input = Vec::new();
                            self.typing_mode = false;
                            log::info!("TypingMode {}", self.typing_mode);
                            try_latch(
                                UserActions::TypingMode,
                                &mut self.latched_actions,
                                action_expiry,
                            );
                        } else if key == &escape_key {
                            self.typing_mode = false;
                            log::info!("TypingMode {}", self.typing_mode);
                        } else if key == &backspace_key || key == &del_key {
                            self.typed_input.pop();
                            log::info!("{:?}", self.typed_input.iter().collect::<String>());
                        } else {
                            self.typed_input.push(*key);
                            log::info!("{:?}", self.typed_input.iter().collect::<String>());
                        }
                    }
                    _ => (),
                }
            }
        } else {
            for action in user_actions {
                if input.action_is_down(&action).unwrap() {
                    if try_latch(action.clone(), &mut self.latched_actions, action_expiry) {
                        if action != UserActions::TypingMode {
                            log::info!("Action: {:?}", action);
                            //command_queue.add(command.clone());
                            cmd_action = Some(action);
                        } else {
                            self.typing_mode = !self.typing_mode;
                            try_latch(
                                VirtualKeyCode::Return,
                                &mut self.latched_keys,
                                typing_expiry,
                            ); // ensure latched to prevent runaway entry
                            log::info!("TypingMode {}", self.typing_mode);
                        }
                    }
                }
            }
        }
        //Apply to player's input:
        for (inp, _) in (&mut input_components, &pcs).join() {
            inp.action = cmd_action.clone();
            // match &cmd_action {
            //     UserActions::Move(dir) => {
            //         let m = Moving {
            //             start_tile_position: p.position.clone(),
            //             direction: dir.clone(),
            //             value: 1,
            //             time_remaining: constants::ACTION_DELAY_MS as f32 / 1000.,
            //             duration: constants::ACTION_DELAY_MS as f32 / 1000.,
            //         };
            //         input_components.insert(e, m).unwrap();
            //     }
            //     _ => (),
            // }
        }
    }
}
/// 1. Checks if the key is down
/// 2. Checks that an action of the same type does not exist in latched_actions.
/// 3. If new action is unique, pushes into latched_actions
fn try_latch<I: PartialEq>(val: I, vec: &mut Vec<(I, f64)>, expiry_time: f64) -> bool {
    let mut can_latch = !vec.iter().any(|a| a.0 == val);
    //When vec is empty, the closure above will return false
    if vec.len() == 0 {
        can_latch = true;
    }
    if can_latch {
        vec.push((val, expiry_time));
        //self.latched_keys.push((key, expiry_time));
    }
    return can_latch;
}
