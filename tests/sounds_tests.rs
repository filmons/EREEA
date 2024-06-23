use macroquad::audio::{load_sound, Sound};
use macroquad::prelude::*;
use ereea::station::sounds::Sounds;

#[macroquad::main("Sound Test")]
async fn main() {
    let result = load_sound("./assets/sounds/music.wav").await;

    match result {
        Ok(sound) => {
            println!("Le son a été chargé avec succès!");
        },
        Err(error) => {
            eprintln!("Erreur lors du chargement du son : {:?}", error);
            panic!("Impossible de charger le son.");
        }
    }

    next_frame().await;
    next_frame().await;
}
