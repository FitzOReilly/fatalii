use crate::bitboard::Bitboard;
use crate::side::Side;

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
        [
            Self::white_east_attack_targets,
            Self::black_east_attack_targets,
        ][side_to_move as usize](pawns)
    }

    pub fn west_attack_targets(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        [
            Self::white_west_attack_targets,
            Self::black_west_attack_targets,
        ][side_to_move as usize](pawns)
    }

    pub fn attack_targets(pawns: Bitboard, side_to_move: Side) -> Bitboard {
        Self::east_attack_targets(pawns, side_to_move)
            | Self::west_attack_targets(pawns, side_to_move)
    }

    pub fn idx_push_origin(target: usize, side_to_move: Side) -> usize {
        [Bitboard::idx_south_one, Bitboard::idx_north_one][side_to_move as usize](target)
    }

    pub fn idx_double_push_origin(target: usize, side_to_move: Side) -> usize {
        [Bitboard::idx_south_two, Bitboard::idx_north_two][side_to_move as usize](target)
    }

    pub fn idx_east_attack_origin(target: usize, side_to_move: Side) -> usize {
        [Bitboard::idx_south_west_one, Bitboard::idx_north_west_one][side_to_move as usize](target)
    }

    pub fn idx_west_attack_origin(target: usize, side_to_move: Side) -> usize {
        [Bitboard::idx_south_east_one, Bitboard::idx_north_east_one][side_to_move as usize](target)
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

    fn white_east_attack_targets(white_pawns: Bitboard) -> Bitboard {
        white_pawns.north_east_one_if_rank_8_empty()
    }

    fn white_west_attack_targets(white_pawns: Bitboard) -> Bitboard {
        white_pawns.north_west_one_if_rank_8_empty()
    }

    fn black_east_attack_targets(black_pawns: Bitboard) -> Bitboard {
        black_pawns.south_east_one_if_rank_1_empty()
    }

    fn black_west_attack_targets(black_pawns: Bitboard) -> Bitboard {
        black_pawns.south_west_one_if_rank_1_empty()
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
    fn idx_push_origin() {
        assert_eq!(
            Bitboard::IDX_D3,
            Pawn::idx_push_origin(Bitboard::IDX_D4, Side::White)
        );
        assert_eq!(
            Bitboard::IDX_D6,
            Pawn::idx_push_origin(Bitboard::IDX_D5, Side::Black)
        );
    }

    #[test]
    fn idx_double_push_origin() {
        assert_eq!(
            Bitboard::IDX_D2,
            Pawn::idx_double_push_origin(Bitboard::IDX_D4, Side::White)
        );
        assert_eq!(
            Bitboard::IDX_D7,
            Pawn::idx_double_push_origin(Bitboard::IDX_D5, Side::Black)
        );
    }

    #[test]
    fn idx_east_attack_origin() {
        assert_eq!(
            Bitboard::IDX_C3,
            Pawn::idx_east_attack_origin(Bitboard::IDX_D4, Side::White)
        );
        assert_eq!(
            Bitboard::IDX_C6,
            Pawn::idx_east_attack_origin(Bitboard::IDX_D5, Side::Black)
        );
    }

    #[test]
    fn idx_west_attack_origin() {
        assert_eq!(
            Bitboard::IDX_E3,
            Pawn::idx_west_attack_origin(Bitboard::IDX_D4, Side::White)
        );
        assert_eq!(
            Bitboard::IDX_E6,
            Pawn::idx_west_attack_origin(Bitboard::IDX_D5, Side::Black)
        );
    }

    #[test]
    fn promotion_rank() {
        assert_eq!(Bitboard::RANK_8, Pawn::promotion_rank(Side::White));
        assert_eq!(Bitboard::RANK_1, Pawn::promotion_rank(Side::Black));
    }
}
