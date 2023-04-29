type TranslateX = f32;
type TranslateY = f32;
type RotateByDegrees = f32;
type Mirror = bool;
type HatArgs = (TranslateX, TranslateY, RotateByDegrees, Mirror);

const H_TILE: &[HatArgs] = &[
    (0.0, 0.0, 120.0, true),
    (0.0, 2.0 * SQ3, 300.0, false),
    (-3.0, 1.0 * SQ3, 60.0, false),
    (3.0, 1.0 * SQ3, 60.0, false),
];
static T_TILE: &[HatArgs] = &[(0.0, 0.0, 180.0, false)];
static P_TILE: &[HatArgs] = &[(0.0, 0.0, 180.0, false), (-3.0, 1.0 * SQ3, 120.0, false)];
static F_TILE: &[HatArgs] = &[(0.0, 0.0, 180.0, false), (-3.0, 1.0 * SQ3, 120.0, false)];

fn hat(
    tile_type: TileType,
    translate_x: f32,
    translate_y: f32,
    rotate: f32,
    mirror: bool,
) -> (ShapeBundle, Fill, Stroke) {
    use TileType::*;
    let color = match (mirror, tile_type) {
        (true, H) => H_MIRROR_COLOR,
        (false, H) => H_COLOR,
        (_, T) => T_COLOR,
        (_, P) => P_COLOR,
        (_, F) => F_COLOR,
        (true, H1Hat) => todo!(),
        (true, HHat) => todo!(),
        (true, THat) => todo!(),
        (true, PHat) => todo!(),
        (true, FHat) => todo!(),
        (false, H1Hat) => todo!(),
        (false, HHat) => todo!(),
        (false, THat) => todo!(),
        (false, PHat) => todo!(),
        (false, FHat) => todo!(),
    };
    let mirror = if mirror { -1.0 } else { 1.0 };
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
    .map(|(x, y)| Vec2::new(x * SCALE * mirror, y * SCALE))
    .collect();

    let a = shapes::Polygon {
        points,
        closed: true,
    };

    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&a),
            transform: Transform::from_translation(Vec3::new(
                translate_x * SCALE,
                translate_y * SCALE,
                0.0,
            ))
            .with_rotation(Quat::from_rotation_z((rotate).to_radians())),
            ..default()
        },
        Fill::color(color),
        Stroke::new(Color::BLACK, SCALE / 3.0),
        // Transform::new(),
    )
}

fn spawn_supertile(
    commands: &mut Commands,
    tile_type: TileType,
    translate_x: f32,
    translate_y: f32,
    rotate: f32,
) -> Entity {
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    translate_x * (SCALE * 10.0), // - 400.0,
                    translate_y * (SCALE * 10.0), // - 475.0,
                    0.0,
                ))
                .with_rotation(Quat::from_rotation_z((rotate).to_radians())),
                ..default()
            },
            tile_type.clone(),
        ))
        .with_children(|parent| {
            let tile_args = match tile_type {
                TileType::H => H_TILE,
                TileType::T => T_TILE,
                TileType::P => P_TILE,
                TileType::F => F_TILE,
                TileType::H1Hat => todo!(),
                TileType::HHat => todo!(),
                TileType::THat => todo!(),
                TileType::PHat => todo!(),
                TileType::FHat => todo!(),
            };
            for hat_args in tile_args {
                parent.spawn(std::ops::Fn::call(
                    &hat,
                    (tile_type, hat_args.0, hat_args.1, hat_args.2, hat_args.3),
                    // (tile_type, ..hat_args),
                ));
            }
        })
        .id()
}
// spawn_supertile(&mut commands, TileType::H, -4.0, 0.0, 0.0);
// spawn_supertile(&mut commands, TileType::T, -2.0, 0.0, 0.0);
// spawn_supertile(&mut commands, TileType::P, 0.0, 0.0, 0.0);
// spawn_supertile(&mut commands, TileType::F, 2.0, 0.0, 0.0);

// for i in 0..10 {
//     for j in 0..10 {
//         let i = i as f32;
//         let j = j as f32;

//         spawn_supertile(
//             &mut commands,
//             Supertile::HTile,
//             j * 1.0,
//             i * 1.0,
//             (i + j) * 5.0,
//         );
//     }
// }
