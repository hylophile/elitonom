use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use kiddo::distance::squared_euclidean;

use crate::life::{init::AliveCells, step::hatsmesh};

use super::init::{Affines, LifeState, MetaTileKdTree}; //, render::camera::RenderTarget};

pub fn draw(
    mut commands: Commands,
    // need to get window dimensions
    windows: Query<&Window>,

    (mut meshes, mut materials, mut life_state): (
        ResMut<Assets<Mesh>>,
        ResMut<Assets<ColorMaterial>>,
        Option<ResMut<LifeState>>,
    ),
    (buttons, affines, kdtree): (
        Res<Input<MouseButton>>,
        Option<Res<Affines>>,
        Option<Res<MetaTileKdTree>>,
    ),
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    if buttons.pressed(MouseButton::Left) {
        if let (Some(kdtree), Some(affines), Some(mut life_state)) = (kdtree, affines, life_state) {
            // Right Button is being held down
            let (camera, camera_transform) = camera_q.single();
            for window in &windows {
                if let Some(world_position) = window
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    let ne = kdtree
                        .0
                        .nearest_n(world_position.as_ref(), 1, &squared_euclidean)
                        .into_iter()
                        .map(|ne| {
                            life_state.new[ne.item as usize] = true;
                            ne.item as usize
                        })
                        .collect();
                    let hatss = hatsmesh(&ne, &affines.0);
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(hatss).into(),
                            material: materials
                                .add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.99))),
                            // material: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.1))),
                            ..default()
                        },
                        AliveCells,
                    ));
                    // eprintln!("World coords: {}/{}", world_position.x, world_position.y);
                }
            }
        }
    }
}
