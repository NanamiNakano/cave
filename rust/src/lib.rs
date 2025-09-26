mod game;
mod player;
mod setting;
mod setting_overlay;

use godot::classes::Engine;
use godot::prelude::*;

use crate::setting::Setting;

struct Cave;

#[gdextension]
unsafe impl ExtensionLibrary for Cave {
    fn on_level_init(level: InitLevel) {
        godot_print!("Rust: Inited on {:?}", level);
        if level == InitLevel::Scene {
            godot_print!("Registering settings singleton");
            Engine::singleton().register_singleton(
                &Setting::class_name().to_string_name(),
                &Setting::new_alloc(),
            );
        }
    }

    fn on_level_deinit(level: InitLevel) {
        godot_print!("Rust: Deinited on {:?}", level);
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            let singleton_name = &Setting::class_name().to_string_name();

            if let Some(my_singleton) = engine.get_singleton(singleton_name) {
                // Unregistering from Godot, and freeing from memory is required
                // to avoid memory leaks, warnings, and hot reloading problems.
                godot_print!("Unregistering settings singleton");
                engine.unregister_singleton(singleton_name);
                my_singleton.free();
            } else {
                godot_error!("Failed to get singleton");
            }
        }
    }
}
