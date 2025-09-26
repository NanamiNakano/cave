use crate::setting::Setting;
use godot::classes::input::MouseMode;
use godot::classes::Button;
use godot::classes::Control;
use godot::classes::IControl;
use godot::classes::Input;
use godot::classes::InputEvent;
use godot::classes::LineEdit;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
struct SettingOverlay {
    setting: Gd<Setting>,
    base: Base<Control>,
}

#[godot_api]
impl IControl for SettingOverlay {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            setting: Setting::singleton().expect("Except setting singleton exists"),
            base,
        }
    }

    fn ready(&mut self) {
        let back_button = self.base().get_node_as::<Button>("Back");
        back_button
            .signals()
            .pressed()
            .connect_other(self, Self::hide);

        let sensitivity_input = self.base().get_node_as::<LineEdit>("Sensitivity/Input");
        sensitivity_input.signals().text_submitted().connect_other(self, |base, value| {
            if let Ok(sensitivity) = value.to_string().parse::<f32>() {
                base.set_sensitivity(sensitivity)
            };
        });

        self.display_value();
    }

    fn unhandled_key_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("escape") {
            self.display_value();
            let mut input = Input::singleton();
            input.set_mouse_mode(MouseMode::VISIBLE);
            self.base_mut().set_visible(true);
        }
    }
}

#[godot_api]
impl SettingOverlay {
    fn display_value(&mut self) {
        let mut sensitivity_input = self.base().get_node_as::<LineEdit>("Sensitivity/Input");
        let sensitivity_setting = self
            .setting
            .bind()
            .get_value::<f32>("global", "sensitivity")
            .expect("Except valur exists");
        sensitivity_input.set_text(&sensitivity_setting.to_string())
    }

    fn hide(&mut self) {
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);
        self.base_mut().set_visible(false);
    }

    fn set_sensitivity(&mut self, sensitivity: f32) {
        self.setting.bind_mut().set_and_save("global", "sensitivity", sensitivity).expect("Except ok");
    }
}
