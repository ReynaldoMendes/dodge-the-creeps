use godot::prelude::*;
use godot::classes::{Button, CanvasLayer, ICanvasLayer, Label, Timer};


#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct HUD {
    base: Base<CanvasLayer>
}

#[godot_api]
impl ICanvasLayer for HUD {
    
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }

    fn ready(&mut self) {

        self.start_button()
            .signals()
            .pressed()
            .connect_other(self, Self::on_start_button_pressed);

        self.message_timer()
            .signals()
            .timeout()
            .connect_other(self, Self::on_message_timer_timeout);
    }
}

#[godot_api]
impl HUD {

    #[signal]
    pub fn start_game();
    
    
    pub fn set_score(&mut self, score: u32) {
        self.score_label().set_text(score.to_string().as_str());
    }
    
    pub fn show_text(&mut self, text: &str) {
        let mut msg_label = self.message_label();
        msg_label.set_text(text);
        msg_label.show();

        self.message_timer().start();
    }
    
    pub fn show_gameover(&mut self) {

        self.show_text("Game Over!");

        // criamos um timer de uso unico e esperamos ele acabar
        let timer = self.base().get_tree().unwrap().create_timer(2.0).unwrap();
        timer.signals()
            .timeout()
            .connect_other(self, Self::show_start_button);
    }

    fn show_start_button(&mut self) {
        let mut msg = self.message_label();
        let mut mensagem = self.message2_label();
        msg.set_text("Dodge the Creeps!");
        mensagem.set_text("Aperte 'E' para usar seu ultimate.");
        msg.show();
        mensagem.show();

        self.start_button().show();
    }
    
    #[func]
    fn on_start_button_pressed(&mut self) {
        self.start_button().hide();
        self.signals().start_game().emit();
    }

    #[func]
    fn on_message_timer_timeout(&mut self) {
        self.message_label().hide();
        self.message2_label().hide();
    }

    fn message_label(&self) -> Gd<Label> {
        self.base().get_node_as("Message")
    }

    fn message2_label(&self) -> Gd<Label> {
        self.base().get_node_as("Message2")
    }

    fn score_label(&self) -> Gd<Label> {
        self.base().get_node_as("ScoreLabel")
    }

    fn start_button(&self) -> Gd<Button> {
        self.base().get_node_as("StartButton")
    }

    fn message_timer(&self) -> Gd<Timer> {
        self.base().get_node_as("MessageTimer")
    }
}