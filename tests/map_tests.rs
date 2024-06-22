use ereea::robot::robot::RobotType;
use macroquad::prelude::*;


#[cfg(test)]
mod tests {
    use super::*;
    use ereea::{map::map::Map, resource::resource::ResourceType};
    use macroquad::rand::ChooseRandom;

    #[macroquad::test]
    async fn test_map_creation() {
        let width = 20;
        let height = 20;
        let seed = 12345;

        let map = Map::new(width, height, seed).await;

        // Vérifier que la carte a été correctement initialisée
        assert_eq!(map.tile_map.map_width, width);
        assert_eq!(map.tile_map.map_height, height);
        assert_eq!(map.resources.len(), 9); 
        assert_eq!(map.robots.lock().unwrap().len(), 3);

        assert!(!map.obstacles.is_empty());
    }

    #[macroquad::test]
    async fn test_add_resources() {
        let width = 20;
        let height = 20;
        let seed = 12345;

        let mut map = Map::new(width, height, seed).await;

        map.add_resources(1, ResourceType::Energie).await;
        assert_eq!(map.resources.len(), 10);
        map.add_resources(2, ResourceType::Minerai).await;
        assert_eq!(map.resources.len(), 12);
    }

    #[macroquad::test]
    async fn test_add_robot() {
        let width = 20;
        let height = 20;
        let seed = 12345;

        let mut map = Map::new(width, height, seed).await;

        map.add_robot(1).await;
        assert_eq!(map.robots.lock().unwrap().len(), 4);
        map.add_robot(2).await;

        assert_eq!(map.robots.lock().unwrap().len(), 6);
    }

    #[macroquad::test]
    async fn test_assign_missions() {
        let width = 20;
        let height = 20;
        let seed = 12345;

        let mut map = Map::new(width, height, seed).await;

        map.assign_missions();

        let robots = map.robots.lock().unwrap();
        for robot in robots.iter() {
            assert!(robot.is_busy);
        }
    }

    #[macroquad::test]
    async fn test_move_robots() {
        let width = 20;
        let height = 20;
        let seed = 12345;

        let map = Map::new(width, height, seed).await;

        let mut robots = map.robots.lock().unwrap();
        let initial_positions: Vec<(f32, f32)> = robots.iter().map(|robot| (robot.entity.pos_x, robot.entity.pos_y)).collect();

        for robot in robots.iter_mut() {
            if !robot.is_busy {
                let free_tiles = map.get_free_tiles();
                if let Some((x, y)) = free_tiles.choose() {
                    robot.assign_mission(*x as f32, *y as f32);
                }
            }
        }

        map.move_robots();

        let final_positions: Vec<(f32, f32)> = robots.iter().map(|robot| (robot.entity.pos_x, robot.entity.pos_y)).collect();

        assert_ne!(initial_positions, final_positions);
    }
}
