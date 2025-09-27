use std::f32::consts::PI;

use derive_more::Display;
use godot::classes::{
    AnimationPlayer, CharacterBody3D, ICharacterBody3D, Input, InputEvent, InputEventMouseMotion,
};
use godot::prelude::*;

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
pub(crate) enum PhysicState {
    Idle,
    Walking,
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
        let mut future_state = self.physic_state;

        if self.base().is_on_floor() {
            self.target_velocity.y = 0.0;
            if self.physic_state != PhysicState::Walking {
                future_state = PhysicState::Idle;
            }

            if input.is_action_pressed("jump") {
                self.target_velocity.y = self.jump_velocity as f32;
                future_state = PhysicState::Jumping;
            }
        } else {
            self.target_velocity += self.base().get_gravity() * delta as f32;
            if self.target_velocity.y < 0.0 {
                future_state = PhysicState::Falling;
            }
        }
        self.target_velocity.x = direction.x * self.speed as f32;
        self.target_velocity.z = direction.z * self.speed as f32;
        let target_velocity = self.target_velocity;

        if future_state != self.physic_state {
            self.physic_state = future_state;
            self.signals()
                .physic_state_changed()
                .emit(&future_state.to_string())
        }
        self.base_mut().set_velocity(target_velocity);
        self.base_mut().move_and_slide();
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
    pub fn physic_state_changed(state: GString); //TODO: Change to enum PhysicState
}
