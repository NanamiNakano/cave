mod player;

use godot::prelude::*;

struct Cave;

#[gdextension]
unsafe impl ExtensionLibrary for Cave {
    fn on_level_init(level: InitLevel) {
        godot_print!("Rust: Inited on {:?}", level)
    }
}
