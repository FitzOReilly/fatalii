use crate::attacks_to::AttacksTo;
use crate::bishop::Bishop;
use crate::bitboard::Bitboard;
use crate::king::King;
use crate::knight::Knight;
use crate::pawn::Pawn;
use crate::piece;
use crate::piece_targets::PieceTargets;
use crate::position::CastlingRights;
use crate::queen::Queen;
use crate::r#move::{Move, MoveList, MoveType};
use crate::rank::Rank;
use crate::rook::Rook;
use crate::side::Side;
use crate::square::Square;

// A move generator template that allows implementers to customize their legality checks
pub trait MoveGeneratorTemplate {
    fn non_capture_target_filter(attacks_to_king: &AttacksTo, targets: Bitboard) -> Bitboard;
    fn capture_target_filter(attacks_to_king: &AttacksTo, targets: Bitboard) -> Bitboard;
    fn pawn_capture_target_filter(attacks_to_king: &AttacksTo, targets: Bitboard) -> Bitboard;

    fn is_legal_non_capture(attacks_to_king: &AttacksTo, origin: Square, target: Square) -> bool;
    fn is_legal_capture(attacks_to_king: &AttacksTo, origin: Square, target: Square) -> bool;
    fn is_legal_en_passant_capture(
        attacks_to_king: &AttacksTo,
        origin: Square,
        target: Square,
    ) -> bool;
    fn is_legal_king_move(attacks_to_king: &AttacksTo, origin: Square, target: Square) -> bool;

    fn has_en_passant_capture(attacks_to_king: &AttacksTo) -> bool {
        let pos = attacks_to_king.pos;
        if pos.en_passant_square() == Bitboard::EMPTY {
            return false;
        }

        let potential_origins =
            Pawn::west_attack_origins(pos.en_passant_square(), pos.side_to_move())
                | Pawn::east_attack_origins(pos.en_passant_square(), pos.side_to_move());
        let own_pawns = pos.piece_occupancy(pos.side_to_move(), piece::Type::Pawn);
        let mut attack_origins = potential_origins & own_pawns;
        if attack_origins == Bitboard::EMPTY {
            return false;
        }

        let target = pos.en_passant_square().to_square();
        while attack_origins != Bitboard::EMPTY {
            let origin = attack_origins.square_scan_forward_reset();
            if Self::is_legal_en_passant_capture(attacks_to_king, origin, target) {
                return true;
            }
        }

        false
    }

    fn generate_moves(move_list: &mut MoveList, attacks_to_king: &AttacksTo) {
        Self::generate_pawn_moves(move_list, attacks_to_king);
        Self::generate_knight_moves(move_list, attacks_to_king);
        Self::generate_sliding_piece_moves(
            move_list,
            attacks_to_king,
            piece::Type::Bishop,
            Bishop::targets,
        );
        Self::generate_sliding_piece_moves(
            move_list,
            attacks_to_king,
            piece::Type::Rook,
            Rook::targets,
        );
        Self::generate_sliding_piece_moves(
            move_list,
            attacks_to_king,
            piece::Type::Queen,
            Queen::targets,
        );
        Self::generate_king_moves(move_list, attacks_to_king);
        Self::generate_castles(move_list, attacks_to_king);
    }

    fn generate_captures(move_list: &mut MoveList, attacks_to_king: &AttacksTo) {
        Self::generate_pawn_captures(move_list, attacks_to_king);
        Self::generate_knight_captures(move_list, attacks_to_king);
        Self::generate_sliding_piece_captures(
            move_list,
            attacks_to_king,
            piece::Type::Bishop,
            Bishop::targets,
        );
        Self::generate_sliding_piece_captures(
            move_list,
            attacks_to_king,
            piece::Type::Rook,
            Rook::targets,
        );
        Self::generate_sliding_piece_captures(
            move_list,
            attacks_to_king,
            piece::Type::Queen,
            Queen::targets,
        );
        Self::generate_king_captures(move_list, attacks_to_king);
    }

