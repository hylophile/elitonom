pub mod hat;
pub mod hat_meta_tiles;
pub mod spectre;

use crate::{constants::LEVELS, life::init::AliveCells};
use bevy::prelude::*;

use self::{
    hat::{hat_background_polygons, HatMetaTileType},
    spectre::spectre_background_polygons,
};

#[derive(Component)]
pub struct DeadCells;

#[derive(Resource, Debug)]
pub struct TreeConfig {
    pub levels: usize,
    pub meta_tile: HatMetaTileType,
    pub spectre: bool,
}

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TreeConfig {
            levels: LEVELS,
            meta_tile: HatMetaTileType::H,
            spectre: true,
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

        if tree_config.spectre {
            spectre_background_polygons(commands, tree_config.levels)
        } else {
            hat_background_polygons(commands, tree_config)
        }
    }
}
