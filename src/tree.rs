use std::{f32::consts::PI, ops::Mul, sync::Arc};

use crate::{
    constants::{STROKE_COLOR, STROKE_WIDTH},
    life::init::AliveCells,
    meta_tiles::{f_init, h_init, p_init, t_init, MetaTile, TileType},
    utils::{intersect, match_two, rot_about},
};
use bevy::math::{Affine2, Vec2};
use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{GeometryBuilder, ShapeBundle, Stroke},
    shapes,
};

fn shape_to_id(shape: TileType) -> usize {
    match shape {
        TileType::H => 0,
        TileType::T => 1,
        TileType::P => 2,
        TileType::F => 3,
        _ => panic!(),
    }
}

#[derive(Debug)]
enum Rule {
    H,
    Four(usize, usize, TileType, usize),
    Six(usize, usize, usize, usize, TileType, usize),
}

static RULES: &[Rule] = &[
    Rule::H,
    Rule::Four(0, 0, TileType::P, 2),
    Rule::Four(1, 0, TileType::H, 2),
    Rule::Four(2, 0, TileType::P, 2),
    Rule::Four(3, 0, TileType::H, 2),
    Rule::Four(4, 4, TileType::P, 2),
    Rule::Four(0, 4, TileType::F, 3),
    Rule::Four(2, 4, TileType::F, 3),
    Rule::Six(4, 1, 3, 2, TileType::F, 0),
    Rule::Four(8, 3, TileType::H, 0),
    Rule::Four(9, 2, TileType::P, 0),
    Rule::Four(10, 2, TileType::H, 0),
    Rule::Four(11, 4, TileType::P, 2),
    Rule::Four(12, 0, TileType::H, 2),
    Rule::Four(13, 0, TileType::F, 3),
    Rule::Four(14, 2, TileType::F, 1),
    Rule::Four(15, 3, TileType::H, 4),
    Rule::Four(8, 2, TileType::F, 1),
    Rule::Four(17, 3, TileType::H, 0),
    Rule::Four(18, 2, TileType::P, 0),
    Rule::Four(19, 2, TileType::H, 2),
    Rule::Four(20, 4, TileType::F, 3),
    Rule::Four(20, 0, TileType::P, 2),
    Rule::Four(22, 0, TileType::H, 2),
    Rule::Four(23, 4, TileType::F, 3),
    Rule::Four(23, 0, TileType::F, 3),
    Rule::Four(16, 0, TileType::P, 2),
    Rule::Six(9, 4, 0, 2, TileType::T, 2),
    Rule::Four(4, 0, TileType::F, 3),
];

pub fn construct_patch(h: MetaTile, t: MetaTile, p: MetaTile, f: MetaTile) -> Vec<Arc<MetaTile>> {
    // let mut root = MetaTile::new(MetaTile::new(
    //     Affine2::IDENTITY,
    //     TileType::Pseudo,
    //     h.data().width,
    //     vec![],
    // ));
    let mut root = Vec::with_capacity(30);
    let shapes = [h, t, p, f];

    for rule in RULES {
        // dbg!(rule);
        match rule {
            Rule::H => {
                let mut h = shapes[0].clone();
                h.transform = Affine2::IDENTITY;
                // dbg!(h.data());
                root.push(Arc::new(h));
            }
            Rule::Four(n_child, n_outline, shape, n_vertex) => {
                let child: Arc<MetaTile> = root[*n_child].clone();
                let poly = child.outline.clone();
                let t = child.transform;

                let p2 = t.transform_point2(poly[(n_outline + 1) % poly.len()]);
                let q2 = t.transform_point2(poly[*n_outline]);

                let mut new_shape = shapes[shape_to_id(*shape)].clone(); //todo
                let new_shape_outline = new_shape.outline.clone();

                let p1 = new_shape_outline[*n_vertex];
                let q1 = new_shape_outline[(*n_vertex + 1) % new_shape_outline.len()];
                // let new_poly = h; //todo
                let new_transform = match_two(p1, q1, p2, q2);

                // *(new_shape).data_mut().transform = new_transform.into();
                // let e = new_shape.data().width;
                // let c = new_shape.abandon();
                // let mut d = MetaTile::new(MetaTile {
                //     transform: new_transform,
                //     shape: *shape,
                //     width: e,
                //     outline: new_shape_outline,
                // });
                // d.append(c);

                new_shape.transform = new_transform;
                new_shape.outline = new_shape_outline;

                // dbg!(d.data());
                root.push(Arc::new(new_shape));
            }
            Rule::Six(n_child_p, n_outline_p, n_child_q, n_outline_q, shape, n_vertex) => {
                // let child_p = root.iter().nth(*n_child_p).unwrap().data();
                // let child_q = root.iter().nth(*n_child_q).unwrap().data();
                let child_p = root[*n_child_p].clone();
                let child_q = root[*n_child_q].clone();

                let p2 = child_q
                    .transform
                    .transform_point2(child_q.outline[*n_outline_q]);
                let q2 = child_p
                    .transform
                    .transform_point2(child_p.outline[*n_outline_p]);

                let mut new_shape = shapes[shape_to_id(*shape)].clone();
                let new_shape_outline = new_shape.outline.clone();
                let p1 = new_shape_outline[*n_vertex];
                let q1 = new_shape_outline[(*n_vertex + 1) % new_shape_outline.len()];
                let new_transform = match_two(p1, q1, p2, q2);
                // *(new_shape).data_mut().transform = new_transform.into();
                // let e = new_shape.data().width;
                // let c = new_shape.abandon();

                // let mut d = MetaTile::new(MetaTile {
                //     transform: new_transform,
                //     shape: *shape,
                //     width: e,
                //     outline: new_shape_outline,
                // });
                // d.append(c);

                new_shape.transform = new_transform;
                new_shape.outline = new_shape_outline;

                // dbg!(d.data());
                root.push(Arc::new(new_shape));
            }
        }
    }
    root
}

