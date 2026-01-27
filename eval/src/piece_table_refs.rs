use movegen::piece;

use crate::{params, score_pair::ScorePair};

#[derive(Debug, Clone, Copy)]
pub struct PieceTableRefs<'a> {
    pub pst: &'a [ScorePair],
    pub friendly_distance: &'a [ScorePair],
    pub enemy_distance: &'a [ScorePair],
}

pub const PIECE_TABLE_REFS: [PieceTableRefs; 6] = {
    let piece_types = [
        piece::Type::Pawn,
        piece::Type::Knight,
        piece::Type::Bishop,
        piece::Type::Rook,
        piece::Type::Queen,
        piece::Type::King,
    ];
    let mut piece_tables = [PieceTableRefs {
        pst: &[],
        friendly_distance: &[],
        enemy_distance: &[],
    }; 6];
    let mut i = 0;
    while i < piece_types.len() {
        let pt = piece_types[i];
        let tables = match pt {
            piece::Type::Pawn => PieceTableRefs {
                pst: &params::PST_PAWN,
                friendly_distance: &params::DISTANCE_FRIENDLY_PAWN,
                enemy_distance: &params::DISTANCE_ENEMY_PAWN,
            },
            piece::Type::Knight => PieceTableRefs {
                pst: &params::PST_KNIGHT,
                friendly_distance: &params::DISTANCE_FRIENDLY_KNIGHT,
                enemy_distance: &params::DISTANCE_ENEMY_KNIGHT,
            },
            piece::Type::Bishop => PieceTableRefs {
                pst: &params::PST_BISHOP,
                friendly_distance: &params::DISTANCE_FRIENDLY_BISHOP,
                enemy_distance: &params::DISTANCE_ENEMY_BISHOP,
            },
            piece::Type::Rook => PieceTableRefs {
                pst: &params::PST_ROOK,
                friendly_distance: &params::DISTANCE_FRIENDLY_ROOK,
                enemy_distance: &params::DISTANCE_ENEMY_ROOK,
            },
            piece::Type::Queen => PieceTableRefs {
                pst: &params::PST_QUEEN,
                friendly_distance: &params::DISTANCE_FRIENDLY_QUEEN,
                enemy_distance: &params::DISTANCE_ENEMY_QUEEN,
            },
            piece::Type::King => PieceTableRefs {
                pst: &params::PST_KING,
                friendly_distance: &[],
                enemy_distance: &[],
            },
        };
        piece_tables[pt.idx()] = tables;
        i += 1;
    }
    piece_tables
};
