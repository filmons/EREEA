use crate::{map::{self, map::Map}, robot::robot::Robot, utils::utils::generate_rand};

pub enum StageType {
    Initialiation,
    Analysing,
    Paused,
    Finished,
}

pub struct Station {
    pub stage: StageType,
    pub map: Map,
    epoch: i32,
    name: &'static str,
}

impl Station {
    pub fn new() -> Self {
        //La Seed est aléatoire, mais la map reproductible grâce au numéro de la seed comme dans Minecraft.
        let map_seed = generate_rand(1, 100);
        let map_width: usize = 32;
        let map_height: usize = 32;

        //La station commence avec une Map initiale pour sa mission 0
        let map = Map::new(map_seed, map_width, map_height, 5, 4, 2, 3);

        Self {
            stage: StageType::Initialiation,
            map: map,
            epoch: 0,
            name: "Terre-616",
        }
    }

    pub fn simulation_start(&mut self) {
        println!("Début de la simulation, année {}", self.epoch);

        map::map::Map::display_map(&self.map);

        let closest_robots = &self.map.find_closest_robots();

        println!("Closest Robots to Resources:");

        for ((res_x, res_y), (robot_x, robot_y)) in closest_robots.iter() {
            if let Some(robot) = self.map.robots[*robot_y][*robot_x].as_mut() {
                robot.assign_mission();
            }
            println!("Resource at ({}, {}): Closest Robot at ({}, {})", res_x, res_y, robot_x, robot_y);
        }
    }
}
