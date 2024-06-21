use crate::{entity::entity::Entity, resource::resource::Resource};
use macroquad::prelude::*;

#[derive(Clone)]
pub enum RobotType {
    Neutral,
    Analyse,
    Explorer,
    Forage,
    Imagerie,
}

#[derive(Clone)]
pub struct Robot {
    pub entity: Entity,
    pub is_busy: bool,
    pub mission_target: (f32, f32),
    pub robot_type: RobotType,
    pub resources: Vec<Resource>,
}

impl Robot {
    pub async fn new(robot_type: RobotType, x: f32, y: f32) -> Self {
        let speed = 0.006;

        let entity = Entity::new(x, y, speed, "assets/images/robot_0.png").await;

        Self {
            entity,
            is_busy: false,
            mission_target: (x, y), //Évite qu'il ne bouge s'il n'a pas de mission assignée
            robot_type,
            resources: Vec::new(),
        }
    }

    pub fn assign_mission(&mut self, mission_x: f32, mission_y: f32) {
        if !self.is_busy {
            println!(
                "Assignation de la mission aux coordonnées ({}, {}).",
                mission_x, mission_y
            );
            self.mission_target = (mission_x, mission_y);
            self.is_busy = true;
        }
    }

    pub fn draw(&self, x_offset: f32, y_offset: f32, zoom: f32) {
        self.entity.draw(x_offset, y_offset, zoom);
    }

    pub fn move_to(&mut self, target_x: f32, target_y: f32, obstacles: &[(usize, usize)]) {
        self.entity.move_to(target_x, target_y, obstacles);
    }

    pub async fn specialisation(&mut self, texture_path: &str, resource: Resource) {
        self.entity.set_texture(texture_path).await;
        self.resources.push(resource);
        self.is_busy = false;
    }
}
