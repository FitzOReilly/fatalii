use crate::bitboard::Bitboard;

pub struct Knight;

impl Knight {
    fn targets(origin: Bitboard) -> Bitboard {
        origin.north_two_east_one()
            | origin.north_one_east_two()
            | origin.south_one_east_two()
            | origin.south_two_east_one()
            | origin.south_two_west_one()
            | origin.south_one_west_two()
            | origin.north_one_west_two()
            | origin.north_two_west_one()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn targets() {
        assert_eq!(Bitboard::B3 | Bitboard::C2, Knight::targets(Bitboard::A1));
        assert_eq!(Bitboard::B6 | Bitboard::C7, Knight::targets(Bitboard::A8));
        assert_eq!(Bitboard::G3 | Bitboard::F2, Knight::targets(Bitboard::H1));
        assert_eq!(Bitboard::G6 | Bitboard::F7, Knight::targets(Bitboard::H8));

        assert_eq!(
            Bitboard::A4 | Bitboard::C4 | Bitboard::D3 | Bitboard::D1,
            Knight::targets(Bitboard::B2)
        );
        assert_eq!(
            Bitboard::A5 | Bitboard::C5 | Bitboard::D6 | Bitboard::D8,
            Knight::targets(Bitboard::B7)
        );
        assert_eq!(
            Bitboard::H4 | Bitboard::F4 | Bitboard::E3 | Bitboard::E1,
            Knight::targets(Bitboard::G2)
        );
        assert_eq!(
            Bitboard::H5 | Bitboard::F5 | Bitboard::E6 | Bitboard::E8,
            Knight::targets(Bitboard::G7)
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
            Knight::targets(Bitboard::C3)
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
            Knight::targets(Bitboard::C6)
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
            Knight::targets(Bitboard::F3)
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
            Knight::targets(Bitboard::F6)
        );
    }
}
