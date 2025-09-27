use godot::classes::{Control, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub(crate) struct Hud {
    base: Base<Control>,

    #[init(node = "Debug/PhysicState/Value")]
    physic_state: OnReady<Gd<Label>>,
}

#[godot_api]
impl Hud {
    pub fn change_state(&mut self, state: GString) {
        //TODO: Change to enum PhysicState
        self.physic_state.set_text(&state)
    }
}
