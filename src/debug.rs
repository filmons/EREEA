use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    //Log de la transform de chaque et l'ID de chaque entité ayant une transform.
    for (entity, transform) in query.iter() {
        info!(
            "Entity {:?} est à la transform {:?},",
            entity, transform.translation
        );
    }
}
