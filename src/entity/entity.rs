use crate::utils::utils::TILE_SIZE;
use macroquad::{
    color::WHITE,
    math::vec2,
    texture::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D},
};

#[derive(Clone, PartialEq)]
pub struct Entity {
    pub pos_x: f32,
    pub pos_y: f32,
    pub speed: f32, // speed > 0 = déplacement
    pub texture: Texture2D,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Entity {
    pub async fn new(pos_x: f32, pos_y: f32, speed_factor: f32, texture_path: &str) -> Self {
        let texture = load_texture(texture_path).await.unwrap();

        Self {
            pos_x: pos_x,
            pos_y: pos_y,
            speed: speed_factor * TILE_SIZE, //Ajuster la vitesse de déplacement en fonction de la taille des cases
            texture: texture,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }


    pub fn position(&self) -> (f32, f32) {
        (self.pos_x, self.pos_y)
    }
    /*
        Le mouvement est équivalent à un vecteur de vitesse.
        Il faut normaliser le vecteur direction entre notre position actuelle et notre cible
        pour savoir dans quelle direction nous déplacer.
    */
    pub fn move_to(&mut self, target_x: f32, target_y: f32, obstacles: &[(usize, usize)]) {
        let direction_x = target_x - self.pos_x;
        let direction_y = target_y - self.pos_y;

        let distance = (direction_x.powi(2) + direction_y.powi(2)).sqrt();

        if distance > 0.0 {
            let direction_x = direction_x / distance;
            let direction_y = direction_y / distance;

            let new_pos_x = self.pos_x + direction_x * self.speed;
            let new_pos_y = self.pos_y + direction_y * self.speed;

            let tile_x = (new_pos_x / TILE_SIZE).round() as usize;
            let tile_y = (new_pos_y / TILE_SIZE).round() as usize;

            if !obstacles.contains(&(tile_x, tile_y)) {
                self.pos_x = new_pos_x;
                self.pos_y = new_pos_y;
            } else {
                // Chercher une position alternative si il y a un obstacle ou du vide
                let alternatives = [
                    (self.pos_x + direction_x * self.speed, self.pos_y),
                    (self.pos_x, self.pos_y + direction_y * self.speed),
                    (self.pos_x - direction_x * self.speed, self.pos_y),
                    (self.pos_x, self.pos_y - direction_y * self.speed),
                ];

                for &(alt_x, alt_y) in &alternatives {
                    let alt_tile_x = (alt_x / TILE_SIZE).round() as usize;
                    let alt_tile_y = (alt_y / TILE_SIZE).round() as usize;

                    if !obstacles.contains(&(alt_tile_x, alt_tile_y)) {
                        self.pos_x = alt_x;
                        self.pos_y = alt_y;
                        break;
                    }
                }
            }
        }
    }

    // Dessiner notre objet
    pub fn draw(&self, x_offset: f32, y_offset: f32, zoom: f32) {
        draw_texture_ex(
            &self.texture,
            self.pos_x * zoom + x_offset,
            self.pos_y * zoom + y_offset,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(TILE_SIZE * zoom, TILE_SIZE * zoom)),
                ..Default::default()
            },
        );
    }

    pub async fn set_texture(&mut self, texture_path: &str) {
        self.texture = load_texture(texture_path).await.unwrap();
    }
}
