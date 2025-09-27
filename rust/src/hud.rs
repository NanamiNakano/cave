use godot::classes::Control;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct Hud {
    base: Base<Control>
}
