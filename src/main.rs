use bevy::prelude::*;

use bevy_pancam::PanCamPlugin;
use bevy_prototype_lyon::prelude::*;
use constants::BG_COLOR;

mod constants;
mod life;
mod meta_tiles;
mod tree;
mod ui;
mod utils;
use life::LifePlugin;
use tree::TreePlugin;
use ui::UIPlugin;

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
        // .insert_resource(Msaa::Sample4)
        // .insert_resource(ClearColor(Color::rgb(0.8,0.8,0.8)))
        // .insert_resource(ClearColor(Color::rgb(0.8,0.8,0.8)))
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(ShapePlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(BG_COLOR))
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        .add_plugin(TreePlugin)
        .add_plugin(LifePlugin)
        .add_plugin(UIPlugin)
        .run();
}
