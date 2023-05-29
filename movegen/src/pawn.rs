use crate::bitboard::Bitboard;
use crate::side::Side;
use crate::square::Square;

pub struct Pawn;

impl Pawn {
    pub fn push_targets(
        pawns: Bitboard,
        occupied: Bitboard,
        side_to_move: Side,
    ) -> (Bitboard, Bitboard) {
        [Self::white_push_targets, Self::black_push_targets][side_to_move as usize](pawns, occupied)
    }

    pub fn east_attack_targets(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        match side_to_move {
            Side::White => pawns.north_east_one_if_rank_8_empty(),
            Side::Black => pawns.south_east_one_if_rank_1_empty(),
        }
    }

    pub fn west_attack_targets(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        match side_to_move {
            Side::White => pawns.north_west_one_if_rank_8_empty(),
            Side::Black => pawns.south_west_one_if_rank_1_empty(),
        }
    }

    pub fn east_attack_origins(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        match side_to_move {
            Side::White => pawns.south_west_one_if_rank_1_empty(),
            Side::Black => pawns.north_west_one_if_rank_8_empty(),
        }
    }

    pub fn west_attack_origins(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        match side_to_move {
            Side::White => pawns.south_east_one_if_rank_1_empty(),
            Side::Black => pawns.north_east_one_if_rank_8_empty(),
        }
    }

    pub fn attack_targets(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        Self::east_attack_targets(pawns, side_to_move)
            | Self::west_attack_targets(pawns, side_to_move)
    }

    pub fn single_push_origins(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        [Bitboard::south_one, Bitboard::north_one][side_to_move as usize](pawns)
    }

    pub fn push_origin(target: Square, side_to_move: Side) -> Square {
        [Square::south_one, Square::north_one][side_to_move as usize](target)
    }

    pub fn double_push_origin(target: Square, side_to_move: Side) -> Square {
        [Square::south_two, Square::north_two][side_to_move as usize](target)
    }

    pub fn east_attack_origin(target: Square, side_to_move: Side) -> Square {
        [Square::south_west_one, Square::north_west_one][side_to_move as usize](target)
    }

    pub fn west_attack_origin(target: Square, side_to_move: Side) -> Square {
        [Square::south_east_one, Square::north_east_one][side_to_move as usize](target)
    }

    pub const fn promotion_rank(side_to_move: Side) -> Bitboard {
        [Bitboard::RANK_8, Bitboard::RANK_1][side_to_move as usize]
    }

    fn white_push_targets(white_pawns: Bitboard, occupied: Bitboard) -> (Bitboard, Bitboard) {
        let empty = !occupied;
        let single_push_targets = white_pawns.north_one_if_rank_8_empty() & empty;
        let double_push_targets =
            (single_push_targets & Bitboard::RANK_3).north_one_if_rank_8_empty() & empty;
        (single_push_targets, double_push_targets)
    }

    fn black_push_targets(black_pawns: Bitboard, occupied: Bitboard) -> (Bitboard, Bitboard) {
        let empty = !occupied;
        let single_push_targets = black_pawns.south_one_if_rank_1_empty() & empty;
        let double_push_targets =
            (single_push_targets & Bitboard::RANK_6).south_one_if_rank_1_empty() & empty;
        (single_push_targets, double_push_targets)
    }

    pub fn front_fill(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        match side_to_move {
            Side::White => pawns.north_fill(),
            Side::Black => pawns.south_fill(),
        }
    }

    pub fn rear_fill(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        match side_to_move {
            Side::White => pawns.south_fill(),
            Side::Black => pawns.north_fill(),
        }
    }

    pub fn front_span(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        match side_to_move {
            Side::White => pawns.north_span(),
            Side::Black => pawns.south_span(),
        }
    }

    pub fn rear_span(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        match side_to_move {
            Side::White => pawns.south_span(),
            Side::Black => pawns.north_span(),
        }
    }

    pub fn east_front_attack_span(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        Pawn::front_fill(Self::east_attack_targets(pawns, side_to_move), side_to_move)
    }

