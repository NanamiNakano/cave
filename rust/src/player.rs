use godot::classes::{Camera3D, CharacterBody3D, ICharacterBody3D, Input};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
struct Player {
    #[export]
    #[init(val = 5.0)]
    speed: f64,
    #[export]
    #[init(val = 4.5)]
    jump_velocity: f64,
    #[init(val = Vector3::ZERO)]
    target_velocity: Vector3,

    #[init(node = "Head")]
    head: OnReady<Gd<Node3D>>,
    #[init(node = "Head/Camera3D")]
    cam: OnReady<Gd<Camera3D>>,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();

        let h_direction = input.get_vector("move_right", "move_left", "move_back", "move_forward");
        let direction = Vector3::new(h_direction.x, 0.0, h_direction.y);

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
}
