#[derive(Clone)]
pub enum TileType {
    Obstacle,
    Ground1,
    Ground2,
    Ground3,
    Ground4,
    Void,
}

#[derive(Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub pos_x: usize,
    pub pos_y: usize,
    pub symbol: char,
}

impl Tile {
    pub fn new(tile_type: TileType, x: usize, y: usize) -> Self {
        let symbol = Self::get_symbol(&tile_type);
        Self {
            tile_type,
            pos_x: x,
            pos_y: y,
            symbol,
        }
    }

    pub fn get_symbol(tile_type: &TileType) -> char {
        match tile_type {
            TileType::Obstacle => '#',
            TileType::Ground1 => '.',
            TileType::Ground2 => ',',
            TileType::Ground3 => ';',
            TileType::Ground4 => ':',
            TileType::Void => ' ',
        }
    }

    //Recycler en png si utilisation de Bevy
    /*pub fn name(&self) -> &'static str {
        match *self {
            Tile::Obstacle => "Obstacle",
            Tile::Ground1 => "Ground 1",
            Tile::Ground2 => "Ground 2",
            Tile::Ground3 => "Ground 3",
            Tile::Ground4 => "Ground 4",
            Tile::Void => "Espace",
        }
    }*/
}
