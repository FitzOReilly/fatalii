use eval::{pawn_structure::PawnStructure, GamePhase};
use movegen::{bitboard::Bitboard, piece, position::Position, side::Side};
use nalgebra_sparse::{CooMatrix, CsrMatrix};

pub type EvalType = f64;
pub type FeatureType = f64;
pub type FeatureVector = CsrMatrix<FeatureType>;

pub const PST_SIZE: usize = 32;
const NUM_PIECES: usize = 6;
const NUM_PST_FEATURES: usize = 2 * NUM_PIECES * PST_SIZE;
const NUM_TEMPO_FEATURES: usize = 2;
const NUM_PASSED_PAWN_FEATURES: usize = 2;
const NUM_ISOLATED_PAWN_FEATURES: usize = 2;
const NUM_BACKWARD_PAWN_FEATURES: usize = 2;
pub const NUM_FEATURES: usize = NUM_PST_FEATURES
    + NUM_TEMPO_FEATURES
    + NUM_PASSED_PAWN_FEATURES
    + NUM_ISOLATED_PAWN_FEATURES
    + NUM_BACKWARD_PAWN_FEATURES;

pub const START_IDX_PST: usize = 0;
pub const START_IDX_TEMPO: usize = START_IDX_PST + NUM_PST_FEATURES;
pub const START_IDX_PASSED_PAWN: usize = START_IDX_TEMPO + NUM_TEMPO_FEATURES;
pub const START_IDX_ISOLATED_PAWN: usize = START_IDX_PASSED_PAWN + NUM_PASSED_PAWN_FEATURES;
pub const START_IDX_BACKWARD_PAWN: usize = START_IDX_ISOLATED_PAWN + NUM_ISOLATED_PAWN_FEATURES;

#[derive(Debug, Clone)]
pub struct PositionFeatures {
    pub phase: EvalType,
    pub feature_vec: FeatureVector,
}

impl PositionFeatures {
    pub fn new(phase: EvalType, features: CooMatrix<FeatureType>) -> Self {
        Self {
            phase,
            feature_vec: FeatureVector::from(&features),
        }
    }

    pub fn grad(&self) -> FeatureVector {
        self.feature_vec.clone()
    }
}

impl From<&Position> for PositionFeatures {
    fn from(pos: &Position) -> Self {
        let mut features = CooMatrix::new(1, NUM_FEATURES);
        let game_phase = extract_psts(&mut features, pos);
        extract_tempo(&mut features, pos);
        extract_pawn_structure(&mut features, pos);

        let mg_phase = 1.0 - game_phase;
        let eg_phase = game_phase;
        for (_row, col, feat) in features.triplet_iter_mut() {
            *feat *= match col % 2 {
                0 => mg_phase,
                _ => eg_phase,
            };
        }

        PositionFeatures::new(game_phase, features)
    }
}

fn extract_psts(features: &mut CooMatrix<FeatureType>, pos: &Position) -> EvalType {
    let mut game_phase = GamePhase::default();
    let mut offset = START_IDX_PST;

    for piece_type in [
        piece::Type::Pawn,
        piece::Type::Knight,
        piece::Type::Bishop,
        piece::Type::Rook,
        piece::Type::Queen,
        piece::Type::King,
    ] {
        let mut white_pieces = pos.piece_occupancy(Side::White, piece_type);
        while white_pieces != Bitboard::EMPTY {
            let square = white_pieces.square_scan_forward_reset().fold_to_queenside();
            // Middlegame
            features.push(0, offset + 2 * square.idx(), 1.0);
            // Endgame
            features.push(0, offset + 2 * square.idx() + 1, 1.0);
            game_phase.add_piece(piece_type);
        }
        let mut black_pieces = pos.piece_occupancy(Side::Black, piece_type);
        while black_pieces != Bitboard::EMPTY {
            let square = black_pieces
                .square_scan_forward_reset()
                .flip_vertical()
                .fold_to_queenside();
            // Middlegame
            features.push(0, offset + 2 * square.idx(), -1.0);
            // Endgame
            features.push(0, offset + 2 * square.idx() + 1, -1.0);
            game_phase.add_piece(piece_type);
        }
        offset += 2 * PST_SIZE;
    }

    (GamePhase::MAX - game_phase.game_phase_clamped()) as EvalType / GamePhase::MAX as EvalType
}

fn extract_tempo(features: &mut CooMatrix<FeatureType>, pos: &Position) {
    let val = match pos.side_to_move() {
        Side::White => 1.0,
        Side::Black => -1.0,
    };
    features.push(0, START_IDX_TEMPO, val);
    features.push(0, START_IDX_TEMPO + 1, val);
}

fn extract_pawn_structure(features: &mut CooMatrix<FeatureType>, pos: &Position) {
    let white_pawns = pos.piece_occupancy(Side::White, piece::Type::Pawn);
    let black_pawns = pos.piece_occupancy(Side::Black, piece::Type::Pawn);
    let passed_pawn_count = PawnStructure::passed_pawn_count(white_pawns, black_pawns).into();
    features.push(0, START_IDX_PASSED_PAWN, passed_pawn_count);
    features.push(0, START_IDX_PASSED_PAWN + 1, passed_pawn_count);
    let isolated_pawn_count = PawnStructure::isolated_pawn_count(white_pawns, black_pawns).into();
    features.push(0, START_IDX_ISOLATED_PAWN, isolated_pawn_count);
    features.push(0, START_IDX_ISOLATED_PAWN + 1, isolated_pawn_count);
    let backward_pawn_count = PawnStructure::backward_pawn_count(white_pawns, black_pawns).into();
    features.push(0, START_IDX_BACKWARD_PAWN, backward_pawn_count);
    features.push(0, START_IDX_BACKWARD_PAWN + 1, backward_pawn_count);
}
