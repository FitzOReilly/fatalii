use crate::bitboard::Bitboard;

pub struct Knight;

impl Knight {
    fn targets(origin: usize) -> Bitboard {
        Self::TARGETS_ALL_SQUARES[origin]
    }

    const TARGETS_ALL_SQUARES: [Bitboard; Bitboard::NUM_SQUARES] = [
        Bitboard(0x20400),
        Bitboard(0x50800),
        Bitboard(0xa1100),
        Bitboard(0x142200),
        Bitboard(0x284400),
        Bitboard(0x508800),
        Bitboard(0xa01000),
        Bitboard(0x402000),
        Bitboard(0x2040004),
        Bitboard(0x5080008),
        Bitboard(0xa110011),
        Bitboard(0x14220022),
        Bitboard(0x28440044),
        Bitboard(0x50880088),
        Bitboard(0xa0100010),
        Bitboard(0x40200020),
        Bitboard(0x204000402),
        Bitboard(0x508000805),
        Bitboard(0xa1100110a),
        Bitboard(0x1422002214),
        Bitboard(0x2844004428),
        Bitboard(0x5088008850),
        Bitboard(0xa0100010a0),
        Bitboard(0x4020002040),
        Bitboard(0x20400040200),
        Bitboard(0x50800080500),
        Bitboard(0xa1100110a00),
        Bitboard(0x142200221400),
        Bitboard(0x284400442800),
        Bitboard(0x508800885000),
        Bitboard(0xa0100010a000),
        Bitboard(0x402000204000),
        Bitboard(0x2040004020000),
        Bitboard(0x5080008050000),
        Bitboard(0xa1100110a0000),
        Bitboard(0x14220022140000),
        Bitboard(0x28440044280000),
        Bitboard(0x50880088500000),
        Bitboard(0xa0100010a00000),
        Bitboard(0x40200020400000),
        Bitboard(0x204000402000000),
        Bitboard(0x508000805000000),
        Bitboard(0xa1100110a000000),
        Bitboard(0x1422002214000000),
        Bitboard(0x2844004428000000),
        Bitboard(0x5088008850000000),
        Bitboard(0xa0100010a0000000),
        Bitboard(0x4020002040000000),
        Bitboard(0x400040200000000),
        Bitboard(0x800080500000000),
        Bitboard(0x1100110a00000000),
        Bitboard(0x2200221400000000),
        Bitboard(0x4400442800000000),
        Bitboard(0x8800885000000000),
        Bitboard(0x100010a000000000),
        Bitboard(0x2000204000000000),
        Bitboard(0x4020000000000),
        Bitboard(0x8050000000000),
        Bitboard(0x110a0000000000),
        Bitboard(0x22140000000000),
        Bitboard(0x44280000000000),
        Bitboard(0x88500000000000),
        Bitboard(0x10a00000000000),
        Bitboard(0x20400000000000),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn targets() {
        assert_eq!(
            Bitboard::B3 | Bitboard::C2,
            Knight::targets(Bitboard::IDX_A1)
        );
        assert_eq!(
            Bitboard::B6 | Bitboard::C7,
            Knight::targets(Bitboard::IDX_A8)
        );
        assert_eq!(
            Bitboard::G3 | Bitboard::F2,
            Knight::targets(Bitboard::IDX_H1)
        );
        assert_eq!(
            Bitboard::G6 | Bitboard::F7,
            Knight::targets(Bitboard::IDX_H8)
        );

        assert_eq!(
            Bitboard::A4 | Bitboard::C4 | Bitboard::D3 | Bitboard::D1,
            Knight::targets(Bitboard::IDX_B2)
        );
        assert_eq!(
            Bitboard::A5 | Bitboard::C5 | Bitboard::D6 | Bitboard::D8,
            Knight::targets(Bitboard::IDX_B7)
        );
        assert_eq!(
            Bitboard::H4 | Bitboard::F4 | Bitboard::E3 | Bitboard::E1,
            Knight::targets(Bitboard::IDX_G2)
        );
        assert_eq!(
            Bitboard::H5 | Bitboard::F5 | Bitboard::E6 | Bitboard::E8,
            Knight::targets(Bitboard::IDX_G7)
        );

        assert_eq!(
            Bitboard::A2
                | Bitboard::A4
                | Bitboard::B1
                | Bitboard::B5
                | Bitboard::D1
                | Bitboard::D5
                | Bitboard::E2
                | Bitboard::E4,
            Knight::targets(Bitboard::IDX_C3)
        );
        assert_eq!(
            Bitboard::A5
                | Bitboard::A7
                | Bitboard::B4
                | Bitboard::B8
                | Bitboard::D4
                | Bitboard::D8
                | Bitboard::E5
                | Bitboard::E7,
            Knight::targets(Bitboard::IDX_C6)
        );
        assert_eq!(
            Bitboard::D2
                | Bitboard::D4
                | Bitboard::E1
                | Bitboard::E5
                | Bitboard::G1
                | Bitboard::G5
                | Bitboard::H2
                | Bitboard::H4,
            Knight::targets(Bitboard::IDX_F3)
        );
        assert_eq!(
            Bitboard::D5
                | Bitboard::D7
                | Bitboard::E4
                | Bitboard::E8
                | Bitboard::G4
                | Bitboard::G8
                | Bitboard::H5
                | Bitboard::H7,
            Knight::targets(Bitboard::IDX_F6)
        );
    }
}
