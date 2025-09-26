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
            .connect(Self::capture_mouse);
        setting_overlay
            .signals()
            .shown()
            .connect(Self::release_mouse);

        setting_overlay.bind_mut().hide();
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
}
