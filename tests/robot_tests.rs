#[cfg(test)]
mod tests {
    use super::*;
    use async_std::task::block_on;
    use ereea::{resource::resource::{Resource, ResourceType}, robot::robot::{Robot, RobotType}};
    use tokio::sync::futures;

    #[test]
    fn test_robot_creation() {
        let robot_future = Robot::new(RobotType::Analyse, 100.0, 200.0);
        
        let mut robot = block_on(robot_future);
        
        assert_eq!(robot.robot_type, RobotType::Analyse);
        assert_eq!(robot.is_busy, false);
        assert_eq!(robot.mission_target, (100.0, 200.0));
        assert!(robot.resources.is_empty());
    }

    #[test]
    fn test_assign_mission() {
        let mut robot_future = Robot::new(RobotType::Forage, 150.0, 250.0);
        
        let mut robot = block_on(robot_future);
        
        robot.assign_mission(200.0, 300.0);
        
        assert_eq!(robot.is_busy, true);
        assert_eq!(robot.mission_target, (200.0, 300.0));
    }

    #[test]
    fn test_move_to() {
        let mut robot_future = Robot::new(RobotType::Explorer, 300.0, 400.0);
        
        let mut robot = block_on(robot_future);
        
        let obstacles = [(10, 20), (30, 40)];

        robot.move_to(400.0, 500.0,&obstacles);
        
        assert_eq!(robot.entity.position(), (400.0, 500.0));
    }

    #[test]
    fn test_robot_specialisation() {
        let mut robot = block_on(Robot::new(RobotType::Imagerie, 100.0, 100.0));

        let resource = block_on(Resource::new(ResourceType::Minerai, 200.0, 200.0));

        block_on(async {
            robot.specialisation("./assets/images/robot_0.png", resource).await;

            assert_eq!(robot.is_busy, false);
            assert_eq!(robot.resources.len(), 1);
        });
    }
}
