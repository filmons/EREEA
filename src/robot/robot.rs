use crate::resources::resources::Resource;

#[derive(Clone)]
pub enum RobotType {
    Neutral,
    Analyse,
    Forage,
    Imagerie,
}

#[derive(Clone)]
pub struct Robot {
    pub is_busy: bool,
    pub robot_type: RobotType,
    pub resources: Vec<Resource>,
    pub pos_x: usize,
    pub pos_y: usize,
}

impl Robot {
    pub fn new(robot_type: RobotType, x: usize, y: usize) -> Self {
        Self {
            is_busy: false,
            robot_type,
            resources: Vec::new(),
            pos_x : x,
            pos_y : y
        }
    }

    pub fn assign_mission(&mut self) {
        if !self.is_busy {
            println!("Assignation de la mission au robot aux coordonnées ({}, {}).", self.pos_x, self.pos_y);
            self.is_busy = true;
        }
    }
}