    fn generate_king_moves(move_list: &mut MoveList, attacks_to_king: &AttacksTo) {
        let pos = attacks_to_king.pos;
        let own_occupancy = pos.side_occupancy(pos.side_to_move());
        let origin = attacks_to_king.target;
        let targets = King::targets(origin) & !own_occupancy & !attacks_to_king.all_attack_targets;

        let opponents = pos.side_occupancy(!pos.side_to_move());
        let mut captures = targets & opponents;
        let mut quiets = targets & !captures;

        while captures != Bitboard::EMPTY {
            let target = captures.square_scan_forward_reset();
            if Self::is_legal_king_move(attacks_to_king, origin, target) {
                move_list.push(Move::new(origin, target, MoveType::CAPTURE));
            }
        }
        while quiets != Bitboard::EMPTY {
            let target = quiets.square_scan_forward_reset();
            if Self::is_legal_king_move(attacks_to_king, origin, target) {
                move_list.push(Move::new(origin, target, MoveType::QUIET));
            }
        }
    }

    fn generate_king_captures(move_list: &mut MoveList, attacks_to_king: &AttacksTo) {
        let pos = attacks_to_king.pos;
        let own_occupancy = pos.side_occupancy(pos.side_to_move());
        let origin = attacks_to_king.target;
        let targets = King::targets(origin) & !own_occupancy & !attacks_to_king.all_attack_targets;

        let opponents = pos.side_occupancy(!pos.side_to_move());
        let mut captures = targets & opponents;

        while captures != Bitboard::EMPTY {
            let target = captures.square_scan_forward_reset();
            if Self::is_legal_king_move(attacks_to_king, origin, target) {
                move_list.push(Move::new(origin, target, MoveType::CAPTURE));
            }
        }
    }

    fn generate_knight_moves(move_list: &mut MoveList, attacks_to_king: &AttacksTo) {
        let pos = attacks_to_king.pos;
        let mut knights = pos.piece_occupancy(pos.side_to_move(), piece::Type::Knight);
        let own_occupancy = pos.side_occupancy(pos.side_to_move());
        while knights != Bitboard::EMPTY {
            let origin = knights.square_scan_forward_reset();
            let targets = Knight::targets(origin) & !own_occupancy;
            Self::generate_piece_moves(move_list, attacks_to_king, origin, targets);
        }
    }

    fn generate_knight_captures(move_list: &mut MoveList, attacks_to_king: &AttacksTo) {
        let pos = attacks_to_king.pos;
        let mut knights = pos.piece_occupancy(pos.side_to_move(), piece::Type::Knight);
        let own_occupancy = pos.side_occupancy(pos.side_to_move());
        while knights != Bitboard::EMPTY {
            let origin = knights.square_scan_forward_reset();
            let targets = Knight::targets(origin) & !own_occupancy;
            Self::generate_piece_captures(move_list, attacks_to_king, origin, targets);
        }
    }

    fn generate_sliding_piece_moves(
        move_list: &mut MoveList,
        attacks_to_king: &AttacksTo,
        piece_type: piece::Type,
        piece_targets: fn(Square, Bitboard) -> Bitboard,
    ) {
        let pos = attacks_to_king.pos;
        let mut piece_occupancy = pos.piece_occupancy(pos.side_to_move(), piece_type);
        let own_occupancy = pos.side_occupancy(pos.side_to_move());
        while piece_occupancy != Bitboard::EMPTY {
            let origin = piece_occupancy.square_scan_forward_reset();
            let targets = piece_targets(origin, pos.occupancy()) & !own_occupancy;
            Self::generate_piece_moves(move_list, attacks_to_king, origin, targets);
        }
    }

