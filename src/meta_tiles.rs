use bevy::math::Affine2;
use bevy::math::Vec2;
use std::sync::Arc;

use crate::utils::match_two;

const SQ3: f32 = 1.732_050_8; // sqrt(3)
const HR3: f32 = 0.866_025_4; // sqrt(3) / 2

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
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

#[derive(Debug, Clone)]
pub struct MetaTile {
    pub transform: Affine2,
    pub shape: TileType,
    pub outline: Vec<Vec2>,
    pub width: usize,
    pub children: Vec<Arc<MetaTile>>,
}
use std::fmt;
impl fmt::Display for MetaTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "--\n{:?}\n", self.shape)?;
        writeln!(f, "{:?}", self.transform)?;
        writeln!(f, "{:?}", self.outline)?;
        write!(f, "{:?}\n---", self.width)
    }
}

impl MetaTile {
    pub fn push(&mut self, mt: Self) {
        self.children.push(Arc::new(mt))
    }

    pub fn push_rc(&mut self, mt: Arc<Self>) {
        self.children.push(mt)
    }
}

pub const HAT_OUTLINE: &[Vec2] = &[
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

pub fn h_init() -> MetaTile {
    let mut h = MetaTile {
        transform: Affine2::IDENTITY,
        shape: TileType::H,
        width: 2,
        outline: H_OUTLINE.to_vec(),
        children: Vec::new(),
    };

    h.push_rc(Arc::new(MetaTile {
        children: Vec::new(),
        transform: match_two(HAT_OUTLINE[5], HAT_OUTLINE[7], H_OUTLINE[5], H_OUTLINE[0]),
        shape: TileType::HHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));

    h.push_rc(Arc::new(MetaTile {
        children: Vec::new(),
        transform: match_two(HAT_OUTLINE[9], HAT_OUTLINE[11], H_OUTLINE[1], H_OUTLINE[2]),
        shape: TileType::HHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));

    h.push_rc(Arc::new(MetaTile {
        children: Vec::new(),
        transform: match_two(HAT_OUTLINE[5], HAT_OUTLINE[7], H_OUTLINE[3], H_OUTLINE[4]),
        shape: TileType::HHat,
        width: 1,
        outline: HAT_OUTLINE.to_vec(),
    }));

    h.push_rc(Arc::new(MetaTile {
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

pub fn t_init() -> MetaTile {
    let mut t = MetaTile {
        children: Vec::new(),
        transform: Affine2::IDENTITY,
        shape: TileType::T,
        width: 2,
        outline: T_OUTLINE.to_vec(),
    };

    t.push_rc(Arc::new(MetaTile {
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

pub fn p_init() -> MetaTile {
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

pub fn f_init() -> MetaTile {
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
