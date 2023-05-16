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

// fn draw_tree(
//     commands: &mut Commands,
//     // ass: &Res<AssetServer>,
//     t: Affine2,
//     node: MetaTile,
//     mut z: f32,
// ) -> f32 {
//     for child in node.iter() {
//         let tc = node.data().transform;
//         z = draw_tree(commands, t.mul(tc), child.deep_clone(), z);
//         z += 0.00001;

//         // *hat.data_mut().transform = *t.mul(ht);
//         // if i == 0 {
//         //     break;
//         // }
//     }

//     let mut nn = node.data().clone();
//     nn.transform = t.mul(nn.transform);
//     //|| rand::random::<f32>() < 0.1 {
//     // if !is_hat(node.data().shape) {
//     commands.spawn(polygon_entity(nn, z));
//     // }
//     return z;
// }

fn rotate_colors_playground(
    mut query: Query<(&mut Fill, &mut TileType, &mut Transform)>,
    time: Res<Time>,
) {
    // let mut c = query.;
    // *c = Fill::color(Color::rgba(1.0, 1.0, 1.0, 0.1));
    for (mut fill, shape, mut transform) in query.iter_mut() {
        // if fill.color.r() > 0.9 || fill.color.g() > 0.9 || fill.color.b() > 0.9 {
        //     fill.color = Color::rgba(
        //         fill.color.r() - 0.01,
        //         fill.color.g() - 0.01,
        //         fill.color.b() - 0.01,
        //         1.0,
        //     );
        // } else if fill.color.r() < 0.1 || fill.color.g() < 0.1 || fill.color.b() < 0.1 {
        //     fill.color = fill.color + Color::rgba(0.01, 0.01, 0.01, 0.0);
        // }
        // let a = (((time.elapsed_seconds() * 100.0) as usize) % 100) as f32;
        // let a = a / 100.0;
        // transform.rotate_z(0.125_f32.to_radians());
        let tt = transform.translation / 2.0;
        transform.rotate_around(tt * -1.0, Quat::from_rotation_z((1.0_f32).to_radians()));
        let a = time.elapsed_seconds();
        let b = shape_to_fill_color(*shape);
        let c = Color::rgba(
            b.r() * (a.sin() + 1.0),
            b.g() * (a.cos() + 1.0),
            b.b() * ((a + PI).cos() + 1.0),
            1.0 * b.a(),
        );
        // let a = Color::rgba(
        //     (b.r() * a.sin()).abs(),
        //     (b.g() * a.sin()).abs(),
        //     (b.b() * a.sin()).abs(),
        //     1.0 * b.a(),
        // );
        // fill.color = Color::rgba(a.sin(), a.cos(), a.tan(), 1.0)
        // fill.color = Color::rgba(a, a, a, 1.0);
        fill.color = c;
    }
}
type TileEntity = (ShapeBundle, Fill, Stroke, TileType);
// type TileEntity = (SpriteBundle, TileType);

fn polygon_entity(tile: MetaTile, z: f32) -> TileEntity {
    let polygon = shapes::Polygon {
        // points: Vec::from(HAT_OUTLINE),
        points: tile
            .outline
            .iter()
            .map(|p| tile.transform.transform_point2(*p))
            .collect(),
        closed: true,
    };

    // return (
    //     SpriteBundle {
    //         texture: asset_server.load("hat-monotile.png"), // nope, performance is worse
    //         ..default()
    //     },
    //     tile.shape,
    // );
    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&polygon),
            // transform: Transform::from_matrix(mat4_from_affine2(tile.transform, z)),
            ..default()
        },
        Fill::color(shape_to_fill_color(tile.shape)),
        Stroke::new(
            Color::rgba(0.0, 0.0, 0.0, 1.0),
            // 0.10 * (tile.width as f32), //.sqrt(),
            // 0.15,
            0.10 * (tile.width as f32).sqrt(),
        ),
        tile.shape,
    )
}

fn mat4_from_affine2(affine2: Affine2, z: f32) -> Mat4 {
    Mat4::from_cols(
        Vec4::new(affine2.matrix2.x_axis.x, affine2.matrix2.x_axis.y, 0.0, 0.0),
        Vec4::new(affine2.matrix2.y_axis.x, affine2.matrix2.y_axis.y, 0.0, 0.0),
        Vec4::Z,
        Vec4::new(affine2.translation.x, affine2.translation.y, z, 1.0),
    )
}

// .add_system(rotate_colors_playground.in_schedule(CoreSchedule::FixedUpdate))
// .add_system(rotate_colors_playground)
// .insert_resource(FixedTime::new_from_secs(1.0 / 30.0))

// _ = draw_tree(&mut commands, Affine2::IDENTITY, which_meta_tile, 0.0);

fn is_hat(s: TileType) -> bool {
    match s {
        TileType::H => false,
        TileType::T => false,
        TileType::P => false,
        TileType::F => false,
        TileType::H1Hat => true,
        TileType::HHat => true,
        TileType::THat => true,
        TileType::PHat => true,
        TileType::FHat => true,
    }
}
