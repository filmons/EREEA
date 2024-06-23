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
    let map = MockMap::new().await;
    let mut station = Station::new().await;
    assert_eq!(station.stage, StageType::Initialiation);
    station.simulation_start().await;
    assert_eq!(station.stage, StageType::Analysing);

}
