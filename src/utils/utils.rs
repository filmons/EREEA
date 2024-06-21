use rand::Rng;

pub fn generate_rand(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

pub const TILE_SIZE: f32 = 32.0; //Notre jeu se base sur un tileset de 32x32 cases
