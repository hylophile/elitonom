//! Shows how to render simple primitive shapes with a single color.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let sq3: f32 = 3.0f32.sqrt();

    commands.spawn(hat(0.0, 0.0, 0.0, true));
    commands.spawn(hat(3.0, sq3, 300.0, false));
    commands.spawn(hat(0.0, -2.0 * sq3, 300.0, false));
    commands.spawn(hat(3.0, -1.0 * sq3, 180.0, false));
}

fn hat(
    translate_x: f32,
    translate_y: f32,
    rotate: f32,
    mirror: bool,
) -> (ShapeBundle, Fill, Stroke) {
    let color = if mirror {
        Color::SEA_GREEN
    } else {
        Color::WHITE
    };
    let mirror = if mirror { -1.0 } else { 1.0 };
    let scale = 10.0;
    let sq3: f32 = 3.0f32.sqrt();
    let sq075 = sq3 / 2.0;
    let points = vec![
        (0.0, 0.0),
        (0.0, -sq3),
        (-1.0, -sq3),
        (-1.5, -sq3 - sq075),
        (-3.0, -sq3),
        (-3.0, 0.0),
        (-4.0, 0.0),
        (-4.5, sq075),
        (-3.0, sq3),
        (-1.5, sq075),
        (-1.0, sq3),
        (1.0, sq3),
        (1.5, sq075),
    ]
    .iter()
    .map(|(x, y)| Vec2::new(x * scale * mirror, y * scale))
    .collect();

    let a = shapes::Polygon {
        points,
        closed: true,
    };

    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&a),
            transform: Transform::from_translation(Vec3::new(
                translate_x * scale,
                translate_y * scale,
                0.0,
            ))
            .with_rotation(Quat::from_rotation_z((rotate).to_radians())),
            ..default()
        },
        Fill::color(color),
        Stroke::new(Color::BLACK, 3.0),
        // Transform::new(),
    )
}
