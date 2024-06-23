use ereea::resource::resource::{Resource, ResourceType};
use ereea::entity::entity::Entity;
use macroquad::prelude::*;


#[macroquad::test]
async fn test_resource_creation() {
    let resource = Resource::new(ResourceType::Energie, 100.0, 200.0).await;

    assert_eq!(resource.resource_type, ResourceType::Energie);
    assert_eq!(resource.is_consumed, false);

}

#[macroquad::test]
async fn test_set_consumed() {
    let mut resource = Resource::new(ResourceType::Minerai, 150.0, 250.0).await;

    assert_eq!(resource.resource_type, ResourceType::Minerai);
    assert_eq!(resource.is_consumed, false);

    resource.set_consumed();

    assert_eq!(resource.is_consumed, true);
}