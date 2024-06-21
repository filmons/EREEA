 
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
    epoch: i32,
    name: &'static str,
    play_music: bool,
}

impl Station {
    pub async fn new() -> Self {
        //La Seed est aléatoire, mais la map reproductible grâce au numéro de la seed comme dans Minecraft.
        let map_seed = generate_rand(42, 42);

        let map = Map::new(32 * 2, 32 * 2, map_seed).await;

        println!("Début de la simulation, année {}", 0);

        Self {
            stage: StageType::Initialiation,
            map,
            epoch: 0,
            name: "Terre-616",
            play_music: true,
        }
    }

    pub fn check_simulation_stage(&mut self) {
        if self.stage == StageType::Analysing {
            self.simulation_analysis();
        }
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

        //tmp
        if self.map.resources.len() < 9 {
            self.stage = StageType::Analysing;
        }
    }

    pub fn simulation_analysis(&mut self) {
        println!(
            "[ANNÉE-Y{}] Récupération des données par {} pour leur analyse",
            self.epoch,
            self.name
        );

        let mut collected_resources = Vec::new();

        let robots = self.map.robots.lock().unwrap();
        for robot in &*robots {
            for resource in &robot.resources {
                collected_resources.push(resource.clone());
            }
        }

        for resource in &collected_resources {
            println!("  Resource: {:?}, Quantité: {:?}", resource.resource_type, 1);
        }

        self.stage = StageType::Finished;
    }

    //MODE GRAPHIQUE
    pub fn draw(&self, mouse_x: f32, mouse_y: f32, zoom: f32) {
        self.map.draw(mouse_x, mouse_y, zoom);
    }

    //MODE TEXTUEL
    pub fn draw_terminal(&self) {
        self.map.draw_terminal();
    }
}