    fn generate_sliding_piece_captures(
        move_list: &mut MoveList,
        attacks_to_king: &AttacksTo,
        piece_type: piece::Type,
        piece_targets: fn(Square, Bitboard) -> Bitboard,
    ) {
        let pos = attacks_to_king.pos;
        let mut piece_occupancy = pos.piece_occupancy(pos.side_to_move(), piece_type);
        let own_occupancy = pos.side_occupancy(pos.side_to_move());
        while piece_occupancy != Bitboard::EMPTY {
            let origin = piece_occupancy.square_scan_forward_reset();
            let targets = piece_targets(origin, pos.occupancy()) & !own_occupancy;
            Self::generate_piece_captures(move_list, attacks_to_king, origin, targets);
        }
    }

    fn generate_piece_moves(
        move_list: &mut MoveList,
        attacks_to_king: &AttacksTo,
        origin: Square,
        targets: Bitboard,
    ) {
        Self::generate_piece_captures(move_list, attacks_to_king, origin, targets);
        Self::generate_piece_quiets(move_list, attacks_to_king, origin, targets);
    }

    fn generate_piece_captures(
        move_list: &mut MoveList,
        attacks_to_king: &AttacksTo,
        origin: Square,
        targets: Bitboard,
    ) {
        let mut captures = Self::capture_target_filter(attacks_to_king, targets);
        while captures != Bitboard::EMPTY {
            let target = captures.square_scan_forward_reset();
            if Self::is_legal_capture(attacks_to_king, origin, target) {
                let m = Move::new(origin, target, MoveType::CAPTURE);
                move_list.push(m);
            }
        }
    }

    fn generate_piece_quiets(
        move_list: &mut MoveList,
        attacks_to_king: &AttacksTo,
        origin: Square,
        targets: Bitboard,
    ) {
        let mut quiets = Self::non_capture_target_filter(attacks_to_king, targets);
        while quiets != Bitboard::EMPTY {
            let target = quiets.square_scan_forward_reset();
            if Self::is_legal_non_capture(attacks_to_king, origin, target) {
                let m = Move::new(origin, target, MoveType::QUIET);
                move_list.push(m);
            }
        }
    }

    fn generate_pawn_moves(move_list: &mut MoveList, attacks_to_king: &AttacksTo) {
        Self::generate_pawn_pushes(move_list, attacks_to_king);
        Self::generate_pawn_captures(move_list, attacks_to_king);
    }

    fn generate_pawn_pushes(move_list: &mut MoveList, attacks_to_king: &AttacksTo) {
        let pos = attacks_to_king.pos;
        let side_to_move = pos.side_to_move();
        let own_pawns = pos.piece_occupancy(pos.side_to_move(), piece::Type::Pawn);

        let (single_push_targets, double_push_targets) =
            Pawn::push_targets(own_pawns, pos.occupancy(), side_to_move);

        let single_push_targets =
            Self::non_capture_target_filter(attacks_to_king, single_push_targets);
        let mut double_push_targets =
            Self::non_capture_target_filter(attacks_to_king, double_push_targets);
        let mut promo_targets = single_push_targets & Pawn::promotion_rank(side_to_move);
        let mut non_promo_targets = single_push_targets & !promo_targets;

        while promo_targets != Bitboard::EMPTY {
            let target = promo_targets.square_scan_forward_reset();
            let origin = Pawn::push_origin(target, side_to_move);
            if Self::is_legal_non_capture(attacks_to_king, origin, target) {
                for promo_piece in &[
                    piece::Type::Queen,
                    piece::Type::Rook,
                    piece::Type::Bishop,
                    piece::Type::Knight,
                ] {
                    let m = Move::new(
                        origin,
                        target,
                        MoveType::new_with_promo_piece(MoveType::PROMOTION, *promo_piece),
                    );
                    move_list.push(m);
                }
            }
        }
        while non_promo_targets != Bitboard::EMPTY {
            let target = non_promo_targets.square_scan_forward_reset();
            let origin = Pawn::push_origin(target, side_to_move);
            if Self::is_legal_non_capture(attacks_to_king, origin, target) {
                let m = Move::new(origin, target, MoveType::QUIET);
                move_list.push(m);
            }
        }
        while double_push_targets != Bitboard::EMPTY {
            let target = double_push_targets.square_scan_forward_reset();
            let origin = Pawn::double_push_origin(target, side_to_move);
            if Self::is_legal_non_capture(attacks_to_king, origin, target) {
                let m = Move::new(origin, target, MoveType::DOUBLE_PAWN_PUSH);
                move_list.push(m);
            }
        }
    }

