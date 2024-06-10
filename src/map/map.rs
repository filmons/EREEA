use std::vec;

use noise::{NoiseFn, Perlin, Seedable};

use crate::utils::utils::generate_rand;

use super::resources::{Resource, ResourceType};

#[derive(Clone)]
pub enum Tile {
    Energie,
    Obstacle,
    Ground1,
    Ground2,
    Ground3,
    Ground4,
    Lieu,
    Minerai,
    Robot,
    Void,
}

impl Tile {
    pub fn symbol(&self) -> char {
        match *self {
            Tile::Energie => '🗲',
            Tile::Obstacle => '#',
            Tile::Ground1 => '.',
            Tile::Ground2 => ',',
            Tile::Ground3 => ';',
            Tile::Ground4 => ':',
            Tile::Lieu => '▩',
            Tile::Minerai => '◯',
            Tile::Robot => '🤖',
            Tile::Void => ' ',
        }
    }

    pub fn name(&self) -> &'static str {
        match *self {
            Tile::Energie => "Energie",
            Tile::Obstacle => "Obstacle",
            Tile::Ground1 => "Ground 1",
            Tile::Ground2 => "Ground 2",
            Tile::Ground3 => "Ground 3",
            Tile::Ground4 => "Ground 4",
            Tile::Lieu => "Lieux d'intérêt scientifique",
            Tile::Minerai => "Minerai",
            Tile::Robot => "Robot",
            Tile::Void => "Espace",
        }
    }
}

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub resources: Vec<Vec<Option<Resource>>>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(
        seed: u32,
        width: usize,
        height: usize,
        nb_initial_energies: usize,
        nb_initial_minerals: usize,
        nb_initial_places: usize,
        nb_initial_robots: usize,
    ) -> Self {
        //Je n'ai pas mis frame bordure exprès pour l'effet sphérique à implémenter plus tard...

        let noise = Self::generate_noise(seed, width, height);

        let mut tiles: Vec<Vec<Tile>> = noise
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&val| {
                        // Pour ajouter un peu plus de contraste à la map.
                        match val {
                            v if v < 0.05 => Tile::Void,
                            v if v < 0.35 => Tile::Obstacle,
                            v if v < 0.55 || v >= 0.65 && v < 0.75 || v >= 0.85 && v < 0.95 => {
                                Tile::Ground1
                            }
                            v if v < 0.60 || v >= 0.70 && v < 0.80 => Tile::Ground2,
                            v if v < 0.70 || v >= 0.80 && v < 0.90 => Tile::Ground3,
                            v if v < 0.80 || v >= 0.90 && v < 1.0 => Tile::Ground4,
                            _ => Tile::Ground1,
                        }
                    })
                    .collect()
            })
            .collect();

        let mut resources: Vec<Vec<Option<Resource>>> = vec![vec![None; width]; height];

        for _ in 0..nb_initial_energies {
            if let Some((free_x, free_y)) = Self::get_free_tiles(&tiles) {
                resources[free_y][free_x] = Some(Resource::new(ResourceType::Energie));
                tiles[free_y][free_x] = Tile::Energie;
            }
        }

        for _ in 0..nb_initial_minerals {
            if let Some((free_x, free_y)) = Self::get_free_tiles(&tiles) {
                resources[free_y][free_x] = Some(Resource::new(ResourceType::Minerai));
                tiles[free_y][free_x] = Tile::Minerai;
            }
        }

        for _ in 0..nb_initial_places {
            if let Some((free_x, free_y)) = Self::get_free_tiles(&tiles) {
                resources[free_y][free_x] = Some(Resource::new(ResourceType::Lieu));
                tiles[free_y][free_x] = Tile::Lieu;
            }
        }

        for _ in 0..nb_initial_robots {
            if let Some((free_x, free_y)) = Self::get_free_tiles(&tiles) {
                tiles[free_y][free_x] = Tile::Robot;
            }
        }

        Self {
            tiles: tiles.to_vec(),
            resources,
            width,
            height,
        }
    }

    pub fn display_map(map: &Map) {
        for row in &map.tiles {
            for tile in row {
                print!("{}", tile.symbol());
            }
            println!();
        }
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

    pub fn get_free_tiles(tiles: &Vec<Vec<Tile>>) -> Option<(usize, usize)> {
        let mut free_spaces = Vec::new();

        // On vérifie tous les emplacements libres
        for (y, row) in tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if matches!(
                    tile,
                    Tile::Ground1 | Tile::Ground2 | Tile::Ground3 | Tile::Ground4
                ) {
                    free_spaces.push((x, y));
                }
            }
        }

        let tab_len = free_spaces.len().try_into().unwrap();

        // On choisit un emplacement libre au hasard
        if !free_spaces.is_empty() {
            let index = generate_rand(0, tab_len);
            Some(free_spaces[index as usize])
        } else {
            None
        }
    }
}
