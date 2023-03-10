use crate::file::File;
use crate::rank::Rank;
use std::fmt;
use std::str;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Square(u8);

impl Square {
    pub const NUM_SQUARES: usize = Rank::NUM_RANKS * File::NUM_FILES;

    pub const A1: Self = Square(0);
    pub const A2: Self = Square(1);
    pub const A3: Self = Square(2);
    pub const A4: Self = Square(3);
    pub const A5: Self = Square(4);
    pub const A6: Self = Square(5);
    pub const A7: Self = Square(6);
    pub const A8: Self = Square(7);
    pub const B1: Self = Square(8);
    pub const B2: Self = Square(9);
    pub const B3: Self = Square(10);
    pub const B4: Self = Square(11);
    pub const B5: Self = Square(12);
    pub const B6: Self = Square(13);
    pub const B7: Self = Square(14);
    pub const B8: Self = Square(15);
    pub const C1: Self = Square(16);
    pub const C2: Self = Square(17);
    pub const C3: Self = Square(18);
    pub const C4: Self = Square(19);
    pub const C5: Self = Square(20);
    pub const C6: Self = Square(21);
    pub const C7: Self = Square(22);
    pub const C8: Self = Square(23);
    pub const D1: Self = Square(24);
    pub const D2: Self = Square(25);
    pub const D3: Self = Square(26);
    pub const D4: Self = Square(27);
    pub const D5: Self = Square(28);
    pub const D6: Self = Square(29);
    pub const D7: Self = Square(30);
    pub const D8: Self = Square(31);
    pub const E1: Self = Square(32);
    pub const E2: Self = Square(33);
    pub const E3: Self = Square(34);
    pub const E4: Self = Square(35);
    pub const E5: Self = Square(36);
    pub const E6: Self = Square(37);
    pub const E7: Self = Square(38);
    pub const E8: Self = Square(39);
    pub const F1: Self = Square(40);
    pub const F2: Self = Square(41);
    pub const F3: Self = Square(42);
    pub const F4: Self = Square(43);
    pub const F5: Self = Square(44);
    pub const F6: Self = Square(45);
    pub const F7: Self = Square(46);
    pub const F8: Self = Square(47);
    pub const G1: Self = Square(48);
    pub const G2: Self = Square(49);
    pub const G3: Self = Square(50);
    pub const G4: Self = Square(51);
    pub const G5: Self = Square(52);
    pub const G6: Self = Square(53);
    pub const G7: Self = Square(54);
    pub const G8: Self = Square(55);
    pub const H1: Self = Square(56);
    pub const H2: Self = Square(57);
    pub const H3: Self = Square(58);
    pub const H4: Self = Square(59);
    pub const H5: Self = Square(60);
    pub const H6: Self = Square(61);
    pub const H7: Self = Square(62);
    pub const H8: Self = Square(63);

    pub const fn from_idx(idx: usize) -> Square {
        debug_assert!(idx < Self::NUM_SQUARES);
        Square(idx as u8)
    }

    pub const fn from_file_and_rank(file: File, rank: Rank) -> Square {
        debug_assert!(file.idx() < File::NUM_FILES);
        debug_assert!(rank.idx() < Rank::NUM_RANKS);
        Square::from_idx(file.idx() * Rank::NUM_RANKS + rank.idx())
    }

    pub const fn idx(&self) -> usize {
        self.0 as usize
    }

    pub fn file(&self) -> File {
        debug_assert!(self.idx() < Self::NUM_SQUARES);
        File::from_idx(self.idx() / Rank::NUM_RANKS)
    }

    pub fn rank(&self) -> Rank {
        debug_assert!(self.idx() < Self::NUM_SQUARES);
        Rank::from_idx(self.idx() % Rank::NUM_RANKS)
    }

    pub fn from_ascii(s: &[u8; 2]) -> Result<Self, String> {
        let file = File::from_ascii(s[0]);
        let rank = Rank::from_ascii(s[1]);

        match (file, rank) {
            (Ok(f), Ok(r)) => Ok(Self::from_file_and_rank(f, r)),
            _ => Err(format!("Invalid square `{}`", str::from_utf8(s).unwrap())),
        }
    }

    pub fn to_ascii(self) -> [u8; 2] {
        debug_assert!(self.idx() < Self::NUM_SQUARES);
        [self.file().to_ascii(), self.rank().to_ascii()]
    }

