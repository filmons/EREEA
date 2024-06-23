use std::cmp::max;

use ereea::station::station::{StageType, Station};

use ereea::map::map;

struct MockMap;
struct MockSounds;

impl MockMap {
    async fn new() -> Self {
        MockMap
    }
}

impl MockSounds {
    async fn load() -> Self {
        MockSounds
    }
}

#[tokio::test]
async fn test_station_simulation() {
    let map_size_factor = 2;
    let min_nb_per_resources = 1;
    let max_nb_per_resources = 5;

    let map = MockMap::new().await;
    let mut station = Station::new(map_size_factor, min_nb_per_resources, max_nb_per_resources).await;
    assert_eq!(station.stage, StageType::Initialiation);
    station.simulation_start().await;
    assert_eq!(station.stage, StageType::Analysing);

}
