use godot::prelude::*;
use godot::classes::{IRigidBody2D, RigidBody2D, AnimatedSprite2D, Area2D};
use rand::seq::{IndexedRandom};
use crate::Player;


#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Mob {
    base: Base<RigidBody2D>
}

#[godot_api]
impl IRigidBody2D for Mob {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }

    fn ready(&mut self) {
        let mut sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
    
        let anime_names = sprite.get_sprite_frames().unwrap().get_animation_names().to_vec();

        let excluded_animations = ["explosion"];
    
        let filtered_animations: Vec<GString> = anime_names
            .into_iter()
            .filter(|name| !excluded_animations.contains(&name.to_string().as_str()))
            .collect();

        let mut rgn = rand::rng();
        let animation = filtered_animations.choose(&mut rgn).unwrap();
    
        sprite.set_animation(animation.arg());
    
        sprite.play();

        self.base_mut().add_to_group("mobs");
    }
}

#[godot_api]
impl Mob {
    #[func]
    fn on_visibility_screen_exit(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    fn explosion(&mut self) {
        let mut sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        sprite.set_animation("explosion");
        sprite.play();

        let target = self.base().callable("destroi");

        let _ = sprite.connect("animation_finished", &target);
    }

    fn find_player(&self) -> Gd<Player> {
        self.base().get_node_as("Player")
    }

    #[func]
    fn destroi(&mut self) {
        self.base_mut().queue_free();
    }
}