    pub fn north_one(self) -> Square {
        debug_assert!(self.rank().idx() < Rank::NUM_RANKS);
        Self(self.0 + 1)
    }

    pub fn north_two(self) -> Square {
        debug_assert!(self.rank().idx() < Rank::NUM_RANKS - 1);
        Self(self.0 + 2)
    }

    pub fn north_east_one(self) -> Square {
        debug_assert!(self.rank().idx() < Rank::NUM_RANKS);
        debug_assert!(self.file().idx() < File::NUM_FILES);
        Self(self.0 + 9)
    }

    pub fn north_west_one(self) -> Square {
        debug_assert!(self.rank().idx() < Rank::NUM_RANKS);
        debug_assert!(self.file().idx() > 0);
        Self(self.0 - 7)
    }

    pub fn south_one(self) -> Square {
        debug_assert!(self.rank().idx() > 0);
        Self(self.0 - 1)
    }

    pub fn south_two(self) -> Square {
        debug_assert!(self.rank().idx() > 1);
        Self(self.0 - 2)
    }

    pub fn south_east_one(self) -> Square {
        debug_assert!(self.rank().idx() > 0);
        debug_assert!(self.file().idx() < File::NUM_FILES);
        Self(self.0 + 7)
    }

    pub fn south_west_one(self) -> Square {
        debug_assert!(self.rank().idx() > 0);
        debug_assert!(self.file().idx() > 0);
        Self(self.0 - 9)
    }

    pub const fn flip_vertical(self) -> Square {
        Self(self.0 ^ 0x7)
    }

    pub const fn mirror_horizontal(self) -> Square {
        Self(self.0 ^ 0x38)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", str::from_utf8(&self.to_ascii()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_idx() {
        assert_eq!(Square::A1, Square::from_idx(0));
        assert_eq!(Square::A2, Square::from_idx(1));
        assert_eq!(Square::B1, Square::from_idx(8));
        assert_eq!(Square::H8, Square::from_idx(Square::NUM_SQUARES - 1));
    }

    #[test]
    fn idx() {
        assert_eq!(0, Square::A1.idx());
        assert_eq!(1, Square::A2.idx());
        assert_eq!(8, Square::B1.idx());
        assert_eq!(Square::NUM_SQUARES - 1, Square::H8.idx());
    }

    #[test]
    fn file_rank_square_indexing() {
        // A1
        assert_eq!(Square::A1, Square::from_file_and_rank(File::A, Rank::R1));
        assert_eq!(Rank::R1, Square::A1.rank());
        assert_eq!(File::A, Square::A1.file());
        // A2
        assert_eq!(Square::A2, Square::from_file_and_rank(File::A, Rank::R2));
        assert_eq!(Rank::R2, Square::A2.rank());
        assert_eq!(File::A, Square::A2.file());
        // B1
        assert_eq!(Square::B1, Square::from_file_and_rank(File::B, Rank::R1));
        assert_eq!(Rank::R1, Square::B1.rank());
        assert_eq!(File::B, Square::B1.file());
        // B2
        assert_eq!(Square::B2, Square::from_file_and_rank(File::B, Rank::R2));
        assert_eq!(Rank::R2, Square::B2.rank());
        assert_eq!(File::B, Square::B2.file());
        // E6
        assert_eq!(Square::E6, Square::from_file_and_rank(File::E, Rank::R6));
        assert_eq!(Rank::R6, Square::E6.rank());
        assert_eq!(File::E, Square::E6.file());
        // F8
        assert_eq!(Square::F8, Square::from_file_and_rank(File::F, Rank::R8));
        assert_eq!(Rank::R8, Square::F8.rank());
        assert_eq!(File::F, Square::F8.file());
        // H7
        assert_eq!(Square::H7, Square::from_file_and_rank(File::H, Rank::R7));
        assert_eq!(Rank::R7, Square::H7.rank());
        assert_eq!(File::H, Square::H7.file());
        // H8
        assert_eq!(Square::H8, Square::from_file_and_rank(File::H, Rank::R8));
        assert_eq!(Rank::R8, Square::H8.rank());
        assert_eq!(File::H, Square::H8.file());

        for idx in 0..Square::NUM_SQUARES {
            let square = Square::from_idx(idx);
            assert_eq!(
                square,
                Square::from_file_and_rank(square.file(), square.rank())
            );
        }
    }