    fn generate_pawn_captures(move_list: &mut MoveList, attacks_to_king: &AttacksTo) {
        Self::generate_pawn_captures_one_side(
            move_list,
            attacks_to_king,
            Pawn::east_attack_targets,
            Pawn::east_attack_origin,
        );

        Self::generate_pawn_captures_one_side(
            move_list,
            attacks_to_king,
            Pawn::west_attack_targets,
            Pawn::west_attack_origin,
        );
    }

    fn generate_pawn_captures_one_side(
        move_list: &mut MoveList,
        attacks_to_king: &AttacksTo,
        attacks: fn(Bitboard, Side) -> Bitboard,
        attack_origin: fn(Square, Side) -> Square,
    ) {
        let pos = attacks_to_king.pos;
        let side_to_move = pos.side_to_move();
        let own_pawns = pos.piece_occupancy(side_to_move, piece::Type::Pawn);
        let en_passant_square = pos.en_passant_square();
        let promo_rank = Pawn::promotion_rank(side_to_move);
        let targets = attacks(own_pawns, side_to_move);
        let captures = Self::pawn_capture_target_filter(attacks_to_king, targets);

        let mut promo_captures = captures & promo_rank;
        let mut non_promo_captures = captures & !promo_captures;

        while promo_captures != Bitboard::EMPTY {
            let target = promo_captures.square_scan_forward_reset();
            let origin = attack_origin(target, side_to_move);
            if Self::is_legal_capture(attacks_to_king, origin, target) {
                for promo_piece in &[
                    piece::Type::Queen,
                    piece::Type::Rook,
                    piece::Type::Bishop,
                    piece::Type::Knight,
                ] {
                    let m = Move::new(
                        origin,
                        target,
                        MoveType::new_with_promo_piece(MoveType::PROMOTION_CAPTURE, *promo_piece),
                    );
                    move_list.push(m);
                }
            }
        }

        while non_promo_captures != Bitboard::EMPTY {
            let target = non_promo_captures.square_scan_forward_reset();
            let origin = attack_origin(target, side_to_move);
            if Bitboard::from_square(target) == en_passant_square {
                if Self::is_legal_en_passant_capture(attacks_to_king, origin, target) {
                    let m = Move::new(origin, target, MoveType::EN_PASSANT_CAPTURE);
                    move_list.push(m);
                }
            } else if Self::is_legal_capture(attacks_to_king, origin, target) {
                let m = Move::new(origin, target, MoveType::CAPTURE);
                move_list.push(m);
            };
        }
    }

    fn sliding_piece_targets(xray: &PieceTargets, occupancy: Bitboard) -> Bitboard {
        match xray.piece().piece_type() {
            piece::Type::Bishop => Bishop::targets(xray.origin(), occupancy),
            piece::Type::Rook => Rook::targets(xray.origin(), occupancy),
            piece::Type::Queen => Queen::targets(xray.origin(), occupancy),
            _ => panic!(
                "Expected sliding piece, found `{:?}` (in `{:?}`)",
                xray.piece().piece_type(),
                xray
            ),
        }
    }

    fn generate_castles(move_list: &mut MoveList, attacks_to_king: &AttacksTo) {
        match attacks_to_king.pos.side_to_move() {
            Side::White => Self::generate_white_castles(move_list, attacks_to_king),
            Side::Black => Self::generate_black_castles(move_list, attacks_to_king),
        }
    }

