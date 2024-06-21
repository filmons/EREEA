use super::tiles::Tiles;

use crate::{
    resource::resource::{Resource, ResourceType},
    robot::robot::{Robot, RobotType},
    utils::utils::{generate_rand, TILE_SIZE},
};

use macroquad::{
    color::WHITE,
    texture::{draw_texture_ex, DrawTextureParams},
    window::{screen_height, screen_width},
};

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Map {
    pub tile_map: Tiles,
    pub robots: Arc<Mutex<Vec<Robot>>>,
    pub resources: Vec<Resource>,
    pub obstacles: Vec<(usize, usize)>, // pub resources: Vec<Resource>,
}

impl Map {
    pub async fn new(width: usize, height: usize, seed: u32) -> Self {
        let tiles = Tiles::new(seed, height, width, "assets/images/tileset.png").await;

        let mut obstacles = Vec::new();

        //On recherche les obstacles et emplacements vide de la map
        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                let tile = tiles.tiles_map[index];

                if tile == b' ' || tile == b'#' {
                    obstacles.push((x, y));
                }
            }
        }

        let robots = Arc::new(Mutex::new(Vec::new()));
        let resources = Vec::new();

        let mut map = Self {
            tile_map: tiles,
            robots: Arc::clone(&robots),
            resources,
            obstacles,
        };

        let nb_initial_energies: u32 = 3;
        let nb_initial_minerals: u32 = 3;
        let nb_initial_places: u32 = 3;

        // 1 robot pour 3 ressources pour mieux profiter de la simulation
        let nb_initial_robots =
            ((nb_initial_energies + nb_initial_minerals + nb_initial_places) as f32 / 3.0).ceil()
                as u32;

        map.add_resources(nb_initial_energies, ResourceType::Energie)
            .await;
        map.add_resources(nb_initial_minerals, ResourceType::Minerai)
            .await;
        map.add_resources(nb_initial_places, ResourceType::Lieu)
            .await;

        map.add_robot(nb_initial_robots).await;

        map.assign_missions();

        map
    }

    pub async fn add_resources(&mut self, nb_items: u32, resource_type: ResourceType) {
        let free_tiles = self.get_free_tiles();

        for _ in 0..nb_items {
            if !free_tiles.is_empty() {
                let index = generate_rand(0, free_tiles.len() as u32); // Sélectionne un index aléatoire parmis les cases libres
                let (free_x, free_y) = free_tiles[index as usize];
                let resource = Resource::new(resource_type.clone(), free_x as f32, free_y as f32);
                self.resources.push(resource.await);
            }
        }
    }

    pub async fn add_robot(&mut self, nb_items: u32) {
        let free_tiles = self.get_free_tiles();

        for _ in 0..nb_items {
            if !free_tiles.is_empty() {
                let index = generate_rand(0, free_tiles.len() as u32); // Sélectionne un index aléatoire parmis les cases libres
                let (free_x, free_y) = free_tiles[index as usize];
                let robot = Robot::new(RobotType::Neutral, free_x as f32, free_y as f32).await;
                self.robots.lock().unwrap().push(robot);
            }
        }
    }

    // Assigner des missions aux robots en fonction des ressources
    pub fn assign_missions(&mut self) {
        for resource in &self.resources {
            if !resource.is_consumed {
                let x = resource.entity.pos_x as usize;
                let y = resource.entity.pos_y as usize;

                if let Some(robot_index) = self.find_closest_robot(x, y) {
                    let mut robots = self.robots.lock().unwrap(); // Verrouiller l'accès aux robots
                    let robot = &mut robots[robot_index];
                    robot.assign_mission(x as f32, y as f32);
                }
            }
        }
    }

    //On cherche le robot libre le plus proche de la mission souhaitée
    pub fn find_closest_robot(&self, x: usize, y: usize) -> Option<usize> {
        let mut min_distance = usize::MAX;
        let mut closest_robot: Option<usize> = None;

        let robots = self.robots.lock().unwrap();
        for (index, robot) in robots.iter().enumerate() {
            if !robot.is_busy {
                // Calcul de la distance entre le robot et les coordonnées données
                let distance = ((x as isize - robot.entity.pos_x as isize).abs()
                    + (y as isize - robot.entity.pos_y as isize).abs())
                    as usize;

                if distance < min_distance {
                    min_distance = distance;
                    closest_robot = Some(index);
                }
            }
        }

        closest_robot
    }

    pub fn draw(&self, mouse_x: f32, mouse_y: f32, zoom: f32) {
        let mut x = -(mouse_x - screen_width() / 2.0);
        let mut y = -(mouse_y - screen_height() / 2.0);

        // Coéfficient de Zoom
        x *= zoom;
        y *= zoom;

        let max_offset_x = -(self.tile_map.map_width as f32 * TILE_SIZE - screen_width()) * zoom;
        let max_offset_y = -(self.tile_map.map_height as f32 * TILE_SIZE - screen_height()) * zoom;

        // S'assurer que l'on ne sorte pas de la map
        if x > 0.0 {
            x = 0.0;
        } else if x < max_offset_x {
            x = max_offset_x;
        }

        if y > 0.0 {
            y = 0.0;
        } else if y < max_offset_y {
            y = max_offset_y;
        }

        // Dessiner le résultat
        for (index, &tile_char) in self.tile_map.tiles_map.iter().enumerate() {
            let tile_x = (index % self.tile_map.map_width) as f32 * TILE_SIZE * zoom
                + self.tile_map.offset_x
                + x;
            let tile_y = (index / self.tile_map.map_width) as f32 * TILE_SIZE * zoom
                + self.tile_map.offset_y
                + y;

            let texture_rect = self.tile_map.get_tile_texture(tile_char as char);

            draw_texture_ex(
                &self.tile_map.tileset_gi,
                tile_x,
                tile_y,
                WHITE,
                DrawTextureParams {
                    source: Some(*texture_rect),
                    ..Default::default()
                },
            );
        }

        // Dessiner les ressources
        for resource in self.resources.iter() {
            if !resource.is_consumed {
                let resource_x =
                    resource.entity.pos_x * TILE_SIZE * zoom + self.tile_map.offset_x + x;
                let resource_y =
                    resource.entity.pos_y * TILE_SIZE * zoom + self.tile_map.offset_y + y;
                resource.draw(resource_x, resource_y, zoom);
            }
        }

        // Dessiner les robots en tenant compte des offsets et du zoom
        let robots = self.robots.lock().unwrap();
        for robot in robots.iter() {
            let robot_x = robot.entity.pos_x * TILE_SIZE * zoom + self.tile_map.offset_x + x;
            let robot_y = robot.entity.pos_y * TILE_SIZE * zoom + self.tile_map.offset_y + y;
            robot.draw(robot_x, robot_y, zoom);
        }
    }

    pub fn draw_terminal(&self) {
        for &tile in &self.tile_map.tiles_map {
            print!("{}", tile as char);
        }
    }

    //Obtenir toutes les cases qui ne sont ni obstacle ni vide pour faire pop nos objets
    pub fn get_free_tiles(&self) -> Vec<(usize, usize)> {
        let mut free_spaces = Vec::new();

        for y in 0..self.tile_map.map_height {
            for x in 0..self.tile_map.map_width {
                let index = y * self.tile_map.map_width + x;
                let tile = self.tile_map.tiles_map[index];

                if tile != b' ' && tile != b'#' {
                    // Éviter le vide et les obstacles
                    let robots = self.robots.lock().unwrap();

                    if !robots.iter().any(|robot| {
                        (robot.entity.pos_x as usize, robot.entity.pos_y as usize) == (x, y)
                    }) {
                        free_spaces.push((x, y));
                    }
                }
            }
        }

        free_spaces
    }

    pub async fn check_robot_missions(&mut self) {
        let mut robots = self.robots.lock().unwrap(); // Verrouiller l'accès aux robots

        for resource in self.resources.iter_mut() {
            for robot in robots.iter_mut() {
                if robot.is_busy &&
                    (robot.mission_target.0 == resource.entity.pos_x &&
                    robot.mission_target.1 == resource.entity.pos_y) &&

                    // Vérifier si le robot est proche de sa cible avec une marge de tolérance
                    ((robot.entity.pos_x - resource.entity.pos_x).abs() < 4.0 &&
                    (robot.entity.pos_y - resource.entity.pos_y).abs() < 4.0)
                {
                    resource.set_consumed();

                    match resource.resource_type {
                        // Spécialiser le robot en fonction du type de ressource
                        ResourceType::Minerai => {
                            robot
                                .specialisation("assets/images/robot_1.png", resource.clone())
                                .await;
                        }
                        ResourceType::Lieu => {
                            robot
                                .specialisation("assets/images/robot_2.png", resource.clone())
                                .await;
                        }
                        ResourceType::Energie => {
                            robot
                                .specialisation("assets/images/robot_3.png", resource.clone())
                                .await;
                        }
                    }
                }
            }
        }

        // Retirer les resources consommées du vecteur
        self.resources.retain(|resource| !resource.is_consumed);
    }

    pub fn move_robots(&self) {
        let mut robots = self.robots.lock().unwrap();

        for robot in robots.iter_mut() {
            if robot.is_busy {
                robot.move_to(
                    robot.mission_target.0,
                    robot.mission_target.1,
                    &self.obstacles,
                );
            }
        }
    }
}
