use crate::setting::Setting;
use godot::classes::{
    CharacterBody3D, ICharacterBody3D, Input, InputEvent, InputEventMouseMotion,
};
use godot::prelude::*;
use std::f32::consts::PI;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
struct Player {
    #[export]
    #[init(val = 5.0)]
    pub speed: f64,
    #[export]
    #[init(val = 4.5)]
    pub jump_velocity: f64,
    #[init(val = Vector3::ZERO)]
    pub target_velocity: Vector3,

    #[init(node = "Head")]
    head: OnReady<Gd<Node3D>>,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let basis = self.base().get_global_transform().basis;

        let h_direction = input.get_vector("move_right", "move_left", "move_back", "move_forward");
        let direction = h_direction.x * basis.col_a() + h_direction.y * basis.col_c();

        if self.base().is_on_floor() {
            if input.is_action_pressed("jump") {
                self.target_velocity.y = self.jump_velocity as f32
            }
        } else {
            self.target_velocity += self.base().get_gravity() * delta as f32;
        }
        self.target_velocity.x = direction.x * self.speed as f32;
        self.target_velocity.z = direction.z * self.speed as f32;

        let target_velocity = self.target_velocity;
        self.base_mut().set_velocity(target_velocity);
        self.base_mut().move_and_slide();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let Ok(mouse_motion) = event.try_cast::<InputEventMouseMotion>() else {
            return;
        };

        let sensitivity = Setting::singleton().bind().get_sensitivity();
        self.base_mut().rotate_y(-mouse_motion.get_relative().x * sensitivity);
        self.head.rotate_x(mouse_motion.get_relative().y * sensitivity);

        let mut raw_head_rotation = self.head.get_rotation();
        godot_print!("{:?}", raw_head_rotation);
        raw_head_rotation.x = raw_head_rotation.x.clamp(-PI/3.0, PI/3.0);
        self.head.set_rotation(raw_head_rotation);
    }
}
