use crate::bitboard::Bitboard;
use crate::direction::Direction;
use crate::ray;

pub struct Bishop;

impl Bishop {
    fn targets(origin: usize, occupied: Bitboard) -> Bitboard {
        Self::positive_ray_targets(origin, occupied, Direction::NorthEast)
            | Self::positive_ray_targets(origin, occupied, Direction::SouthEast)
            | Self::negative_ray_targets(origin, occupied, Direction::SouthWest)
            | Self::negative_ray_targets(origin, occupied, Direction::NorthWest)
    }

    fn positive_ray_targets(origin: usize, occupied: Bitboard, direction: Direction) -> Bitboard {
        let empty_board_targets = ray::RAYS[direction as usize][origin];
        let blocked = empty_board_targets & occupied;
        // At least one bit must be set when calling bit_scan_forward. Setting the most significant
        // bit does not change the targets and allows a branchless implementation.
        let first_blocked = (blocked | Bitboard(0x8000000000000000)).bit_scan_forward();
        let targets = empty_board_targets ^ ray::RAYS[direction as usize][first_blocked];
        targets
    }

    fn negative_ray_targets(origin: usize, occupied: Bitboard, direction: Direction) -> Bitboard {
        let empty_board_targets = ray::RAYS[direction as usize][origin];
        let blocked = empty_board_targets & occupied;
        // At least one bit must be set when calling bit_scan_reverse. Setting the least
        // significant bit does not change the targets and allows a branchless implementation.
        let first_blocked = (blocked | Bitboard(0x0000000000000001)).bit_scan_reverse();
        let targets = empty_board_targets ^ ray::RAYS[direction as usize][first_blocked];
        targets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn targets_north_east() {
        assert_eq!(
            Bitboard::B2
                | Bitboard::C3
                | Bitboard::D4
                | Bitboard::E5
                | Bitboard::F6
                | Bitboard::G7
                | Bitboard::H8,
            Bishop::targets(Bitboard::IDX_A1, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::B2 | Bitboard::C3 | Bitboard::D4 | Bitboard::E5 | Bitboard::F6 | Bitboard::G7,
            Bishop::targets(Bitboard::IDX_A1, Bitboard::G7)
        );

        assert_eq!(
            Bitboard::B2,
            Bishop::targets(Bitboard::IDX_A1, Bitboard::B2 | Bitboard::G7)
        );
    }

    #[test]
    fn targets_south_east() {
        assert_eq!(
            Bitboard::B7
                | Bitboard::C6
                | Bitboard::D5
                | Bitboard::E4
                | Bitboard::F3
                | Bitboard::G2
                | Bitboard::H1,
            Bishop::targets(Bitboard::IDX_A8, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::B7 | Bitboard::C6 | Bitboard::D5 | Bitboard::E4 | Bitboard::F3 | Bitboard::G2,
            Bishop::targets(Bitboard::IDX_A8, Bitboard::G2)
        );

        assert_eq!(
            Bitboard::B7,
            Bishop::targets(Bitboard::IDX_A8, Bitboard::B7 | Bitboard::G2)
        );
    }

    #[test]
    fn targets_south_west() {
        assert_eq!(
            Bitboard::G7
                | Bitboard::F6
                | Bitboard::E5
                | Bitboard::D4
                | Bitboard::C3
                | Bitboard::B2
                | Bitboard::A1,
            Bishop::targets(Bitboard::IDX_H8, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::G7 | Bitboard::F6 | Bitboard::E5 | Bitboard::D4 | Bitboard::C3 | Bitboard::B2,
            Bishop::targets(Bitboard::IDX_H8, Bitboard::B2)
        );

        assert_eq!(
            Bitboard::G7,
            Bishop::targets(Bitboard::IDX_H8, Bitboard::G7 | Bitboard::B2)
        );
    }

    #[test]
    fn targets_north_west() {
        assert_eq!(
            Bitboard::G2
                | Bitboard::F3
                | Bitboard::E4
                | Bitboard::D5
                | Bitboard::C6
                | Bitboard::B7
                | Bitboard::A8,
            Bishop::targets(Bitboard::IDX_H1, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::G2 | Bitboard::F3 | Bitboard::E4 | Bitboard::D5 | Bitboard::C6 | Bitboard::B7,
            Bishop::targets(Bitboard::IDX_H1, Bitboard::B7)
        );

        assert_eq!(
            Bitboard::G2,
            Bishop::targets(Bitboard::IDX_H1, Bitboard::G2 | Bitboard::B7)
        );
    }

    #[test]
    fn targets_multiple_directions() {
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
                Bitboard::IDX_D4,
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
    fn targets_non_blocking_occupancy() {
        assert_eq!(
            Bishop::targets(Bitboard::IDX_D4, Bitboard::EMPTY),
            Bishop::targets(
                Bitboard::IDX_D4,
                Bitboard::C4 | Bitboard::D3 | Bitboard::D5 | Bitboard::E4
            )
        );
    }
}
