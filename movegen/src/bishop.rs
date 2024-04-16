use crate::bitboard::Bitboard;
use crate::ray::Ray;
use crate::square::Square;

pub struct Bishop;

impl Bishop {
    pub fn targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Ray::diagonal_targets(origin, occupied) | Ray::anti_diagonal_targets(origin, occupied)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagonal_targets() {
        assert_eq!(
            Bitboard::C3
                | Bitboard::E3
                | Bitboard::F2
                | Bitboard::C5
                | Bitboard::B6
                | Bitboard::E5
                | Bitboard::F6
                | Bitboard::G7,
            Bishop::targets(
                Square::D4,
                Bitboard::C3
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
            Bishop::targets(Square::D4, Bitboard::EMPTY),
            Bishop::targets(
                Square::D4,
                Bitboard::C4 | Bitboard::D3 | Bitboard::D5 | Bitboard::E4
            )
        );
    }
}
