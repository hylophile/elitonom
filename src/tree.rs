pub mod hat;
pub mod hat_meta_tiles;

use crate::{constants::LEVELS, life::init::AliveCells};
use bevy::prelude::*;

use self::hat::{hat_background_polygons, MetaTileType};

#[derive(Component)]
pub struct DeadCells;

#[derive(Resource, Debug)]
pub struct TreeConfig {
    pub levels: usize,
    pub meta_tile: MetaTileType,
    pub spectre: bool,
}

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TreeConfig {
            levels: LEVELS,
            meta_tile: MetaTileType::H,
            spectre: false,
        })
        .add_system(background_polygons);
    }
}

fn background_polygons(
    mut commands: Commands,
    tree_config: Res<TreeConfig>,
    dead_cells: Query<Entity, With<DeadCells>>,
    alive_cells: Query<Entity, With<AliveCells>>,
) {
    if tree_config.is_added() || tree_config.is_changed() {
        for c in dead_cells.iter() {
            commands.entity(c).despawn();
        }
        for c in alive_cells.iter() {
            commands.entity(c).despawn();
        }

        hat_background_polygons(commands, tree_config)
    }
}
