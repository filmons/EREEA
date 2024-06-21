use macroquad::audio::{load_sound, Sound};

#[derive(Clone)]
pub struct Sounds {
    pub game_music: Sound,
}

impl Sounds {
    pub async fn load() -> Self {
        let game_music = load_sound("assets/sounds/music.wav").await.unwrap();

        Self { game_music }
    }
}
