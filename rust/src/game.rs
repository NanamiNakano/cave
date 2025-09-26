use godot::classes::Input;
use godot::classes::input::MouseMode;
use godot::prelude::*;

use crate::setting_overlay::SettingOverlay;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Game {
    base: Base<Node>,
}

#[godot_api]
impl INode for Game {
    fn ready(&mut self) {
        let mut setting_overlay = self
            .base()
            .get_node_as::<SettingOverlay>("CanvasLayer/SettingOverlay");

        setting_overlay
            .signals()
            .hidden()
            .connect_other(self, |base| {
                Self::capture_mouse();
                base.resume();
            });
        setting_overlay
            .signals()
            .shown()
            .connect_other(self, |base| {
                Self::release_mouse();
                base.pause();
            });

        Self::capture_mouse();
    }
}

#[godot_api]
impl Game {
    fn capture_mouse() {
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED)
    }

    fn release_mouse() {
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::VISIBLE)
    }

    fn pause(&self) {
        let mut tree = self.base().get_tree().expect("Expect tree to exist");
        tree.set_pause(true);
    }

    fn resume(&self) {
        let mut tree = self.base().get_tree().expect("Expect tree to exist");
        tree.set_pause(false);
    }
}
