use godot::classes::Input;
use godot::classes::input::MouseMode;
use godot::prelude::*;

use crate::hud::Hud;
use crate::player::Player;
use crate::setting_overlay::SettingOverlay;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Game {
    base: Base<Node>,

    #[init(node = "CanvasLayer/SettingOverlay")]
    setting_overlay: OnReady<Gd<SettingOverlay>>,
    #[init(node = "CanvasLayer/HUD")]
    hud: OnReady<Gd<Hud>>,
    #[init(node = "Player")]
    player: OnReady<Gd<Player>>,
}

#[godot_api]
impl INode for Game {
    fn ready(&mut self) {
        self.setting_overlay
            .signals()
            .hidden()
            .connect_other(self, |game| {
                Self::capture_mouse();
                game.resume();
                game.hud.set_visible(true)
            });
        self.setting_overlay
            .signals()
            .shown()
            .connect_other(self, |game| {
                Self::release_mouse();
                game.pause();
                game.hud.set_visible(false);
            });

        self.player
            .signals()
            .physic_state_changed()
            .connect_other(&(*self.hud), Hud::change_state);

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
