use godot::classes::{Button, Control, IControl, InputEvent, LineEdit};
use godot::prelude::*;

use crate::setting::Setting;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct SettingOverlay {
    setting: Gd<Setting>,
    base: Base<Control>,
}

#[godot_api]
impl IControl for SettingOverlay {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            setting: Setting::singleton(),
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
        sensitivity_input
            .signals()
            .text_submitted()
            .connect_other(self, |base, value| {
                if let Ok(sensitivity) = value.to_string().parse::<f32>() {
                    base.setting.bind_mut().set_sensitivity(sensitivity);
                };
            });

        self.hide();
    }

    fn unhandled_key_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("escape") {
            self.toggle()
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
            .expect("Expect value to exist");
        sensitivity_input.set_text(&sensitivity_setting.to_string())
    }

    pub fn hide(&mut self) {
        self.base_mut().set_visible(false);
        self.signals().hidden().emit();
    }

    #[signal]
    pub fn shown();
    pub fn show(&mut self) {
        self.display_value();
        self.base_mut().set_visible(true);
        self.signals().shown().emit();
    }

    pub fn toggle(&mut self) {
        if self.base().is_visible() {
            self.hide()
        } else {
            self.show();
        }
    }
}
