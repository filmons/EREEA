use ereea::utils::utils;
use utils::{generate_rand, TILE_SIZE};

#[test]
fn test_generate_rand() {
    let min = 1;
    let max = 10;

    let result = generate_rand(min, max);

    assert!(result >= min && result <= max);
}

#[test]
fn test_tile_size_constant() {
    assert_eq!(TILE_SIZE, 32.0);
}