pub struct AllFour {
    pub h: MetaTile,
    pub t: MetaTile,
    pub p: MetaTile,
    pub f: MetaTile,
}

fn eval_meta_tile(mt: &MetaTile, i: usize) -> Vec2 {
    let p = mt.outline[i];
    mt.transform.transform_point2(p)
}

pub fn construct_meta_tiles(patch: Vec<Arc<MetaTile>>) -> AllFour {
    let bps1 = eval_meta_tile(&patch[8], 2);
    let bps2 = eval_meta_tile(&patch[21], 2);
    let rbps = rot_about(bps1, (-2.0 * PI) / 3.0).transform_point2(bps2);

    let p72 = eval_meta_tile(&patch[7], 2);
    let p252 = eval_meta_tile(&patch[25], 2);

    let llc = intersect(bps1, rbps, eval_meta_tile(&patch[6], 2), p72);
    let w = eval_meta_tile(&patch[6], 2) - llc;

    let mut new_h_outline = vec![llc, bps1];

    let w = Affine2::from_angle(-PI / 3.0).transform_point2(w);

    // dbg!(bps1);
    // dbg!(llc);

    new_h_outline.push(new_h_outline[1] + w);
    new_h_outline.push(eval_meta_tile(&patch[14], 2));

    let w = Affine2::from_angle(-PI / 3.0).transform_point2(w);

    new_h_outline.push(new_h_outline[3] - w);
    new_h_outline.push(eval_meta_tile(&patch[6], 2));

    let mut new_h = MetaTile {
        children: Vec::new(),
        transform: Affine2::IDENTITY,
        width: patch[0].width * 2,
        shape: TileType::H,
        outline: new_h_outline.clone(),
    };

    for n_child in [0, 9, 16, 27, 26, 6, 1, 8, 10, 15] {
        new_h.push_rc(patch[n_child].clone());
    }

    let new_p_outline = vec![p72, (p72 + (bps1 - llc)), bps1, llc];
    let mut new_p = MetaTile {
        children: Vec::new(),
        transform: Affine2::IDENTITY,
        width: patch[0].width * 2,
        shape: TileType::P,
        outline: new_p_outline,
    };

    for n_child in [7, 2, 3, 4, 28] {
        new_p.push_rc(patch[n_child].clone());
    }

    let new_f_outline = vec![
        bps2,
        eval_meta_tile(&patch[24], 2),
        eval_meta_tile(&patch[25], 0),
        p252,
        (p252 + (llc - bps1)),
    ];

    let mut new_f = MetaTile {
        children: Vec::new(),
        transform: Affine2::IDENTITY,
        width: patch[0].width * 2,
        shape: TileType::F,
        outline: new_f_outline,
    };

    for n_child in [21, 20, 22, 23, 24, 25] {
        new_f.push_rc(patch[n_child].clone());
    }

    let aaa = new_h_outline[2];
    let bbb = new_h_outline[1] + (new_h_outline[4] - new_h_outline[5]);
    let ccc = rot_about(bbb, -PI / 3.0).transform_point2(aaa);
    let new_t_outline = vec![bbb, ccc, aaa];

    let mut new_t = MetaTile {
        children: Vec::new(),
        transform: Affine2::IDENTITY,
        width: patch[0].width * 2,
        shape: TileType::T,
        outline: new_t_outline,
    };

    new_t.push_rc(patch[11].clone());

    AllFour {
        h: new_h,
        t: new_t,
        p: new_p,
        f: new_f,
    }
}