    fn generate_white_castles(move_list: &mut MoveList, attacks_to_king: &AttacksTo) {
        let pos = attacks_to_king.pos;
        let castling_rights = pos.castling_rights();

        if castling_rights.contains(CastlingRights::WHITE_KINGSIDE) {
            let king_square = Square::from_file_and_rank(pos.king_start_file(), Rank::R1);
            let rook_square = Square::from_file_and_rank(pos.kingside_castling_file(), Rank::R1);
            debug_assert_eq!(Some(piece::Piece::WHITE_KING), pos.piece_at(king_square));
            debug_assert_eq!(Some(piece::Piece::WHITE_ROOK), pos.piece_at(rook_square));
            let squares_passable = pos.occupancy()
                & pos.castling_squares().white_kingside().non_blocked
                == Bitboard::EMPTY;
            let squares_attacked = attacks_to_king.all_attack_targets
                & pos.castling_squares().white_kingside().non_attacked
                != Bitboard::EMPTY;
            if squares_passable && !squares_attacked {
                move_list.push(Move::new(
                    king_square,
                    Square::G1,
                    MoveType::CASTLE_KINGSIDE,
                ));
            }
        }
        if castling_rights.contains(CastlingRights::WHITE_QUEENSIDE) {
            let king_square = Square::from_file_and_rank(pos.king_start_file(), Rank::R1);
            let rook_square = Square::from_file_and_rank(pos.queenside_castling_file(), Rank::R1);
            debug_assert_eq!(Some(piece::Piece::WHITE_KING), pos.piece_at(king_square));
            debug_assert_eq!(Some(piece::Piece::WHITE_ROOK), pos.piece_at(rook_square));
            let squares_passable = pos.occupancy()
                & pos.castling_squares().white_queenside().non_blocked
                == Bitboard::EMPTY;
            let squares_attacked = attacks_to_king.all_attack_targets
                & pos.castling_squares().white_queenside().non_attacked
                != Bitboard::EMPTY;
            if squares_passable && !squares_attacked {
                move_list.push(Move::new(
                    king_square,
                    Square::C1,
                    MoveType::CASTLE_QUEENSIDE,
                ));
            }
        }
    }

    fn generate_black_castles(move_list: &mut MoveList, attacks_to_king: &AttacksTo) {
        let pos = attacks_to_king.pos;
        let castling_rights = pos.castling_rights();

        if castling_rights.contains(CastlingRights::BLACK_KINGSIDE) {
            let king_square = Square::from_file_and_rank(pos.king_start_file(), Rank::R8);
            let rook_square = Square::from_file_and_rank(pos.kingside_castling_file(), Rank::R8);
            debug_assert_eq!(Some(piece::Piece::BLACK_KING), pos.piece_at(king_square));
            debug_assert_eq!(Some(piece::Piece::BLACK_ROOK), pos.piece_at(rook_square));
            let squares_passable = pos.occupancy()
                & pos.castling_squares().black_kingside().non_blocked
                == Bitboard::EMPTY;
            let squares_attacked = attacks_to_king.all_attack_targets
                & pos.castling_squares().black_kingside().non_attacked
                != Bitboard::EMPTY;
            if squares_passable && !squares_attacked {
                move_list.push(Move::new(
                    king_square,
                    Square::G8,
                    MoveType::CASTLE_KINGSIDE,
                ));
            }
        }
        if castling_rights.contains(CastlingRights::BLACK_QUEENSIDE) {
            let king_square = Square::from_file_and_rank(pos.king_start_file(), Rank::R8);
            let rook_square = Square::from_file_and_rank(pos.queenside_castling_file(), Rank::R8);
            debug_assert_eq!(Some(piece::Piece::BLACK_KING), pos.piece_at(king_square));
            debug_assert_eq!(Some(piece::Piece::BLACK_ROOK), pos.piece_at(rook_square));
            let squares_passable = pos.occupancy()
                & pos.castling_squares().black_queenside().non_blocked
                == Bitboard::EMPTY;
            let squares_attacked = attacks_to_king.all_attack_targets
                & pos.castling_squares().black_queenside().non_attacked
                != Bitboard::EMPTY;
            if squares_passable && !squares_attacked {
                move_list.push(Move::new(
                    king_square,
                    Square::C8,
                    MoveType::CASTLE_QUEENSIDE,
                ));
            }
        }
    }
}