    #[test]
    fn from_ascii() {
        assert_eq!(Ok(Square::A1), Square::from_ascii(&[b'a', b'1']));
        assert_eq!(Ok(Square::H1), Square::from_ascii(&[b'h', b'1']));
        assert_eq!(Ok(Square::D4), Square::from_ascii(&[b'd', b'4']));
        assert_eq!(Ok(Square::E5), Square::from_ascii(&[b'e', b'5']));
        assert_eq!(Ok(Square::A8), Square::from_ascii(&[b'a', b'8']));
        assert_eq!(Ok(Square::H8), Square::from_ascii(&[b'h', b'8']));
        assert_eq!(
            Err(String::from("Invalid square `i1`")),
            Square::from_ascii(&[b'i', b'1'])
        );
        assert_eq!(
            Err(String::from("Invalid square `a9`")),
            Square::from_ascii(&[b'a', b'9'])
        );
    }

    #[test]
    fn to_ascii() {
        assert_eq!([b'a', b'1'], Square::A1.to_ascii());
        assert_eq!([b'h', b'1'], Square::H1.to_ascii());
        assert_eq!([b'd', b'4'], Square::D4.to_ascii());
        assert_eq!([b'e', b'5'], Square::E5.to_ascii());
        assert_eq!([b'a', b'8'], Square::A8.to_ascii());
        assert_eq!([b'h', b'8'], Square::H8.to_ascii());
    }

    #[test]
    fn north_one() {
        assert_eq!(Square::D5, Square::north_one(Square::D4));
    }

    #[test]
    fn north_two() {
        assert_eq!(Square::D6, Square::north_two(Square::D4));
    }

    #[test]
    fn north_east_one() {
        assert_eq!(Square::E5, Square::north_east_one(Square::D4));
    }

    #[test]
    fn north_west_one() {
        assert_eq!(Square::C5, Square::north_west_one(Square::D4));
    }

    #[test]
    fn south_one() {
        assert_eq!(Square::D3, Square::south_one(Square::D4));
    }

    #[test]
    fn south_two() {
        assert_eq!(Square::D2, Square::south_two(Square::D4));
    }

    #[test]
    fn south_east_one() {
        assert_eq!(Square::E3, Square::south_east_one(Square::D4));
    }

    #[test]
    fn south_west_one() {
        assert_eq!(Square::C3, Square::south_west_one(Square::D4));
    }

    #[test]
    fn flip_vertical() {
        assert_eq!(Square::A8, Square::flip_vertical(Square::A1));
        assert_eq!(Square::A2, Square::flip_vertical(Square::A7));
        assert_eq!(Square::B1, Square::flip_vertical(Square::B8));
        assert_eq!(Square::D4, Square::flip_vertical(Square::D5));
        assert_eq!(Square::E5, Square::flip_vertical(Square::E4));
        assert_eq!(Square::G8, Square::flip_vertical(Square::G1));
        assert_eq!(Square::H7, Square::flip_vertical(Square::H2));
        assert_eq!(Square::H1, Square::flip_vertical(Square::H8));
    }

    #[test]
    fn mirror_horizontal() {
        assert_eq!(Square::H1, Square::mirror_horizontal(Square::A1));
        assert_eq!(Square::H7, Square::mirror_horizontal(Square::A7));
        assert_eq!(Square::G8, Square::mirror_horizontal(Square::B8));
        assert_eq!(Square::E5, Square::mirror_horizontal(Square::D5));
        assert_eq!(Square::D4, Square::mirror_horizontal(Square::E4));
        assert_eq!(Square::B1, Square::mirror_horizontal(Square::G1));
        assert_eq!(Square::A2, Square::mirror_horizontal(Square::H2));
        assert_eq!(Square::A8, Square::mirror_horizontal(Square::H8));
    }

    #[test]
    fn fmt() {
        assert_eq!("a1", format!("{}", Square::A1));
        assert_eq!("h1", format!("{}", Square::H1));
        assert_eq!("d4", format!("{}", Square::D4));
        assert_eq!("e5", format!("{}", Square::E5));
        assert_eq!("a8", format!("{}", Square::A8));
        assert_eq!("h8", format!("{}", Square::H8));
    }
}
