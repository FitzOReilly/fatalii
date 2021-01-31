use crate::bitboard::Bitboard;
use crate::ray::Ray;

pub struct Rook;

impl Rook {
    pub fn targets(origin: usize, occupied: Bitboard) -> Bitboard {
        Ray::north_targets(origin, occupied)
            | Ray::east_targets(origin, occupied)
            | Ray::south_targets(origin, occupied)
            | Ray::west_targets(origin, occupied)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_and_file_targets() {
        assert_eq!(
            Bitboard::C4
                | Bitboard::D3
                | Bitboard::D2
                | Bitboard::D5
                | Bitboard::D6
                | Bitboard::E4
                | Bitboard::F4
                | Bitboard::G4,
            Rook::targets(
                Bitboard::IDX_D4,
                Bitboard::C4
                    | Bitboard::B4
                    | Bitboard::A4
                    | Bitboard::D2
                    | Bitboard::D6
                    | Bitboard::G4
            )
        );
    }

    #[test]
    fn non_blocking_occupancy_targets() {
        assert_eq!(
            Rook::targets(Bitboard::IDX_D4, Bitboard::EMPTY),
            Rook::targets(
                Bitboard::IDX_D4,
                Bitboard::C3 | Bitboard::C5 | Bitboard::E3 | Bitboard::E5
            )
        );
    }
}
