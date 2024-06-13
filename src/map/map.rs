use std::{collections::HashMap, vec};

use super::tiles::{Tile, TileType};

use noise::{NoiseFn, Perlin, Seedable};
use crate::utils::utils::generate_rand;
use crate::resources::resources::{Resource, ResourceType};
use crate::robot::robot::{Robot, RobotType};

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub resources: Vec<Vec<Option<Resource>>>,
    pub robots: Vec<Vec<Option<Robot>>>,
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

        let tiles: Vec<Vec<Tile>> = noise
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &val)| {
                    // Pour ajouter un peu plus de contraste à la map.
                    let tile_type = match val {
                        v if v < 0.05 => TileType::Void,
                        v if v < 0.35 => TileType::Obstacle,
                        v if v < 0.55 || v >= 0.65 && v < 0.75 || v >= 0.85 && v < 0.95 => {
                            TileType::Ground1
                        }
                        v if v < 0.60 || v >= 0.70 && v < 0.80 => TileType::Ground2,
                        v if v < 0.70 || v >= 0.80 && v < 0.90 => TileType::Ground3,
                        v if v < 0.80 || v >= 0.90 && v < 1.0 => TileType::Ground4,
                        _ => TileType::Ground1,
                    };
                    Tile::new(tile_type, x, y)
                })
                .collect()
        })
        .collect();

        let resources: Vec<Vec<Option<Resource>>> = vec![vec![None; width]; height];
        let robots: Vec<Vec<Option<Robot>>> = vec![vec![None; width]; height];

        let mut map = Self {
            tiles,
            resources,
            robots,
            width,
            height,
        };

        map.add_resources(nb_initial_energies, ResourceType::Energie);
        map.add_resources(nb_initial_minerals, ResourceType::Minerai);
        map.add_resources(nb_initial_places, ResourceType::Lieu);
        map.add_resources(nb_initial_robots, ResourceType::Lieu);
        map.add_robots(nb_initial_robots);

        map
    }

    pub fn add_resources(&mut self, nb_items: usize, resource_type: ResourceType) {
        for _ in 0..nb_items {
            if let Some((free_x, free_y)) = Self::get_free_tiles(&self.tiles) {
                self.resources[free_y][free_x] = Some(Resource::new(resource_type.clone()));
            }
        }
    }

    pub fn add_robots(&mut self, nb_items: usize) {
        for _ in 0..nb_items {
            if let Some((free_x, free_y)) = Self::get_free_tiles(&self.tiles) {
                self.robots[free_y][free_x] = Some(Robot::new(RobotType::Neutral, free_x, free_y));
            }
        }
    }

    pub fn display_map(map: &Map) {
        for (y, row) in map.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                // Si un robot existe à cet emplacement
                if let Some(resource) = &map.resources[y][x] {
                    // Si la ressource n'est pas consommée : éviter les futurs pb de synchro affichage/extraction
                    if !resource.is_consumed {
                        match resource.resource_type {
                            ResourceType::Energie => print!("🗲"),
                            ResourceType::Minerai => print!("◯"),
                            ResourceType::Lieu => print!("▩"),
                        }
                    } else {
                        // Si la ressource est consommée
                        print!("X"); // Utilisez le symbole de votre choix pour une ressource consommée
                    }
                } else if let Some(robot) = &map.robots[y][x] {
                    // Afficher un emoji en fonction du type de robot
                    match robot.robot_type {
                        RobotType::Neutral => print!("🤖"),
                        RobotType::Analyse => print!("🔬"),
                        RobotType::Forage => print!("🛠️"),
                        RobotType::Imagerie => print!("📸"),
                    }
                } else {
                    // Aucune ressource à cet emplacement
                    print!("{}", tile.symbol);
                }
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

        for (y, row) in tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if matches!(
                    tile.tile_type,
                    TileType::Ground1 | TileType::Ground2 | TileType::Ground3 | TileType::Ground4
                ) {
                    free_spaces.push((x, y));
                }
            }
        }

        if !free_spaces.is_empty() {
            let index = generate_rand(0, free_spaces.len() as u32);
            Some(free_spaces[index as usize])
        } else {
            None
        }
    }

    pub fn find_closest_robot(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let mut min_distance = usize::MAX;
        let mut closest_robot: Option<(usize, usize)> = None;

        for (row_index, row) in self.robots.iter().enumerate() {
            for (col_index, robot) in row.iter().enumerate() {
                if let Some(robot) = robot {
                    let distance = self.calculate_distance(x, y, robot.pos_x, robot.pos_y);
                    if distance < min_distance {
                        min_distance = distance;
                        closest_robot = Some((col_index, row_index));
                    }
                }
            }
        }

        closest_robot
    }

    fn calculate_distance(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
        ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as usize
    }

    pub fn find_closest_robots(&mut self) -> HashMap<(usize, usize), (usize, usize)> {
        let mut closest_robots = HashMap::new();
    
        // Vérifier si tous les robots sont occupés d'abord
        let all_robots_busy = self.robots.iter().all(|row| {
            row.iter().all(|robot| {
                if let Some(robot) = robot {
                    robot.is_busy
                } else {
                    false
                }
            })
        });
    
        // Si c'est le cas on crée un nouveau robot
        if all_robots_busy {
            println!("{}", "Tous les robots de l'essaim sont occupés sont occupés, appel à un nouveau robot...");
            self.add_robots(1);
        }
    
        for (row_index, row) in self.resources.iter().enumerate() {
            for (col_index, resource) in row.iter().enumerate() {
                if let Some(resource) = resource {
                    if !resource.is_consumed {
                        if let Some((robot_x, robot_y)) = self.find_closest_robot(col_index, row_index) {
                            closest_robots.insert((col_index, row_index), (robot_x, robot_y));
                        }
                    }
                }
            }
        }

        closest_robots
    }
}