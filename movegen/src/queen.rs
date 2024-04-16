use crate::bitboard::Bitboard;
use crate::ray::Ray;
use crate::square::Square;

pub struct Queen;

impl Queen {
    pub fn targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Ray::file_targets(origin, occupied)
            | Ray::rank_targets(origin, occupied)
            | Ray::diagonal_targets(origin, occupied)
            | Ray::anti_diagonal_targets(origin, occupied)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn targets() {
        assert_eq!(
            Bitboard::C4
                | Bitboard::D3
                | Bitboard::D2
                | Bitboard::D5
                | Bitboard::D6
                | Bitboard::E4
                | Bitboard::F4
                | Bitboard::G4
                | Bitboard::C3
                | Bitboard::E3
                | Bitboard::F2
                | Bitboard::C5
                | Bitboard::B6
                | Bitboard::E5
                | Bitboard::F6
                | Bitboard::G7,
            Queen::targets(
                Square::D4,
                Bitboard::C4
                    | Bitboard::B4
                    | Bitboard::A4
                    | Bitboard::D2
                    | Bitboard::D6
                    | Bitboard::G4
                    | Bitboard::C3
                    | Bitboard::B2
                    | Bitboard::A1
                    | Bitboard::F2
                    | Bitboard::B6
                    | Bitboard::G7
            )
        );
    }

    #[test]
    fn non_blocking_occupancy_targets() {
        assert_eq!(
            Queen::targets(Square::D4, Bitboard::EMPTY),
            Queen::targets(
                Square::D4,
                Bitboard::B3
                    | Bitboard::B5
                    | Bitboard::C2
                    | Bitboard::C6
                    | Bitboard::E2
                    | Bitboard::E6
                    | Bitboard::F3
                    | Bitboard::F5
            )
        );
    }
}
