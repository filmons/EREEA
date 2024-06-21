use ereea::station::station::Station;
use macroquad::prelude::*;

#[macroquad::main("EREEA")]
async fn main() {
    let mut station = Station::new().await;
    // station.play_galactic_music().await;
    station.draw_terminal();

    loop {
        clear_background(BLACK);

        // Obtenir la position de la souris
        let mouse_x = mouse_position().0;
        let mouse_y = mouse_position().1;

        station.draw(mouse_x, mouse_y, 0.4);

        station.simulation_start().await;

        station.check_simulation_stage();

        next_frame().await;
    }
}
