use crate::bitboard::Bitboard;
use crate::direction::Direction;
use crate::ray_lookup_tables::{
    ANTI_DIAGONAL_RAYS, DIAGONAL_RAYS, DIRECTION_RAYS, FILE_A_TARGETS, FILE_RAYS, RANK_RAYS,
};
use crate::square::Square;

pub struct Ray;

impl Ray {
    pub fn file_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        let file_mask = FILE_RAYS[origin.idx()];
        let occupancy_idx = ((file_mask & occupied) >> (origin.file().idx() * 8 + 1)).0 & 0x3f;
        FILE_A_TARGETS[origin.rank().idx()][occupancy_idx as usize] & file_mask
    }

    pub fn rank_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        const B3_TO_G8_DIAGONAL: Bitboard = Bitboard(0x0080402010080400);
        const A1_TO_H8_DIAGONAL: Bitboard = Bitboard(0x8040201008040201);
        let rank_mask = RANK_RAYS[origin.idx()];
        let rank_idx = origin.rank().idx();
        // Shift the relevant rank to rank 1
        let rank_1_occupancy = (rank_mask & occupied) >> rank_idx & Bitboard::RANK_1;
        // Move the bits in B1-G1 to A6-A1 (the order gets reversed)
        let rev_occupancy_idx = rank_1_occupancy.0.wrapping_mul(B3_TO_G8_DIAGONAL.0) >> 58;
        let rev_file_a_targets =
            FILE_A_TARGETS[7 - origin.file().idx()][rev_occupancy_idx as usize] & Bitboard::FILE_A;
        // This multiplication reverses the bits a second time, so in the end
        // the bits are in the right order
        Bitboard(rev_file_a_targets.0.wrapping_mul(A1_TO_H8_DIAGONAL.0) >> (7 - rank_idx))
            & rank_mask
    }

    pub fn diagonal_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        const B2_TO_G2_RANK: Bitboard = Bitboard(0x0002020202020200);
        let diagonal_mask = DIAGONAL_RAYS[origin.idx()];
        let occupancy_idx = (diagonal_mask & occupied).0.wrapping_mul(B2_TO_G2_RANK.0) >> 58;
        FILE_A_TARGETS[origin.rank().idx()][occupancy_idx as usize] & diagonal_mask
    }

    pub fn anti_diagonal_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        const B2_TO_G2_RANK: Bitboard = Bitboard(0x0002020202020200);
        let anti_diagonal_mask = ANTI_DIAGONAL_RAYS[origin.idx()];
        let occupancy_idx = (anti_diagonal_mask & occupied)
            .0
            .wrapping_mul(B2_TO_G2_RANK.0)
            >> 58;
        FILE_A_TARGETS[origin.rank().idx()][occupancy_idx as usize] & anti_diagonal_mask
    }

    #[allow(dead_code)]
    fn north_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::positive_targets(origin, occupied, Direction::North)
    }

    #[allow(dead_code)]
    fn north_east_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::positive_targets(origin, occupied, Direction::NorthEast)
    }

    #[allow(dead_code)]
    fn east_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::positive_targets(origin, occupied, Direction::East)
    }

    #[allow(dead_code)]
    fn south_east_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::positive_targets(origin, occupied, Direction::SouthEast)
    }

    #[allow(dead_code)]
    fn south_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::negative_targets(origin, occupied, Direction::South)
    }

    #[allow(dead_code)]
    fn south_west_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::negative_targets(origin, occupied, Direction::SouthWest)
    }

    #[allow(dead_code)]
    fn west_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::negative_targets(origin, occupied, Direction::West)
    }

    #[allow(dead_code)]
    fn north_west_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::negative_targets(origin, occupied, Direction::NorthWest)
    }

    fn positive_targets(origin: Square, occupied: Bitboard, direction: Direction) -> Bitboard {
        let empty_board_targets = DIRECTION_RAYS[direction as usize][origin.idx()];
        let blocked = empty_board_targets & occupied;
        // At least one bit must be set when calling square_scan_forward. Setting the most
        // significant bit does not change the targets and allows a branchless implementation.
        let first_blocked = (blocked | Bitboard(0x8000000000000000)).square_scan_forward();
        empty_board_targets ^ DIRECTION_RAYS[direction as usize][first_blocked.idx()]
    }

    fn negative_targets(origin: Square, occupied: Bitboard, direction: Direction) -> Bitboard {
        let empty_board_targets = DIRECTION_RAYS[direction as usize][origin.idx()];
        let blocked = empty_board_targets & occupied;
        // At least one bit must be set when calling square_scan_reverse. Setting the least
        // significant bit does not change the targets and allows a branchless implementation.
        let first_blocked = (blocked | Bitboard(0x0000000000000001)).square_scan_reverse();
        empty_board_targets ^ DIRECTION_RAYS[direction as usize][first_blocked.idx()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray_lookup_tables::{
        ANTI_DIAGONAL_RAYS, DIAGONAL_RAYS, EAST_RAYS, NORTH_EAST_RAYS, NORTH_RAYS, NORTH_WEST_RAYS,
        RANK_RAYS, SOUTH_EAST_RAYS, SOUTH_RAYS, SOUTH_WEST_RAYS, WEST_RAYS,
    };

    #[test]
    fn north_targets() {
        assert_eq!(
            Bitboard::D2
                | Bitboard::D3
                | Bitboard::D4
                | Bitboard::D5
                | Bitboard::D6
                | Bitboard::D7
                | Bitboard::D8,
            Ray::north_targets(Square::D1, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::D2 | Bitboard::D3 | Bitboard::D4 | Bitboard::D5 | Bitboard::D6 | Bitboard::D7,
            Ray::north_targets(Square::D1, Bitboard::D7)
        );

        assert_eq!(
            Bitboard::D2,
            Ray::north_targets(Square::D1, Bitboard::D2 | Bitboard::D7)
        );
    }

    #[test]
    fn south_targets() {
        assert_eq!(
            Bitboard::D7
                | Bitboard::D6
                | Bitboard::D5
                | Bitboard::D4
                | Bitboard::D3
                | Bitboard::D2
                | Bitboard::D1,
            Ray::south_targets(Square::D8, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::D7 | Bitboard::D6 | Bitboard::D5 | Bitboard::D4 | Bitboard::D3 | Bitboard::D2,
            Ray::south_targets(Square::D8, Bitboard::D2)
        );

        assert_eq!(
            Bitboard::D7,
            Ray::south_targets(Square::D8, Bitboard::D7 | Bitboard::D2)
        );
    }

    #[test]
    fn east_targets() {
        assert_eq!(
            Bitboard::B4
                | Bitboard::C4
                | Bitboard::D4
                | Bitboard::E4
                | Bitboard::F4
                | Bitboard::G4
                | Bitboard::H4,
            Ray::east_targets(Square::A4, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::B4 | Bitboard::C4 | Bitboard::D4 | Bitboard::E4 | Bitboard::F4 | Bitboard::G4,
            Ray::east_targets(Square::A4, Bitboard::G4)
        );

        assert_eq!(
            Bitboard::B4,
            Ray::east_targets(Square::A4, Bitboard::B4 | Bitboard::G4)
        );
    }

    #[test]
    fn west_targets() {
        assert_eq!(
            Bitboard::G4
                | Bitboard::F4
                | Bitboard::E4
                | Bitboard::D4
                | Bitboard::C4
                | Bitboard::B4
                | Bitboard::A4,
            Ray::west_targets(Square::H4, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::G4 | Bitboard::F4 | Bitboard::E4 | Bitboard::D4 | Bitboard::C4 | Bitboard::B4,
            Ray::west_targets(Square::H4, Bitboard::B4)
        );

        assert_eq!(
            Bitboard::G4,
            Ray::west_targets(Square::H4, Bitboard::G4 | Bitboard::B4)
        );
    }

    #[test]
    fn north_east_targets() {
        assert_eq!(
            Bitboard::B2
                | Bitboard::C3
                | Bitboard::D4
                | Bitboard::E5
                | Bitboard::F6
                | Bitboard::G7
                | Bitboard::H8,
            Ray::north_east_targets(Square::A1, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::B2 | Bitboard::C3 | Bitboard::D4 | Bitboard::E5 | Bitboard::F6 | Bitboard::G7,
            Ray::north_east_targets(Square::A1, Bitboard::G7)
        );

        assert_eq!(
            Bitboard::B2,
            Ray::north_east_targets(Square::A1, Bitboard::B2 | Bitboard::G7)
        );
    }

    #[test]
    fn south_east_targets() {
        assert_eq!(
            Bitboard::B7
                | Bitboard::C6
                | Bitboard::D5
                | Bitboard::E4
                | Bitboard::F3
                | Bitboard::G2
                | Bitboard::H1,
            Ray::south_east_targets(Square::A8, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::B7 | Bitboard::C6 | Bitboard::D5 | Bitboard::E4 | Bitboard::F3 | Bitboard::G2,
            Ray::south_east_targets(Square::A8, Bitboard::G2)
        );

        assert_eq!(
            Bitboard::B7,
            Ray::south_east_targets(Square::A8, Bitboard::B7 | Bitboard::G2)
        );
    }

    #[test]
    fn south_west_targets() {
        assert_eq!(
            Bitboard::G7
                | Bitboard::F6
                | Bitboard::E5
                | Bitboard::D4
                | Bitboard::C3
                | Bitboard::B2
                | Bitboard::A1,
            Ray::south_west_targets(Square::H8, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::G7 | Bitboard::F6 | Bitboard::E5 | Bitboard::D4 | Bitboard::C3 | Bitboard::B2,
            Ray::south_west_targets(Square::H8, Bitboard::B2)
        );

        assert_eq!(
            Bitboard::G7,
            Ray::south_west_targets(Square::H8, Bitboard::G7 | Bitboard::B2)
        );
    }

    #[test]
    fn north_west_targets() {
        assert_eq!(
            Bitboard::G2
                | Bitboard::F3
                | Bitboard::E4
                | Bitboard::D5
                | Bitboard::C6
                | Bitboard::B7
                | Bitboard::A8,
            Ray::north_west_targets(Square::H1, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::G2 | Bitboard::F3 | Bitboard::E4 | Bitboard::D5 | Bitboard::C6 | Bitboard::B7,
            Ray::north_west_targets(Square::H1, Bitboard::B7)
        );

        assert_eq!(
            Bitboard::G2,
            Ray::north_west_targets(Square::H1, Bitboard::G2 | Bitboard::B7)
        );
    }

    #[test]
    fn file_rays() {
        let mut idx = 0;
        while idx < Square::NUM_SQUARES {
            assert_eq!(FILE_RAYS[idx], NORTH_RAYS[idx] | SOUTH_RAYS[idx]);
            idx += 1;
        }
        println!("{FILE_RAYS:#018x?}");
    }

    #[test]
    fn rank_rays() {
        let mut idx = 0;
        while idx < Square::NUM_SQUARES {
            assert_eq!(RANK_RAYS[idx], EAST_RAYS[idx] | WEST_RAYS[idx]);
            idx += 1;
        }
        println!("{RANK_RAYS:#018x?}");
    }

    #[test]
    fn diagonal_rays() {
        let mut idx = 0;
        while idx < Square::NUM_SQUARES {
            assert_eq!(
                DIAGONAL_RAYS[idx],
                NORTH_EAST_RAYS[idx] | SOUTH_WEST_RAYS[idx]
            );
            idx += 1;
        }
        println!("{DIAGONAL_RAYS:#018x?}");
    }

    #[test]
    fn anti_diagonal_rays() {
        let mut idx = 0;
        while idx < Square::NUM_SQUARES {
            assert_eq!(
                ANTI_DIAGONAL_RAYS[idx],
                NORTH_WEST_RAYS[idx] | SOUTH_EAST_RAYS[idx]
            );
            idx += 1;
        }
        println!("{ANTI_DIAGONAL_RAYS:#018x?}");
    }

    #[test]
    fn calculate_file_a_targets() {
        let mut file_a_targets = [[Bitboard::EMPTY; 64]; 8];
        for (square_idx, file_a_targets_for_square) in file_a_targets.iter_mut().enumerate() {
            let square = Square::from_idx(square_idx);
            for (occupancy_idx, targets) in file_a_targets_for_square.iter_mut().enumerate() {
                let occupancy = Bitboard((occupancy_idx as u64) << 1);
                *targets =
                    Ray::north_targets(square, occupancy) | Ray::south_targets(square, occupancy);
                *targets = targets.east_fill();
            }
        }
        println!("{file_a_targets:#018x?}");
    }

    #[test]
    fn file_targets() {
        let square = Square::E3;
        let occupancy = Bitboard::E1 | Bitboard::E3 | Bitboard::E7 | Bitboard::E8;
        let expected_targets =
            Bitboard::E1 | Bitboard::E2 | Bitboard::E4 | Bitboard::E5 | Bitboard::E6 | Bitboard::E7;
        let actual_targets = Ray::file_targets(square, occupancy);
        assert_eq!(
            expected_targets, actual_targets,
            "\nExpected targets:\n{expected_targets}\nActual targets:\n{actual_targets}",
        );
    }

    #[test]
    fn rank_targets() {
        let square = Square::E3;
        let occupancy = Bitboard::B3 | Bitboard::E3 | Bitboard::G3 | Bitboard::H3;
        let expected_targets =
            Bitboard::B3 | Bitboard::C3 | Bitboard::D3 | Bitboard::F3 | Bitboard::G3;
        let actual_targets = Ray::rank_targets(square, occupancy);
        assert_eq!(
            expected_targets, actual_targets,
            "\nExpected targets:\n{expected_targets}\nActual targets:\n{actual_targets}",
        );
    }

    #[test]
    fn diagonal_targets() {
        let square = Square::E3;
        let occupancy = Bitboard::C1 | Bitboard::E3 | Bitboard::G5;
        let expected_targets = Bitboard::C1 | Bitboard::D2 | Bitboard::F4 | Bitboard::G5;
        let actual_targets = Ray::diagonal_targets(square, occupancy);
        assert_eq!(
            expected_targets, actual_targets,
            "\nExpected targets:\n{expected_targets}\nActual targets:\n{actual_targets}",
        );
    }

    #[test]
    fn anti_diagonal_targets() {
        let square = Square::E3;
        let occupancy = Bitboard::G1 | Bitboard::E3 | Bitboard::B6;
        let expected_targets =
            Bitboard::G1 | Bitboard::F2 | Bitboard::D4 | Bitboard::C5 | Bitboard::B6;
        let actual_targets = Ray::anti_diagonal_targets(square, occupancy);
        assert_eq!(
            expected_targets, actual_targets,
            "\nExpected targets:\n{expected_targets}\nActual targets:\n{actual_targets}",
        );
    }
}