    pub fn west_front_attack_span(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        Pawn::front_fill(Self::west_attack_targets(pawns, side_to_move), side_to_move)
    }

    pub fn front_attack_span(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        Pawn::front_fill(Self::attack_targets(pawns, side_to_move), side_to_move)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_push_targets() {
        let occupied = Bitboard::A1
            | Bitboard::B2
            | Bitboard::C3
            | Bitboard::D4
            | Bitboard::E5
            | Bitboard::F6
            | Bitboard::G7
            | Bitboard::H8;

        let white_pawns = Bitboard::RANK_2;
        let (single_push_targets, double_push_targets) =
            Pawn::push_targets(white_pawns, occupied, Side::White);
        assert_eq!(Bitboard::RANK_3 & !Bitboard::C3, single_push_targets);
        assert_eq!(
            Bitboard::RANK_4 & !Bitboard::C4 & !Bitboard::D4,
            double_push_targets
        );

        let white_pawns = Bitboard::RANK_3;
        let (single_push_targets, double_push_targets) =
            Pawn::push_targets(white_pawns, occupied, Side::White);
        assert_eq!(Bitboard::RANK_4 & !Bitboard::D4, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);

        let white_pawns = Bitboard::RANK_4;
        let (single_push_targets, double_push_targets) =
            Pawn::push_targets(white_pawns, occupied, Side::White);
        assert_eq!(Bitboard::RANK_5 & !Bitboard::E5, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);

        let white_pawns = Bitboard::RANK_5;
        let (single_push_targets, double_push_targets) =
            Pawn::push_targets(white_pawns, occupied, Side::White);
        assert_eq!(Bitboard::RANK_6 & !Bitboard::F6, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);

        let white_pawns = Bitboard::RANK_6;
        let (single_push_targets, double_push_targets) =
            Pawn::push_targets(white_pawns, occupied, Side::White);
        assert_eq!(Bitboard::RANK_7 & !Bitboard::G7, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);

        let white_pawns = Bitboard::RANK_7;
        let (single_push_targets, double_push_targets) =
            Pawn::push_targets(white_pawns, occupied, Side::White);
        assert_eq!(Bitboard::RANK_8 & !Bitboard::H8, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);
    }

    #[test]
    fn black_push_targets() {
        let occupied = Bitboard::A1
            | Bitboard::B2
            | Bitboard::C3
            | Bitboard::D4
            | Bitboard::E5
            | Bitboard::F6
            | Bitboard::G7
            | Bitboard::H8;

        let black_pawns = Bitboard::RANK_7;
        let (single_push_targets, double_push_targets) =
            Pawn::push_targets(black_pawns, occupied, Side::Black);
        assert_eq!(Bitboard::RANK_6 & !Bitboard::F6, single_push_targets);
        assert_eq!(
            Bitboard::RANK_5 & !Bitboard::E5 & !Bitboard::F5,
            double_push_targets
        );

        let black_pawns = Bitboard::RANK_6;
        let (single_push_targets, double_push_targets) =
            Pawn::push_targets(black_pawns, occupied, Side::Black);
        assert_eq!(Bitboard::RANK_5 & !Bitboard::E5, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);

        let black_pawns = Bitboard::RANK_5;
        let (single_push_targets, double_push_targets) =
            Pawn::push_targets(black_pawns, occupied, Side::Black);
        assert_eq!(Bitboard::RANK_4 & !Bitboard::D4, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);

        let black_pawns = Bitboard::RANK_4;
        let (single_push_targets, double_push_targets) =
            Pawn::push_targets(black_pawns, occupied, Side::Black);
        assert_eq!(Bitboard::RANK_3 & !Bitboard::C3, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);

        let black_pawns = Bitboard::RANK_3;
        let (single_push_targets, double_push_targets) =
            Pawn::push_targets(black_pawns, occupied, Side::Black);
        assert_eq!(Bitboard::RANK_2 & !Bitboard::B2, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);

        let black_pawns = Bitboard::RANK_2;
        let (single_push_targets, double_push_targets) =
            Pawn::push_targets(black_pawns, occupied, Side::Black);
        assert_eq!(Bitboard::RANK_1 & !Bitboard::A1, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);
    }

    #[test]
    fn white_east_attack_targets() {
        let white_pawns = Bitboard::RANK_2;
        assert_eq!(
            Bitboard::RANK_3 & !Bitboard::A3,
            Pawn::east_attack_targets(white_pawns, Side::White)
        );

        let white_pawns = Bitboard::RANK_7;
        assert_eq!(
            Bitboard::RANK_8 & !Bitboard::A8,
            Pawn::east_attack_targets(white_pawns, Side::White)
        );

        let white_pawns = Bitboard::FILE_A & !Bitboard::A1 & !Bitboard::A8;
        assert_eq!(
            Bitboard::FILE_B & !Bitboard::B1 & !Bitboard::B2,
            Pawn::east_attack_targets(white_pawns, Side::White)
        );

        let white_pawns = Bitboard::FILE_H & !Bitboard::H1 & !Bitboard::H8;
        assert_eq!(
            Bitboard::EMPTY,
            Pawn::east_attack_targets(white_pawns, Side::White)
        );
    }

    #[test]
    fn white_west_attack_targets() {
        let white_pawns = Bitboard::RANK_2;
        assert_eq!(
            Bitboard::RANK_3 & !Bitboard::H3,
            Pawn::west_attack_targets(white_pawns, Side::White)
        );

        let white_pawns = Bitboard::RANK_7;
        assert_eq!(
            Bitboard::RANK_8 & !Bitboard::H8,
            Pawn::west_attack_targets(white_pawns, Side::White)
        );

        let white_pawns = Bitboard::FILE_A & !Bitboard::A1 & !Bitboard::A8;
        assert_eq!(
            Bitboard::EMPTY,
            Pawn::west_attack_targets(white_pawns, Side::White)
        );

        let white_pawns = Bitboard::FILE_H & !Bitboard::H1 & !Bitboard::H8;
        assert_eq!(
            Bitboard::FILE_G & !Bitboard::G1 & !Bitboard::G2,
            Pawn::west_attack_targets(white_pawns, Side::White)
        );
    }

    #[test]
    fn black_east_attack_targets() {
        let black_pawns = Bitboard::RANK_2;
        assert_eq!(
            Bitboard::RANK_1 & !Bitboard::A1,
            Pawn::east_attack_targets(black_pawns, Side::Black)
        );

        let black_pawns = Bitboard::RANK_7;
        assert_eq!(
            Bitboard::RANK_6 & !Bitboard::A6,
            Pawn::east_attack_targets(black_pawns, Side::Black)
        );

        let black_pawns = Bitboard::FILE_A & !Bitboard::A1 & !Bitboard::A8;
        assert_eq!(
            Bitboard::FILE_B & !Bitboard::B7 & !Bitboard::B8,
            Pawn::east_attack_targets(black_pawns, Side::Black)
        );

        let black_pawns = Bitboard::FILE_H & !Bitboard::H1 & !Bitboard::H8;
        assert_eq!(
            Bitboard::EMPTY,
            Pawn::east_attack_targets(black_pawns, Side::Black)
        );
    }

    #[test]
    fn black_west_attack_targets() {
        let black_pawns = Bitboard::RANK_2;
        assert_eq!(
            Bitboard::RANK_1 & !Bitboard::H1,
            Pawn::west_attack_targets(black_pawns, Side::Black)
        );

        let black_pawns = Bitboard::RANK_7;
        assert_eq!(
            Bitboard::RANK_6 & !Bitboard::H6,
            Pawn::west_attack_targets(black_pawns, Side::Black)
        );

        let black_pawns = Bitboard::FILE_A & !Bitboard::A1 & !Bitboard::A8;
        assert_eq!(
            Bitboard::EMPTY,
            Pawn::west_attack_targets(black_pawns, Side::Black)
        );

        let black_pawns = Bitboard::FILE_H & !Bitboard::H1 & !Bitboard::H8;
        assert_eq!(
            Bitboard::FILE_G & !Bitboard::G7 & !Bitboard::G8,
            Pawn::west_attack_targets(black_pawns, Side::Black)
        );
    }

    #[test]
    fn white_east_attack_origins() {
        let white_pawns = Bitboard::RANK_3 & !Bitboard::A3;
        assert_eq!(
            Bitboard::RANK_2 & !Bitboard::H2,
            Pawn::east_attack_origins(white_pawns, Side::White)
        );

        let white_pawns = Bitboard::RANK_8 & !Bitboard::A8;
        assert_eq!(
            Bitboard::RANK_7 & !Bitboard::H7,
            Pawn::east_attack_origins(white_pawns, Side::White)
        );

        let white_pawns = Bitboard::FILE_B & !Bitboard::B1 & !Bitboard::B2;
        assert_eq!(
            Bitboard::FILE_A & !Bitboard::A1 & !Bitboard::A8,
            Pawn::east_attack_origins(white_pawns, Side::White)
        );

        let white_pawns = Bitboard::FILE_H & !Bitboard::H1 & !Bitboard::H2;
        assert_eq!(
            Bitboard::FILE_G & !Bitboard::G1 & !Bitboard::G8,
            Pawn::east_attack_origins(white_pawns, Side::White)
        );
    }

    #[test]
    fn white_west_attack_origins() {
        let white_pawns = Bitboard::RANK_3 & !Bitboard::H3;
        assert_eq!(
            Bitboard::RANK_2 & !Bitboard::A2,
            Pawn::west_attack_origins(white_pawns, Side::White)
        );

        let white_pawns = Bitboard::RANK_8 & !Bitboard::H8;
        assert_eq!(
            Bitboard::RANK_7 & !Bitboard::A7,
            Pawn::west_attack_origins(white_pawns, Side::White)
        );

        let white_pawns = Bitboard::FILE_A & !Bitboard::A1 & !Bitboard::A2;
        assert_eq!(
            Bitboard::FILE_B & !Bitboard::B1 & !Bitboard::B8,
            Pawn::west_attack_origins(white_pawns, Side::White)
        );

        let white_pawns = Bitboard::FILE_G & !Bitboard::G1 & !Bitboard::G2;
        assert_eq!(
            Bitboard::FILE_H & !Bitboard::H1 & !Bitboard::H8,
            Pawn::west_attack_origins(white_pawns, Side::White)
        );
    }

    #[test]
    fn black_east_attack_origins() {
        let black_pawns = Bitboard::RANK_1 & !Bitboard::A1;
        assert_eq!(
            Bitboard::RANK_2 & !Bitboard::H2,
            Pawn::east_attack_origins(black_pawns, Side::Black)
        );

        let black_pawns = Bitboard::RANK_6 & !Bitboard::A6;
        assert_eq!(
            Bitboard::RANK_7 & !Bitboard::H7,
            Pawn::east_attack_origins(black_pawns, Side::Black)
        );

        let black_pawns = Bitboard::FILE_B & !Bitboard::B7 & !Bitboard::B8;
        assert_eq!(
            Bitboard::FILE_A & !Bitboard::A1 & !Bitboard::A8,
            Pawn::east_attack_origins(black_pawns, Side::Black)
        );

        let black_pawns = Bitboard::FILE_H & !Bitboard::H7 & !Bitboard::H8;
        assert_eq!(
            Bitboard::FILE_G & !Bitboard::G1 & !Bitboard::G8,
            Pawn::east_attack_origins(black_pawns, Side::Black)
        );
    }

    #[test]
    fn black_west_attack_origins() {
        let black_pawns = Bitboard::RANK_1 & !Bitboard::H1;
        assert_eq!(
            Bitboard::RANK_2 & !Bitboard::A2,
            Pawn::west_attack_origins(black_pawns, Side::Black)
        );

        let black_pawns = Bitboard::RANK_6 & !Bitboard::H6;
        assert_eq!(
            Bitboard::RANK_7 & !Bitboard::A7,
            Pawn::west_attack_origins(black_pawns, Side::Black)
        );

        let black_pawns = Bitboard::FILE_A & !Bitboard::A7 & !Bitboard::A8;
        assert_eq!(
            Bitboard::FILE_B & !Bitboard::B1 & !Bitboard::B8,
            Pawn::west_attack_origins(black_pawns, Side::Black)
        );

        let black_pawns = Bitboard::FILE_G & !Bitboard::G7 & !Bitboard::G8;
        assert_eq!(
            Bitboard::FILE_H & !Bitboard::H1 & !Bitboard::H8,
            Pawn::west_attack_origins(black_pawns, Side::Black)
        );
    }

    #[test]
    fn single_push_origins() {
        assert_eq!(
            Bitboard::D3 | Bitboard::E4,
            Pawn::single_push_origins(Bitboard::D4 | Bitboard::E5, Side::White)
        );
        assert_eq!(
            Bitboard::D6 | Bitboard::E5,
            Pawn::single_push_origins(Bitboard::D5 | Bitboard::E4, Side::Black)
        );
    }

    #[test]
    fn push_origin() {
        assert_eq!(Square::D3, Pawn::push_origin(Square::D4, Side::White));
        assert_eq!(Square::D6, Pawn::push_origin(Square::D5, Side::Black));
    }

    #[test]
    fn double_push_origin() {
        assert_eq!(
            Square::D2,
            Pawn::double_push_origin(Square::D4, Side::White)
        );
        assert_eq!(
            Square::D7,
            Pawn::double_push_origin(Square::D5, Side::Black)
        );
    }

    #[test]
    fn east_attack_origin() {
        assert_eq!(
            Square::C3,
            Pawn::east_attack_origin(Square::D4, Side::White)
        );
        assert_eq!(
            Square::C6,
            Pawn::east_attack_origin(Square::D5, Side::Black)
        );
    }

    #[test]
    fn west_attack_origin() {
        assert_eq!(
            Square::E3,
            Pawn::west_attack_origin(Square::D4, Side::White)
        );
        assert_eq!(
            Square::E6,
            Pawn::west_attack_origin(Square::D5, Side::Black)
        );
    }

    #[test]
    fn promotion_rank() {
        assert_eq!(Bitboard::RANK_8, Pawn::promotion_rank(Side::White));
        assert_eq!(Bitboard::RANK_1, Pawn::promotion_rank(Side::Black));
    }

    #[test]
    fn front_attack_spans() {
        let pawn = Bitboard::D4 | Bitboard::E5;
        let exp_white_east = Bitboard::E5
            | Bitboard::E6
            | Bitboard::E7
            | Bitboard::E8
            | Bitboard::F6
            | Bitboard::F7
            | Bitboard::F8;
        let exp_white_west = Bitboard::C5
            | Bitboard::C6
            | Bitboard::C7
            | Bitboard::C8
            | Bitboard::D6
            | Bitboard::D7
            | Bitboard::D8;
        let exp_black_east = Bitboard::E3
            | Bitboard::E2
            | Bitboard::E1
            | Bitboard::F4
            | Bitboard::F3
            | Bitboard::F2
            | Bitboard::F1;
        let exp_black_west = Bitboard::C3
            | Bitboard::C2
            | Bitboard::C1
            | Bitboard::D4
            | Bitboard::D3
            | Bitboard::D2
            | Bitboard::D1;
        assert_eq!(
            exp_white_east,
            Pawn::east_front_attack_span(pawn, Side::White)
        );
        assert_eq!(
            exp_white_west,
            Pawn::west_front_attack_span(pawn, Side::White)
        );
        assert_eq!(
            exp_white_east | exp_white_west,
            Pawn::front_attack_span(pawn, Side::White)
        );
        assert_eq!(
            exp_black_east,
            Pawn::east_front_attack_span(pawn, Side::Black)
        );
        assert_eq!(
            exp_black_west,
            Pawn::west_front_attack_span(pawn, Side::Black)
        );
        assert_eq!(
            exp_black_east | exp_black_west,
            Pawn::front_attack_span(pawn, Side::Black)
        );
    }
}
