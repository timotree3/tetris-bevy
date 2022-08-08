use bevy::prelude::Color;
use rand::{rngs::SmallRng, seq::SliceRandom};

use crate::FallingSegment;

#[derive(Clone, Copy)]
pub(crate) struct Tetromino {
    pub shape: [FallingSegment; 4],
    pub color: Color,
}

const I: Tetromino = Tetromino {
    shape: [
        FallingSegment {
            x_offset: 0,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: -1,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: 1,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: 2,
            y_offset: 0,
        },
    ],
    color: Color::TEAL,
};
const T: Tetromino = Tetromino {
    shape: [
        FallingSegment {
            x_offset: 0,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: -1,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: 1,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: 0,
            y_offset: 1,
        },
    ],
    color: Color::PURPLE,
};
const J: Tetromino = Tetromino {
    shape: [
        FallingSegment {
            x_offset: 0,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: -1,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: 1,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: 1,
            y_offset: 1,
        },
    ],
    color: Color::ORANGE,
};
const L: Tetromino = Tetromino {
    shape: [
        FallingSegment {
            x_offset: 0,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: -1,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: -1,
            y_offset: 1,
        },
        FallingSegment {
            x_offset: 1,
            y_offset: 0,
        },
    ],
    color: Color::BLUE,
};

const Z: Tetromino = Tetromino {
    shape: [
        FallingSegment {
            x_offset: 0,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: 0,
            y_offset: 1,
        },
        FallingSegment {
            x_offset: -1,
            y_offset: 1,
        },
        FallingSegment {
            x_offset: 1,
            y_offset: 0,
        },
    ],
    color: Color::RED,
};

const S: Tetromino = Tetromino {
    shape: [
        FallingSegment {
            x_offset: 0,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: 0,
            y_offset: 1,
        },
        FallingSegment {
            x_offset: -1,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: 1,
            y_offset: 1,
        },
    ],
    color: Color::GREEN,
};
const O: Tetromino = Tetromino {
    shape: [
        FallingSegment {
            x_offset: 0,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: 0,
            y_offset: 1,
        },
        FallingSegment {
            x_offset: -1,
            y_offset: 0,
        },
        FallingSegment {
            x_offset: -1,
            y_offset: 1,
        },
    ],
    color: Color::YELLOW,
};
impl Tetromino {
    pub fn random(rng: &mut SmallRng) -> Tetromino {
        *[I, T, L, J, S, Z, O].choose(rng).unwrap()
    }
}
