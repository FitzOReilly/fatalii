use crate::bitboard::Bitboard;

pub struct Pawn;

impl Pawn {
    fn white_push_targets(white_pawns: Bitboard, empty: Bitboard) -> (Bitboard, Bitboard) {
        let single_push_targets = white_pawns.north_one_if_rank_8_empty() & empty;
        let double_push_targets =
            single_push_targets.north_one_if_rank_8_empty() & empty & Bitboard::RANK_4;
        (single_push_targets, double_push_targets)
    }

    fn black_push_targets(black_pawns: Bitboard, empty: Bitboard) -> (Bitboard, Bitboard) {
        let single_push_targets = black_pawns.south_one_if_rank_1_empty() & empty;
        let double_push_targets =
            single_push_targets.south_one_if_rank_1_empty() & empty & Bitboard::RANK_5;
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
        let white_pawns = Bitboard::RANK_2;
        let empty = !Bitboard::RANK_1 & !Bitboard::RANK_2 & !Bitboard::RANK_7 & !Bitboard::RANK_8;
        let (single_push_targets, double_push_targets) =
            Pawn::white_push_targets(white_pawns, empty);
        assert_eq!(Bitboard::RANK_3, single_push_targets);
        assert_eq!(Bitboard::RANK_4, double_push_targets);

        let white_pawns = Bitboard::RANK_3;
        let (single_push_targets, double_push_targets) =
            Pawn::white_push_targets(white_pawns, empty);
        assert_eq!(Bitboard::RANK_4, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);

        let white_pawns = Bitboard::RANK_5;
        let (single_push_targets, double_push_targets) =
            Pawn::white_push_targets(white_pawns, empty);
        assert_eq!(Bitboard::RANK_6, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);

        let white_pawns = Bitboard::RANK_6;
        let (single_push_targets, double_push_targets) =
            Pawn::white_push_targets(white_pawns, empty);
        assert_eq!(Bitboard::EMPTY, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);

        let white_pawns = Bitboard::A2 | Bitboard::B3 | Bitboard::C4 | Bitboard::D5 | Bitboard::E6;
        let expected_single_push_targets =
            Bitboard::A3 | Bitboard::B4 | Bitboard::C5 | Bitboard::D6;
        let expected_double_push_targets = Bitboard::A4;
        let (single_push_targets, double_push_targets) =
            Pawn::white_push_targets(white_pawns, empty);
        assert_eq!(expected_single_push_targets, single_push_targets);
        assert_eq!(expected_double_push_targets, double_push_targets);
    }

    #[test]
    fn black_push_targets() {
        let black_pawns = Bitboard::RANK_7;
        let empty = !Bitboard::RANK_1 & !Bitboard::RANK_2 & !Bitboard::RANK_7 & !Bitboard::RANK_8;
        let (single_push_targets, double_push_targets) =
            Pawn::black_push_targets(black_pawns, empty);
        assert_eq!(Bitboard::RANK_6, single_push_targets);
        assert_eq!(Bitboard::RANK_5, double_push_targets);

        let black_pawns = Bitboard::RANK_6;
        let (single_push_targets, double_push_targets) =
            Pawn::black_push_targets(black_pawns, empty);
        assert_eq!(Bitboard::RANK_5, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);

        let black_pawns = Bitboard::RANK_4;
        let (single_push_targets, double_push_targets) =
            Pawn::black_push_targets(black_pawns, empty);
        assert_eq!(Bitboard::RANK_3, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);

        let black_pawns = Bitboard::RANK_3;
        let (single_push_targets, double_push_targets) =
            Pawn::black_push_targets(black_pawns, empty);
        assert_eq!(Bitboard::EMPTY, single_push_targets);
        assert_eq!(Bitboard::EMPTY, double_push_targets);

        let black_pawns = Bitboard::A7 | Bitboard::B6 | Bitboard::C5 | Bitboard::D4 | Bitboard::E3;
        let expected_single_push_targets =
            Bitboard::A6 | Bitboard::B5 | Bitboard::C4 | Bitboard::D3;
        let expected_double_push_targets = Bitboard::A5;
        let (single_push_targets, double_push_targets) =
            Pawn::black_push_targets(black_pawns, empty);
        assert_eq!(expected_single_push_targets, single_push_targets);
        assert_eq!(expected_double_push_targets, double_push_targets);
    }

