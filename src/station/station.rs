use macroquad::audio::play_sound;
use quad_snd::PlaySoundParams;

use crate::{map::map::Map, utils::utils::generate_rand};

use super::sounds::Sounds;

#[derive(Debug, PartialEq)]
pub enum StageType {
    Initialiation,
    Analysing,
    Paused,
    Finished,
}

pub struct Station {
    pub stage: StageType,
    pub map: Map,
    pub map_size_factor: u32,
    pub epoch: i32,
    min_nb_per_resources : u32, 
    max_nb_per_resources : u32,
    name: &'static str,
    play_music: bool,
}

impl Station {
    pub async fn new(map_size_factor: u32, min_nb_per_resources: u32, max_nb_per_resources: u32) -> Self {
        Self::mission_quote(0);

        let map = Map::new(32 * map_size_factor as usize, 32 * map_size_factor as usize, min_nb_per_resources, max_nb_per_resources).await;

        Self {
            stage: StageType::Initialiation,
            map,
            map_size_factor,
            epoch: 0,
            min_nb_per_resources,
            max_nb_per_resources,
            name: "Terre-616",
            play_music: true,
        }
    }

    pub fn check_(&mut self) {
        if self.stage == StageType::Analysing {
            self.simulation_analysis();
        }
    }

    // pub fn check_simulation_stage(&mut self) {
    //     if self.stage == StageType::Analysing {
    //         self.simulation_analysis();
    //     }
    // }

    pub fn check_if_finished(&self) -> bool {
        self.stage == StageType::Finished
    }

    pub fn draw(&self, mouse_x: f32, mouse_y: f32, zoom: f32) {
        self.map.draw(mouse_x, mouse_y, zoom);
    }

    fn mission_quote(epoch: i32) {
        println!("---------------\n\n[ANNÉE-Y{}] Début de la simulation.\n\nDéploiement de la nouvelle génération d'essaim sur la planète cible...\n", 
            epoch
        );
    }

    pub async fn play_galactic_music(&self) {
        if self.play_music {
            let sounds = Sounds::load().await;

            // play_sound(
            //     &sounds.game_music.clone(),
            //     PlaySoundParams {
            //         looped: true, // Jouer la musique en boucle
            //         volume: 0.25,
            //         ..Default::default()
            //     },
            // );
        }
    }

    pub async fn simulation_start(&mut self) {
        self.map.move_robots();
        self.map.check_robot_missions().await;

        if self.map.resources.len() == 0 {
            self.stage = StageType::Analysing;
            self.simulation_analysis();
        }
    }

    pub fn simulation_analysis(&mut self) {
        println!("\nRécupération des données par {} pour leur analyse :", self.name);

        let resource_sums = self.map.count_resources();

        for (resource_type, quantity) in resource_sums {
            println!("   Resource: {:?}.", resource_type);
            println!("      Quantité: {:?}.\n", quantity);
        }

        self.stage = StageType::Finished;
    }

    pub async fn restart_new_mission(&mut self) {
        self.epoch += 1;
        self.stage = StageType::Initialiation;

        Self::mission_quote(self.epoch);

        self.map = Map::new(32 * self.map_size_factor as usize, 32 * self.map_size_factor as usize, self.min_nb_per_resources, self.max_nb_per_resources).await;
    }
}
