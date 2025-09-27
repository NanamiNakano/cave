use std::f32::consts::PI;

use derive_more::Display;
use godot::classes::{
    AnimationPlayer, CharacterBody3D, ICharacterBody3D, Input, InputEvent, InputEventMouseMotion,
};
use godot::prelude::*;
use crate::player::PhysicState::{WalkingBack, WalkingForward};
use crate::setting::Setting;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub(crate) struct Player {
    #[export]
    #[init(val = 5.0)]
    speed: f64,
    #[export]
    #[init(val = 4.5)]
    jump_velocity: f64,
    #[init(val = Vector3::ZERO)]
    target_velocity: Vector3,
    #[init(val = PhysicState::Idle)]
    physic_state: PhysicState,

    #[init(node = "Head")]
    head: OnReady<Gd<Node3D>>,
    #[init(node = "Pivot/Model/AnimationPlayer")]
    ani_player: OnReady<Gd<AnimationPlayer>>,

    base: Base<CharacterBody3D>,
}

#[derive(GodotConvert, PartialOrd, PartialEq, Debug, Clone, Display, Copy)]
#[godot(via=GString)]
pub enum PhysicState {
    Idle,
    WalkingForward,
    WalkingBack,
    WalkingLeft,
    WalkingRight,
    Jumping,
    Falling,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let basis = self.base().get_global_transform().basis;

        let h_direction = input.get_vector("move_right", "move_left", "move_back", "move_forward");
        let direction = h_direction.x * basis.col_a() + h_direction.y * basis.col_c();
        let mut future_state;

        self.target_velocity.x = direction.x * self.speed as f32;
        self.target_velocity.z = direction.z * self.speed as f32;
        if self.target_velocity.x != 0.0 || self.target_velocity.z != 0.0 {
            if h_direction.x != 0.0 {
                future_state = match h_direction.x {
                    x if x < 0.0 => PhysicState::WalkingRight,
                    x if x > 0.0 => PhysicState::WalkingLeft,
                    _ => unreachable!()
                }
            } else {
                future_state = match h_direction.y {
                    y if y < 0.0 => WalkingBack,
                    y if y > 0.0 => WalkingForward,
                    _ => unreachable!()
                }
            }
        } else {
            future_state = PhysicState::Idle;
        }

        if self.base().is_on_floor() {
            self.target_velocity.y = 0.0;
            if input.is_action_pressed("jump") {
                self.target_velocity.y = self.jump_velocity as f32;
                future_state = PhysicState::Jumping;
            }
        } else {
            self.target_velocity += self.base().get_gravity() * delta as f32;
            if self.target_velocity.y < 0.0 {
                future_state = PhysicState::Falling;
            } else {
                future_state = PhysicState::Jumping;
            }
        }

        if future_state != self.physic_state {
            self.physic_state = future_state;
            self.signals()
                .physic_state_changed()
                .emit(future_state.clone())
        }
        let target_velocity = self.target_velocity;
        self.base_mut().set_velocity(target_velocity);
        self.base_mut().move_and_slide();
    }

    fn ready(&mut self) {
        self.signals().physic_state_changed().emit(PhysicState::Idle)
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let Ok(mouse_motion) = event.try_cast::<InputEventMouseMotion>() else {
            return;
        };

        let sensitivity = Setting::singleton().bind().get_sensitivity();
        self.base_mut()
            .rotate_y(-mouse_motion.get_relative().x * sensitivity);
        self.head
            .rotate_x(mouse_motion.get_relative().y * sensitivity);

        let mut raw_head_rotation = self.head.get_rotation();
        raw_head_rotation.x = raw_head_rotation.x.clamp(-PI / 3.0, PI / 3.0);
        self.head.set_rotation(raw_head_rotation);
    }
}

#[godot_api]
impl Player {
    #[signal]
    pub fn physic_state_changed(state: PhysicState);

    pub fn animation_process(&mut self) {

    }
}
