use movegen::piece;

use crate::{params, score_pair::ScorePair};

#[derive(Debug, Clone, Copy)]
pub struct PieceTableRefs<'a> {
    pub pst: &'a [ScorePair],
    pub piece_relative_to_friendly_king: &'a [ScorePair],
    pub piece_relative_to_enemy_king: &'a [ScorePair],
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
        piece_relative_to_friendly_king: &[],
        piece_relative_to_enemy_king: &[],
    }; 6];
    let mut i = 0;
    while i < piece_types.len() {
        let pt = piece_types[i];
        let tables = match pt {
            piece::Type::Pawn => PieceTableRefs {
                pst: &params::PST_PAWN,
                piece_relative_to_friendly_king: &params::PAWN_RELATIVE_TO_FRIENDLY_KING,
                piece_relative_to_enemy_king: &params::PAWN_RELATIVE_TO_ENEMY_KING,
            },
            piece::Type::Knight => PieceTableRefs {
                pst: &params::PST_KNIGHT,
                piece_relative_to_friendly_king: &params::KNIGHT_RELATIVE_TO_FRIENDLY_KING,
                piece_relative_to_enemy_king: &params::KNIGHT_RELATIVE_TO_ENEMY_KING,
            },
            piece::Type::Bishop => PieceTableRefs {
                pst: &params::PST_BISHOP,
                piece_relative_to_friendly_king: &params::BISHOP_RELATIVE_TO_FRIENDLY_KING,
                piece_relative_to_enemy_king: &params::BISHOP_RELATIVE_TO_ENEMY_KING,
            },
            piece::Type::Rook => PieceTableRefs {
                pst: &params::PST_ROOK,
                piece_relative_to_friendly_king: &params::ROOK_RELATIVE_TO_FRIENDLY_KING,
                piece_relative_to_enemy_king: &params::ROOK_RELATIVE_TO_ENEMY_KING,
            },
            piece::Type::Queen => PieceTableRefs {
                pst: &params::PST_QUEEN,
                piece_relative_to_friendly_king: &params::QUEEN_RELATIVE_TO_FRIENDLY_KING,
                piece_relative_to_enemy_king: &params::QUEEN_RELATIVE_TO_ENEMY_KING,
            },
            piece::Type::King => PieceTableRefs {
                pst: &params::PST_KING,
                piece_relative_to_friendly_king: &[],
                piece_relative_to_enemy_king: &[],
            },
        };
        piece_tables[pt.idx()] = tables;
        i += 1;
    }
    piece_tables
};