    #[test]
    fn white_east_attack_targets() {
        let white_pawns = Bitboard::RANK_2;
        assert_eq!(
            Bitboard::RANK_3 & !Bitboard::A3,
            Pawn::white_east_attack_targets(white_pawns)
        );

        let white_pawns = Bitboard::RANK_7;
        assert_eq!(
            Bitboard::RANK_8 & !Bitboard::A8,
            Pawn::white_east_attack_targets(white_pawns)
        );

        let white_pawns = Bitboard::FILE_A & !Bitboard::A1 & !Bitboard::A8;
        assert_eq!(
            Bitboard::FILE_B & !Bitboard::B1 & !Bitboard::B2,
            Pawn::white_east_attack_targets(white_pawns)
        );

        let white_pawns = Bitboard::FILE_H & !Bitboard::H1 & !Bitboard::H8;
        assert_eq!(
            Bitboard::EMPTY,
            Pawn::white_east_attack_targets(white_pawns)
        );
    }

    #[test]
    fn white_west_attack_targets() {
        let white_pawns = Bitboard::RANK_2;
        assert_eq!(
            Bitboard::RANK_3 & !Bitboard::H3,
            Pawn::white_west_attack_targets(white_pawns)
        );

        let white_pawns = Bitboard::RANK_7;
        assert_eq!(
            Bitboard::RANK_8 & !Bitboard::H8,
            Pawn::white_west_attack_targets(white_pawns)
        );

        let white_pawns = Bitboard::FILE_A & !Bitboard::A1 & !Bitboard::A8;
        assert_eq!(
            Bitboard::EMPTY,
            Pawn::white_west_attack_targets(white_pawns)
        );

        let white_pawns = Bitboard::FILE_H & !Bitboard::H1 & !Bitboard::H8;
        assert_eq!(
            Bitboard::FILE_G & !Bitboard::G1 & !Bitboard::G2,
            Pawn::white_west_attack_targets(white_pawns)
        );
    }

    #[test]
    fn black_east_attack_targets() {
        let black_pawns = Bitboard::RANK_2;
        assert_eq!(
            Bitboard::RANK_1 & !Bitboard::A1,
            Pawn::black_east_attack_targets(black_pawns)
        );

        let black_pawns = Bitboard::RANK_7;
        assert_eq!(
            Bitboard::RANK_6 & !Bitboard::A6,
            Pawn::black_east_attack_targets(black_pawns)
        );

        let black_pawns = Bitboard::FILE_A & !Bitboard::A1 & !Bitboard::A8;
        assert_eq!(
            Bitboard::FILE_B & !Bitboard::B7 & !Bitboard::B8,
            Pawn::black_east_attack_targets(black_pawns)
        );

        let black_pawns = Bitboard::FILE_H & !Bitboard::H1 & !Bitboard::H8;
        assert_eq!(
            Bitboard::EMPTY,
            Pawn::black_east_attack_targets(black_pawns)
        );
    }

    #[test]
    fn black_west_attack_targets() {
        let black_pawns = Bitboard::RANK_2;
        assert_eq!(
            Bitboard::RANK_1 & !Bitboard::H1,
            Pawn::black_west_attack_targets(black_pawns)
        );

        let black_pawns = Bitboard::RANK_7;
        assert_eq!(
            Bitboard::RANK_6 & !Bitboard::H6,
            Pawn::black_west_attack_targets(black_pawns)
        );

        let black_pawns = Bitboard::FILE_A & !Bitboard::A1 & !Bitboard::A8;
        assert_eq!(
            Bitboard::EMPTY,
            Pawn::black_west_attack_targets(black_pawns)
        );

        let black_pawns = Bitboard::FILE_H & !Bitboard::H1 & !Bitboard::H8;
        assert_eq!(
            Bitboard::FILE_G & !Bitboard::G7 & !Bitboard::G8,
            Pawn::black_west_attack_targets(black_pawns)
        );
    }
}
