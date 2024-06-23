use std::{thread::sleep, time::Duration};
use ereea::station::station::Station;
use macroquad::prelude::*;

#[macroquad::main("EREEA")]
async fn main() {
    let map_size_factor = 2; //Multiplie la taille de la map 32x32 par 2 soit 64x64 cases

    let min_nb_per_resources = 1;

    let max_nb_per_resources = 10; //Le nombre maximal qu'on pourrait afficher par ressources.

    let mut station = Station::new(map_size_factor, min_nb_per_resources, max_nb_per_resources).await;

    station.play_galactic_music().await;

    loop {
        clear_background(BLACK);

        // Obtenir la position de la souris
        let mouse_x = mouse_position().0;
        let mouse_y = mouse_position().1;

        station.draw(mouse_x, mouse_y, 0.4);

        station.simulation_start().await;

        if station.check_if_finished() {
            println!("Mission terminée !\n");
            sleep(Duration::from_secs(1));

            station.restart_new_mission().await;
            // break;
        }

        let mission_text = format!("Mission [ANNÉE-Y{}]", station.epoch);
        draw_text(&mission_text, 32.0, 32.0, 32.0, WHITE);

        next_frame().await;
    }
}
