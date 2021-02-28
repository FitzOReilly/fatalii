use crate::bitboard::Bitboard;
use crate::square::Square;

pub struct King;

impl King {
    pub fn targets(origin: Square) -> Bitboard {
        Self::TARGETS_ALL_SQUARES[origin.idx()]
    }

    const TARGETS_ALL_SQUARES: [Bitboard; Square::NUM_SQUARES] = [
        Bitboard(0x302),
        Bitboard(0x705),
        Bitboard(0xe0a),
        Bitboard(0x1c14),
        Bitboard(0x3828),
        Bitboard(0x7050),
        Bitboard(0xe0a0),
        Bitboard(0xc040),
        Bitboard(0x30203),
        Bitboard(0x70507),
        Bitboard(0xe0a0e),
        Bitboard(0x1c141c),
        Bitboard(0x382838),
        Bitboard(0x705070),
        Bitboard(0xe0a0e0),
        Bitboard(0xc040c0),
        Bitboard(0x3020300),
        Bitboard(0x7050700),
        Bitboard(0xe0a0e00),
        Bitboard(0x1c141c00),
        Bitboard(0x38283800),
        Bitboard(0x70507000),
        Bitboard(0xe0a0e000),
        Bitboard(0xc040c000),
        Bitboard(0x302030000),
        Bitboard(0x705070000),
        Bitboard(0xe0a0e0000),
        Bitboard(0x1c141c0000),
        Bitboard(0x3828380000),
        Bitboard(0x7050700000),
        Bitboard(0xe0a0e00000),
        Bitboard(0xc040c00000),
        Bitboard(0x30203000000),
        Bitboard(0x70507000000),
        Bitboard(0xe0a0e000000),
        Bitboard(0x1c141c000000),
        Bitboard(0x382838000000),
        Bitboard(0x705070000000),
        Bitboard(0xe0a0e0000000),
        Bitboard(0xc040c0000000),
        Bitboard(0x3020300000000),
        Bitboard(0x7050700000000),
        Bitboard(0xe0a0e00000000),
        Bitboard(0x1c141c00000000),
        Bitboard(0x38283800000000),
        Bitboard(0x70507000000000),
        Bitboard(0xe0a0e000000000),
        Bitboard(0xc040c000000000),
        Bitboard(0x302030000000000),
        Bitboard(0x705070000000000),
        Bitboard(0xe0a0e0000000000),
        Bitboard(0x1c141c0000000000),
        Bitboard(0x3828380000000000),
        Bitboard(0x7050700000000000),
        Bitboard(0xe0a0e00000000000),
        Bitboard(0xc040c00000000000),
        Bitboard(0x203000000000000),
        Bitboard(0x507000000000000),
        Bitboard(0xa0e000000000000),
        Bitboard(0x141c000000000000),
        Bitboard(0x2838000000000000),
        Bitboard(0x5070000000000000),
        Bitboard(0xa0e0000000000000),
        Bitboard(0x40c0000000000000),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn targets() {
        assert_eq!(
            Bitboard::A2 | Bitboard::B1 | Bitboard::B2,
            King::targets(Square::A1)
        );
        assert_eq!(
            Bitboard::A7 | Bitboard::B8 | Bitboard::B7,
            King::targets(Square::A8)
        );
        assert_eq!(
            Bitboard::H2 | Bitboard::G1 | Bitboard::G2,
            King::targets(Square::H1)
        );
        assert_eq!(
            Bitboard::H7 | Bitboard::G8 | Bitboard::G7,
            King::targets(Square::H8)
        );

        assert_eq!(
            Bitboard::A1 | Bitboard::A2 | Bitboard::B2 | Bitboard::C1 | Bitboard::C2,
            King::targets(Square::B1)
        );

        assert_eq!(
            Bitboard::A1
                | Bitboard::A2
                | Bitboard::A3
                | Bitboard::B1
                | Bitboard::B3
                | Bitboard::C1
                | Bitboard::C2
                | Bitboard::C3,
            King::targets(Square::B2)
        );
    }
}
