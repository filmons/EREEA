use macroquad::audio::{load_sound, play_sound_once, Sound};

#[derive(Clone)]
pub struct Sounds {
    pub game_music: Sound,
}

impl Sounds {
    pub async fn load_music() -> Self {
        let game_music = load_sound("assets/sounds/music.wav").await.unwrap();
        Self { game_music }
    }

    pub fn play_galactic_music(&self) {
        play_sound_once(&self.game_music);
    }
}
