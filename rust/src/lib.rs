use godot::prelude::*;

struct GameExtension;

mod entity;
mod main_game;
mod hud;

pub use entity::*;
pub use hud::*;

#[gdextension]
unsafe impl ExtensionLibrary for GameExtension {}
