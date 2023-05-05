const LEVELS: usize = 6;
const H_COLOR: Color = Color::WHITE;
const H_MIRROR_COLOR: Color = Color::SEA_GREEN;
const T_COLOR: Color = Color::TEAL;

const P_COLOR: Color = Color::CRIMSON;
const F_COLOR: Color = Color::GOLD;
// static STROKE_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);
static STROKE_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 1.0);
static BG_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

const STROKE_WIDTH: f32 = 0.5;

const SQ3: f32 = 1.732_050_8; // sqrt(3)
const HR3: f32 = 0.866_025_4; // sqrt(3) / 2

use bevy::{
    math::Affine2,
    prelude::*,
    // sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::PanCamPlugin;
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;
use std::{f32::consts::PI, ops::Mul, rc::Rc};

mod utils;
use utils::*;

const HAT_OUTLINE: &[Vec2] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(-1.5, -0.5 * SQ3),
    Vec2::new(-1.0, -SQ3),
    Vec2::new(1.0, -SQ3),
    Vec2::new(1.5, -0.5 * SQ3),
    Vec2::new(3.0, -SQ3),
    Vec2::new(4.5, -0.5 * SQ3),
    Vec2::new(4.0, 0.0),
    Vec2::new(3.0, 0.0),
    Vec2::new(3.0, SQ3),
    Vec2::new(1.5, 1.5 * SQ3),
    Vec2::new(1.0, SQ3),
    Vec2::new(0.0, SQ3),
];

const H_OUTLINE: &[Vec2] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(4.0, 0.0),
    Vec2::new(4.5, HR3),
    Vec2::new(2.5, 5.0 * HR3),
    Vec2::new(1.5, 5.0 * HR3),
    Vec2::new(-0.5, HR3),
];

fn h_init() -> MetaTile {
    let mut h = MetaTile {
        transform: Affine2::IDENTITY,
        shape: TileType::H,
        width: 2,
        outline: H_OUTLINE.to_vec(),
        children: Vec::new(),
    };

    h.push_rc(Rc::new(MetaTile {
        children: Vec::new(),
        transform: match_two(HAT_OUTLINE[5], HAT_OUTLINE[7], H_OUTLINE[5], H_OUTLINE[0]),
        shape: TileType::HHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));

    h.push_rc(Rc::new(MetaTile {
        children: Vec::new(),
        transform: match_two(HAT_OUTLINE[9], HAT_OUTLINE[11], H_OUTLINE[1], H_OUTLINE[2]),
        shape: TileType::HHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));

    h.push_rc(Rc::new(MetaTile {
        children: Vec::new(),
        transform: match_two(HAT_OUTLINE[5], HAT_OUTLINE[7], H_OUTLINE[3], H_OUTLINE[4]),
        shape: TileType::HHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));

    h.push_rc(Rc::new(MetaTile {
        children: Vec::new(),
        transform: Affine2::from_cols_array_2d(&[
            [-0.25, 0.5 * HR3],
            [0.5 * HR3, 0.25],
            [2.5, HR3],
        ]),
        shape: TileType::H1Hat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));

    h
}

const T_OUTLINE: &[Vec2] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(3.0, 0.0),
    Vec2::new(1.5, 3.0 * HR3),
];

fn t_init() -> MetaTile {
    let mut t = MetaTile {
        children: Vec::new(),
        transform: Affine2::IDENTITY,
        shape: TileType::T,
        width: 2,
        outline: T_OUTLINE.to_vec(),
    };

    t.push_rc(Rc::new(MetaTile {
        children: Vec::new(),
        transform: Affine2::from_cols_array_2d(&[[0.5, 0.0], [0.0, 0.5], [0.5, HR3]]),
        shape: TileType::THat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));

    t
}

const P_OUTLINE: &[Vec2] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(4.0, 0.0),
    Vec2::new(3.0, 2.0 * HR3),
    Vec2::new(-1.0, 2.0 * HR3),
];

fn p_init() -> MetaTile {
    let mut p = MetaTile {
        children: Vec::new(),
        transform: Affine2::IDENTITY,
        shape: TileType::P,
        width: 2,
        outline: P_OUTLINE.to_vec(),
    };
    p.push(MetaTile {
        children: Vec::new(),
        transform: Affine2::from_cols_array_2d(&[[0.5, 0.0], [0.0, 0.5], [1.5, HR3]]),
        shape: TileType::PHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    });
    p.push(MetaTile {
        children: Vec::new(),
        transform: Affine2::from_cols_array_2d(&[
            [0.25, -0.5 * HR3],
            [0.5 * HR3, 0.25],
            [0.0, SQ3],
        ]),
        shape: TileType::PHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    });
    p
}

const F_OUTLINE: &[Vec2] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(3.0, 0.0),
    Vec2::new(3.5, HR3),
    Vec2::new(3.0, 2.0 * HR3),
    Vec2::new(-1.0, 2.0 * HR3),
];

fn f_init() -> MetaTile {
    let mut f = MetaTile {
        children: Vec::new(),
        transform: Affine2::IDENTITY,
        shape: TileType::F,
        width: 2,
        outline: F_OUTLINE.to_vec(),
    };

    f.push(MetaTile {
        children: Vec::new(),
        transform: Affine2::from_cols_array_2d(&[[0.5, 0.0], [0.0, 0.5], [1.5, HR3]]),
        shape: TileType::FHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    });

    f.push(MetaTile {
        children: Vec::new(),
        transform: Affine2::from_cols_array_2d(&[
            [0.25, -0.5 * HR3],
            [0.5 * HR3, 0.25],
            [0.0, SQ3],
        ]),
        shape: TileType::FHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    });

    f
}

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
    for child in tree.children.clone() {
        make_polygons(polys, t.mul(tree.transform), &child)
    }

    let tt = t.mul(tree.transform);
    let points = tree
        .outline
        .iter()
        .map(|p| tt.transform_point2(*p))
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

fn setup(
    mut commands: Commands,
    // ass: Res<AssetServer>,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
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

    let h = h_init();
    let t = t_init();
    let p = p_init();
    let f = f_init();
    let mut a = AllFour {
        h: h.clone(),
        t: t.clone(),
        p: p.clone(),
        f: f.clone(),
    };

    for _ in 0..LEVELS {
        let patch = construct_patch(a.h, a.t, a.p, a.f);
        a = construct_meta_tiles(patch);
    }
    // let cap = 13_usize.pow(LEVELS.try_into().unwrap());
    let cap = 200_000;
    let which_meta_tile = a.h;

    let mut polys = HatPolys {
        h: Vec::with_capacity(cap),
        h1: Vec::with_capacity(cap),
        t: Vec::with_capacity(cap),
        f: Vec::with_capacity(cap),
        p: Vec::with_capacity(cap),
        meta: vec![Vec::new(); LEVELS + 2],
    };

    make_polygons(
        &mut polys,
        Affine2::from_scale(Vec2 { x: 5.0, y: 5.0 }),
        &which_meta_tile,
    );
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
        for chunk in polys.chunks(50_000) {
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
                Fill::color(shape_to_fill_color(*shape)),
                Stroke::new(STROKE_COLOR, STROKE_WIDTH),
            ));
        }
    }

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
    // std::process::exit(0);
}

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(ShapePlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(BG_COLOR))
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct HTile;
#[derive(Component)]
struct TTile;
#[derive(Component)]
struct PTile;
#[derive(Component)]
struct FTile;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum TileType {
    H,
    T,
    P,
    F,
    H1Hat,
    HHat,
    THat,
    PHat,
    FHat,
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

#[derive(Debug, Clone)]
struct MetaTile {
    transform: Affine2,
    shape: TileType,
    outline: Vec<Vec2>,
    width: usize,
    children: Vec<Rc<MetaTile>>,
}
use std::fmt;
impl fmt::Display for MetaTile {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "--\n{:?}\n", self.shape)?;
        write!(f, "{:?}\n", self.transform)?;
        write!(f, "{:?}\n", self.outline)?;
        write!(f, "{:?}\n---", self.width)
    }
}
impl MetaTile {
    pub fn new(transform: Affine2, shape: TileType, width: usize, outline: Vec<Vec2>) -> Self {
        Self {
            transform,
            shape,
            width,
            outline,
            children: Vec::new(),
        }
    }

