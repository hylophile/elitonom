use bevy::prelude::*;

use bevy_pancam::PanCamPlugin;
use bevy_prototype_lyon::prelude::*;
use constants::BG_COLOR;

mod constants;
mod life;
mod tree;
mod ui;
mod utils;
use life::LifePlugin;
use tree::TreePlugin;
use ui::UIPlugin;

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.1,
                ..default()
            },
            ..default()
        })
        .insert(bevy_pancam::PanCam {
            grab_buttons: vec![MouseButton::Right],
            ..default()
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Aperiodic Game of Life".to_string(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(PanCamPlugin::default())
        .add_plugin(ShapePlugin)
        .insert_resource(ClearColor(BG_COLOR))
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        .add_plugin(TreePlugin)
        .add_plugin(LifePlugin)
        .add_plugin(UIPlugin)
        .run();
}
