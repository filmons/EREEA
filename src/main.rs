use map::map::Map;
use utils::utils::generate_rand;
mod utils;
mod map;

fn main() {
    //Seed aléatoire, maiss maps reproductibles grâce au numéro de la seed comme dans Minecraft.
    let map_seed = generate_rand(1, 2);
    let map_width: usize = 32;
    let map_height: usize = 32;
    let map = Map::new(map_seed, map_width, map_height, 5, 4, 2, 3);

    // Afficher la carte
    map::map::Map::display_map(&map);
}
