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
    pub robot_type: RobotType,
    pub resources: Vec<Resource>,
    pub posx: usize,
    pub posy: usize,
}

impl Robot {
    pub fn new(robot_type: RobotType, posx: usize, posy: usize) -> Self {
        Self { 
            robot_type,
            resources: Vec::new(),
            posx,
            posy,
        }
    }
}
