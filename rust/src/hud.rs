#[cfg(not(debug_assertions))]
use godot::classes::IControl;
#[cfg(debug_assertions)]
use godot::classes::Label;
use godot::classes::{Control, Panel};
use godot::prelude::*;
use crate::player::PhysicState;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub(crate) struct Hud {
    base: Base<Control>,

    #[init(node = "Debug")]
    debug: OnReady<Gd<Panel>>,
}

#[cfg(debug_assertions)]
#[godot_api]
impl Hud {
    pub fn change_state(&mut self, state: PhysicState) {
        //TODO: Change to enum PhysicState
        let mut physic_state = self.debug.get_node_as::<Label>("PhysicState/Value");
        physic_state.set_text(&state.to_string())
    }
}

#[cfg(not(debug_assertions))]
#[godot_api]
impl IControl for Hud {
    fn ready(&mut self) {
        self.debug.queue_free()
    }
}