    pub fn push(self: &mut MetaTile, mt: MetaTile) {
        self.children.push(Rc::new(mt))
    }

    pub fn push_rc(self: &mut MetaTile, mt: Rc<MetaTile>) {
        self.children.push(mt)
    }
}

fn shape_to_id(shape: TileType) -> usize {
    match shape {
        TileType::H => 0,
        TileType::T => 1,
        TileType::P => 2,
        TileType::F => 3,
        _ => panic!(),
    }
}

fn construct_patch(h: MetaTile, t: MetaTile, p: MetaTile, f: MetaTile) -> Vec<Rc<MetaTile>> {
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
                root.push(Rc::new(h));
            }
            Rule::Four(n_child, n_outline, shape, n_vertex) => {
                let child: Rc<MetaTile> = root[*n_child].clone();
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
                root.push(Rc::new(new_shape));
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
                root.push(Rc::new(new_shape));
            }
        }
    }
    root
}

struct AllFour {
    h: MetaTile,
    t: MetaTile,
    p: MetaTile,
    f: MetaTile,
}

fn eval_meta_tile(mt: &MetaTile, i: usize) -> Vec2 {
    let p = mt.outline[i];
    mt.transform.transform_point2(p)
}

fn construct_meta_tiles(patch: Vec<Rc<MetaTile>>) -> AllFour {
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
    let bbb = (new_h_outline[1] + (new_h_outline[4] - new_h_outline[5]));
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

fn shape_to_fill_color(shape: TileType) -> Color {
    let tr = 0.4;
    match shape {
        TileType::H1Hat => H_MIRROR_COLOR,
        TileType::HHat => H_COLOR,
        TileType::THat => T_COLOR,
        TileType::PHat => P_COLOR,
        TileType::FHat => F_COLOR,
        // TileType::H => Color::rgba(0.0, 0.0, 1.0, tr),
        // TileType::T => Color::rgba(0.0, 1.0, 1.0, tr),
        // TileType::P => Color::rgba(1.0, 0.0, 1.0, tr),
        // TileType::F => Color::rgba(1.0, 1.0, 0.0, tr),
        // TileType::Pseudo => _
        _ => Color::rgba(1.0, 1.0, 1.0, 0.0),
        // _ => Color::rgba(0.0, 0.0, 0.0, 0.0),
    }
}
