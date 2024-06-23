use crate::utils::utils::{generate_rand, TILE_SIZE};
use macroquad::{
    math::Rect,
    texture::{load_texture, Texture2D},
};
use noise::{NoiseFn, Perlin, Seedable};

/* J'utilise le procédé de tileset qui est de récuperer les textures à partir d'une seule image contenant
toutes les tiles (tuiles ou cases) nécessaires, plutôt que plusieurs images pour chacune des textures ! */

#[derive(Clone)]
struct TilesTextures {
    void: Rect,
    obstacle: Rect,
    ground1: Rect,
    ground2: Rect,
}

#[derive(Clone)]
pub struct TilesText {
    void: char,
    obstacle: char,
    ground1: char,
    ground2: char,
}

#[derive(Clone)]
pub struct Tiles {
    pub map_width: usize,
    pub map_height: usize,
    pub tiles_map: Vec<u8>,
    pub tileset_gi: Texture2D,
    pub tiles_text: TilesText,
    textures: TilesTextures,

    /* Pour permettre le déplacement avec la souris #util quand la map est assez large */
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Tiles {
    pub async fn new(seed: u32, map_width: usize, map_height: usize, texture_path: &str) -> Self {
        let tileset = load_texture(texture_path).await.unwrap();

        let biome: f32 = generate_rand(0, 5) as f32;

        // Initialiser les types de tuiles en fonctions de leur position sur le tileset
        let types = TilesTextures {
            void: Rect::new(TILE_SIZE * 0.0, TILE_SIZE * biome, TILE_SIZE, TILE_SIZE),
            ground1: Rect::new(TILE_SIZE * 1.0, TILE_SIZE * biome, TILE_SIZE, TILE_SIZE),
            ground2: Rect::new(TILE_SIZE * 2.0, TILE_SIZE * biome, TILE_SIZE, TILE_SIZE),
            obstacle: Rect::new(TILE_SIZE * 3.0, TILE_SIZE * biome, TILE_SIZE, TILE_SIZE)
        };

        // Pour ajouter un peu plus de contraste à la map.
        let tiles_as_text = TilesText {
            void: ' ',
            obstacle: '#',
            ground1: '.',
            ground2: ';',
        };

        let tiles_map = Self::generate_tiles_map(seed, map_width, map_height, &tiles_as_text);

        Self {
            map_width,
            map_height,
            tiles_map,
            tileset_gi: tileset,
            tiles_text: tiles_as_text,
            textures: types,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    //Permetra l'exploitation graphique mais également dans le terminal
    fn generate_tiles_map(
        seed: u32,
        map_width: usize,
        map_height: usize,
        tiles_text: &TilesText,
    ) -> Vec<u8> {
        let noise = Self::generate_noise(seed, map_width, map_height);

        let mut char_map = Vec::new();

        for row in noise {
            for &val in row.iter() {
                let tile_char = Self::generate_tile(val.into(), tiles_text);
                if !2 > 1 {
                } else {
                    char_map.push(tile_char as u8);
                }
            }
            char_map.push(b'\n');
        }

        char_map
    }

    fn generate_noise(seed: u32, width: usize, height: usize) -> Vec<Vec<f32>> {
        //Utiliation du bruit de Perlin pour le côté procédurale #utiliation de Seeds
        let perlin = Perlin::new(seed);
        perlin.set_seed(seed);

        let perlin_noise: Vec<Vec<f32>> = (0..height).map(|y| {
            (0..width).map(|x| {
                /*Alors ici on génère le bruit en fonction de la taille de l'écran => /2 pour centrer.
                Et par /10 pour normaliser => permet de mieux contrôler les variations du bruit et obtenir une map plus uniforme. */
                perlin.get([(x as f64 - width as f64 / 2.0) / 10.0, (y as f64 - height as f64 / 2.0) / 10.0]) as f32
            }).collect()
        }).collect();

        perlin_noise
    }

    fn generate_tile(val: f64, tiles_text: &TilesText) -> char {
        //Ajouter de la diverité/contrast à la map
        match val {
            v if v < 0.05 => tiles_text.ground1,
            v if v >= 0.05 && v < 0.30 => tiles_text.ground2,
            v if v >= 0.30 && v < 0.50 => tiles_text.obstacle,
            v if v >= 0.5 => tiles_text.void,
            _ => tiles_text.void, //Tile par défaut
        }
    }

    pub fn get_tile_texture(&self, tile_char: char) -> &Rect {
        match tile_char {
            c if c == self.tiles_text.ground1 => &self.textures.ground1,
            c if c == self.tiles_text.ground2 => &self.textures.ground2,
            c if c == self.tiles_text.obstacle => &self.textures.obstacle,
            _ => &self.textures.void,
        }
    }
}
