use godot::classes::{CharacterBody3D, ICharacterBody3D, Input};
use godot::prelude::*;

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
    target_velocity: Vector3,

    base: Base<CharacterBody3D>
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn physics_process(&mut self, delta: f64) {
        let mut direction = Vector3::ZERO;

        let input = Input::singleton();
        if input.is_action_pressed("move_right") {
            direction.x += 1.0;
        }
        if input.is_action_pressed("move_left") {
            direction.x -= 1.0;
        }
        if input.is_action_pressed("move_forward") {
            direction.z += 1.0;
        }
        if input.is_action_pressed("move_back") {
            direction.z -= 1.0;
        }

        if direction != Vector3::ZERO {
            let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
            direction = direction.normalized();
            pivot.set_basis(Basis::looking_at(direction, Vector3::new(0.0, 1.0, 0.0), true))
        }

        if input.is_action_pressed("jump") && self.base().is_on_floor() {
            self.target_velocity.y = self.jump_velocity as f32
        }

        self.target_velocity.x = direction.x * self.speed as f32;
        self.target_velocity.z = direction.z * self.speed as f32;

        if !self.base().is_on_floor() {
            self.target_velocity += self.base().get_gravity() * delta as f32;
        }

        let target_velocity = self.target_velocity.clone();
        self.base_mut().set_velocity(target_velocity);
        self.base_mut().move_and_slide();
    }
}
