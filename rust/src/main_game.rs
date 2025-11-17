/// main_game.rs
use godot::prelude::*;
use godot::classes::{Node, Marker2D, Timer, PathFollow2D, AudioStreamPlayer, CanvasItem};
use crate::{Mob, Player, HUD};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    mob_scene: OnReady<Gd<PackedScene>>,
    player: OnReady<Gd<Player>>,
    hud: OnReady<Gd<HUD>>,

    score: u32,
    base: Base<Node>
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            mob_scene: OnReady::from_loaded("res://mob.tscn"),
            player: OnReady::from_node("Player"),
            hud: OnReady::from_node("Hud"),
            score: 0,
            base
        } 
    }
    fn ready(&mut self) {
        let game = self.to_gd();
        self.clouds1().set_visible(true);
        self.clouds2().set_visible(false);
        self.clouds3().set_visible(false);
        // se o jogador for atingido o jogo acaba
        self.player
            .signals()
            .hit()
            .connect_other(&game, Self::game_over);
    
        self.start_timer()
            .signals()
            .timeout()
            .connect_other(&game, Self::on_start_timer_timeout);
    
        self.mob_timer()
            .signals()
            .timeout()
            .connect_other(&game, Self::on_mob_timer_timeout);
    
        self.score_timer()
            .signals()
            .timeout()
            .connect_other(&game, Self::on_score_timer_timeout);

        self.hud
            .signals()
            .start_game()
            .connect_other(&game, Self::new_game);
    } 
}


#[godot_api]
impl Main {
    // Nenhuma dessas funções será usada pelo Godot então não temos que adicionar o macro `#[func]`
    // encerra o jogo
    fn game_over(&mut self) {
        self.score_timer().stop();
        self.mob_timer().stop();
        self.hud.bind_mut().show_gameover();
        self.music().stop();
        self.deathsound().play();
    }
    
    fn new_game(&mut self) {
        self.clouds1().set_visible(true);
        self.clouds2().set_visible(false);
        self.clouds3().set_visible(false);
        let start_pos = self.start_position().get_position();
        self.player.bind_mut().start(start_pos);
        
        self.start_timer().start();
        self.score = 0;
        self.hud.bind_mut().set_score(self.score);
        self.hud.bind_mut().show_text("Get Ready");
        self.base().get_tree().unwrap().call_group("mobs", "queue_free", &[]);
        self.music().play();
    }

    fn start_position(&self) -> Gd<Marker2D> {
        self.base().get_node_as::<Marker2D>("StartPosition")
    }
    
    fn score_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("ScoreTimer")
    }
    
    fn start_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("StartTimer")
    }
    
    fn mob_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("MobTimer")
    }

    fn music(&self) -> Gd<AudioStreamPlayer> {
        self.base().get_node_as("Music")
    }

    fn deathsound(&self) -> Gd<AudioStreamPlayer> {
        self.base().get_node_as("DeathSound")
    }
    
    fn clouds1(&self) -> Gd<CanvasItem> {
        self.base().get_node_as("Clouds1")
    }

    fn clouds2(&self) -> Gd<CanvasItem> {
        self.base().get_node_as("Clouds2")
    }

    fn clouds3(&self) -> Gd<CanvasItem> {
        self.base().get_node_as("Clouds3")
    }

    #[func]
    fn on_score_timer_timeout(&mut self) {
        self.score += 1;
        if self.score == 10 {
            self.clouds1().set_visible(false);
            self.clouds2().set_visible(true);
        }
        if self.score == 20 {
            self.clouds2().set_visible(false);
            self.clouds3().set_visible(true);
        }
        self.hud.bind_mut().set_score(self.score);
    }
    #[func]
    fn on_start_timer_timeout(&mut self) {
        self.score_timer().start();
        self.mob_timer().start();
    }
    #[func]
    fn on_mob_timer_timeout(&mut self) {

        let mut mob_spawn_location = self.base().get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");
        let mut mob_scene = self.mob_scene.instantiate_as::<Mob>();

        let progress = rand::random_range(u32::MIN..u32::MAX) as f32;
        mob_spawn_location.set_progress(progress);
        mob_scene.set_position(mob_spawn_location.get_position());

        let mut direction = mob_spawn_location.get_rotation() + std::f32::consts::PI / 2.0;
        direction += rand::random_range((-std::f32::consts::PI / 4.0)..std::f32::consts::PI/4.0);
        mob_scene.set_rotation(direction);

        let velocity = Vector2::new(rand::random_range(150..200) as f32, 0.0);
        mob_scene.set_linear_velocity(velocity.rotated(direction));

        self.base_mut().add_child(&mob_scene);
    }
}