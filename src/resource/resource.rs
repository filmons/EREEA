use crate::entity::entity::Entity;
use macroquad::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Energie,
    Lieu,
    Minerai,
}

#[derive(Clone, PartialEq)]
pub struct Resource {
    pub entity: Entity,
    pub resource_type: ResourceType,
    pub is_consumed: bool,
}

impl Resource {
    pub async fn new(resource_type: ResourceType, x: f32, y: f32) -> Self {
        let speed = 0.0; //Immobile

        let texture_path = match resource_type {
            ResourceType::Energie => "assets/images/resource_energy.png",
            ResourceType::Minerai => "assets/images/resource_mineral.png",
            ResourceType::Lieu => "assets/images/resource_place.png",
        };

        let entity = Entity::new(x, y, speed, texture_path).await;

        Self {
            entity,
            resource_type,
            is_consumed: false,
        }
    }

    pub fn set_consumed(&mut self) {
        self.is_consumed = true;
    }

    pub fn draw(&self, x_offset: f32, y_offset: f32, zoom: f32) {
        self.entity.draw(x_offset, y_offset, zoom);
    }
}
