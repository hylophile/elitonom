use bevy::{
    math::Affine2,
    prelude::*,
    sprite::MaterialMesh2dBundle,
    // sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::PanCamPlugin;
use bevy_prototype_lyon::prelude::*;
use constants::BG_COLOR;
use rand::prelude::*;
use std::{f32::consts::PI, ops::Mul, sync::Arc};

mod constants;
mod life;
mod meta_tiles;
mod tree;
mod utils;
use tree::{construct_meta_tiles, construct_patch, AllFour, TreePlugin};

fn setup(
    mut commands: Commands,
    // ass: Res<AssetServer>,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 1.0,
                ..default()
            },
            ..default()
        })
        .insert(bevy_pancam::PanCam::default());
}

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(ShapePlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(BG_COLOR))
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        .add_plugin(TreePlugin)
        .run();
}