pub struct TreePlugin;

#[derive(Resource)]
pub struct MetaTileTree(pub MetaTile);

#[derive(Resource, Debug)]
pub struct TreeConfig {
    pub levels: usize,
    pub meta_tile: MetaTileType,
}
impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TreeConfig {
            levels: 5,
            meta_tile: MetaTileType::H,
        })
        .add_system(background_polygons);
    }
}

fn construct_tree(levels: usize) -> AllFour {
    let mut a = AllFour {
        h: h_init(),
        t: t_init(),
        p: p_init(),
        f: f_init(),
    };

    for _ in 0..levels {
        let patch = construct_patch(a.h, a.t, a.p, a.f);
        a = construct_meta_tiles(patch);
    }

    a
}

#[derive(PartialEq, Eq, Debug)]
pub enum MetaTileType {
    H,
    T,
    P,
    F,
}

#[derive(Component)]
pub struct DeadCells;

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
        let all = construct_tree(tree_config.levels);
        let mtt = match tree_config.meta_tile {
            MetaTileType::H => all.h,
            MetaTileType::T => all.t,
            MetaTileType::P => all.p,
            MetaTileType::F => all.f,
        };

        let cap = 200_000;

        let mut polys = HatPolys {
            h: Vec::with_capacity(cap),
            h1: Vec::with_capacity(cap),
            t: Vec::with_capacity(cap),
            f: Vec::with_capacity(cap),
            p: Vec::with_capacity(cap),
            meta: vec![Vec::new(); tree_config.levels + 2],
        };

        make_polygons(
            &mut polys,
            Affine2::from_scale(Vec2 { x: 5.0, y: 5.0 }),
            &mtt,
        );

        commands.insert_resource(MetaTileTree(mtt));
        // std::process::exit(0);

        for (i, shape) in [
            TileType::H1Hat,
            TileType::HHat,
            TileType::THat,
            TileType::FHat,
            TileType::PHat,
        ]
        .iter()
        .enumerate()
        {
            let polys = match shape {
                TileType::H1Hat => &polys.h1,
                TileType::HHat => &polys.h,
                TileType::THat => &polys.t,
                TileType::PHat => &polys.p,
                TileType::FHat => &polys.f,
                _ => panic!(),
            };
            for chunk in polys.chunks(500_000) {
                let mut g = GeometryBuilder::new();
                for tile in chunk {
                    g = g.add(tile);
                }

                // std::process::exit(0);

                commands.spawn((
                    ShapeBundle {
                        path: g.build(),
                        transform: Transform::from_xyz(0.0, 0.0, i as f32 * 0.01),
                        ..default()
                    },
                    // Fill::color(shape_to_fill_color(*shape)),
                    Stroke::new(STROKE_COLOR, STROKE_WIDTH),
                    DeadCells,
                ));
            }
        }

        if false {
            for (i, outlines) in polys.meta.iter().enumerate() {
                let mut g = GeometryBuilder::new();
                for outline in outlines {
                    g = g.add(outline);
                }

                // std::process::exit(0);

                commands.spawn((
                    ShapeBundle {
                        path: g.build(),
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..default()
                    },
                    Stroke::new(STROKE_COLOR, STROKE_WIDTH * 2.0_f32.powi(i as i32)),
                ));
            }
        }
    }
}
type HatPoints = Vec<shapes::Polygon>;
struct HatPolys {
    h: HatPoints,
    h1: HatPoints,
    t: HatPoints,
    f: HatPoints,
    p: HatPoints,
    meta: Vec<Vec<shapes::Polygon>>,
}
fn make_polygons(polys: &mut HatPolys, t: Affine2, tree: &MetaTile) {
    for child in &tree.children {
        make_polygons(polys, t.mul(tree.transform), child);
    }

    let tt = t.mul(tree.transform);
    let points = tree
        .outline
        .iter()
        .map(|p| tt.transform_point2(*p))
        // .map(|p| (Affine2::from_scale(Vec2::new(100.0, 100.0))).transform_point2(*p))
        .collect();
    let poly = shapes::Polygon {
        points,
        closed: true,
    };
    match tree.shape {
        TileType::H1Hat => polys.h1.push(poly),
        TileType::HHat => polys.h.push(poly),
        TileType::THat => polys.t.push(poly),
        TileType::PHat => polys.p.push(poly),
        TileType::FHat => polys.f.push(poly),
        _ => {
            let level = (tree.width as f32).log2() as usize;
            polys.meta[level].push(poly);
        }
    }
}
