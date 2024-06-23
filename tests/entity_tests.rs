use macroquad::prelude::*;
use ereea::entity::entity::Entity;
use ereea::utils::utils::TILE_SIZE;

#[macroquad::test]
async fn test_create_entity() {
    let pos_x = 10.0;
    let pos_y = 20.0;
    let speed_factor = 1.0;
    let texture_path = "./assets/images/robot_0.png";

    let entity = Entity::new(pos_x, pos_y, speed_factor, texture_path).await;

    assert_eq!(entity.pos_x, pos_x);
    assert_eq!(entity.pos_y, pos_y);
    assert_eq!(entity.speed, speed_factor * TILE_SIZE);
}

#[macroquad::test]
async fn test_move_entity_no_obstacles() {
    let mut entity = Entity {
        pos_x: 0.0,
        pos_y: 0.0,
        speed: 1.0 * TILE_SIZE,
        texture: dummy_texture().await,
        offset_x: 0.0,
        offset_y: 0.0,
    };

    let target_x = 5.0;
    let target_y = 5.0;
    let obstacles = vec![]; // Aucun obstacle pour ce test

    println!("Initial position: ({}, {})", entity.pos_x, entity.pos_y);

    entity.move_to(target_x, target_y, &obstacles);

    println!("Final position: ({}, {})", entity.pos_x, entity.pos_y);

    assert!(
        entity.pos_x >= 0.0 && entity.pos_x <= target_x,
        "pos_x: {}, target_x: {}",
        entity.pos_x,
        target_x
    );
    assert!(
        entity.pos_y >= 0.0 && entity.pos_y <= target_y,
        "pos_y: {}, target_y: {}",
        entity.pos_y,
        target_y
    );
}


#[macroquad::test]
async fn test_move_entity_with_obstacles() {
    let mut entity = Entity {
        pos_x: 0.0,
        pos_y: 0.0,
        speed: 1.0 * TILE_SIZE,
        texture: dummy_texture().await,
        offset_x: 0.0,
        offset_y: 0.0,
    };

    let target_x = 5.0;
    let target_y = 5.0;
    let obstacles = vec![(5, 5)];

    entity.move_to(target_x, target_y, &obstacles);

    assert!(!(entity.pos_x == 5.0 && entity.pos_y == 5.0));
}

#[macroquad::test]
async fn test_change_texture() {
    let mut entity = Entity {
        pos_x: 0.0,
        pos_y: 0.0,
        speed: 1.0 * TILE_SIZE,
        texture: dummy_texture().await,
        offset_x: 0.0,
        offset_y: 0.0,
    };

    let new_texture_path = "./assets/images/resource_place.png";
    entity.set_texture(new_texture_path).await;

    let new_texture = load_texture(new_texture_path).await.unwrap();
    assert_eq!(entity.texture.width(), new_texture.width());
    assert_eq!(entity.texture.height(), new_texture.height());
}

// Fonction utilitaire pour créer une texture factice pour les tests
async fn dummy_texture() -> Texture2D {
    load_texture("./assets/images/tileset.png").await.unwrap()
}
