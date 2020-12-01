use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
use std::str;

// Bitboard using little endian file rank mapping (LEFR)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const NUM_RANKS: usize = 8;
    pub const NUM_FILES: usize = 8;
    pub const NUM_SQUARES: usize = Self::NUM_RANKS * Self::NUM_FILES;

    pub const EMPTY: Self = Bitboard(0x0000000000000000);
    pub const UNIVERSE: Self = Bitboard(0xffffffffffffffff);

    pub const FILE_A: Self = Bitboard(0x00000000000000ff);
    pub const FILE_B: Self = Bitboard(0x000000000000ff00);
    pub const FILE_C: Self = Bitboard(0x0000000000ff0000);
    pub const FILE_D: Self = Bitboard(0x00000000ff000000);
    pub const FILE_E: Self = Bitboard(0x000000ff00000000);
    pub const FILE_F: Self = Bitboard(0x0000ff0000000000);
    pub const FILE_G: Self = Bitboard(0x00ff000000000000);
    pub const FILE_H: Self = Bitboard(0xff00000000000000);

    pub const RANK_1: Self = Bitboard(0x0101010101010101);
    pub const RANK_2: Self = Bitboard(0x0202020202020202);
    pub const RANK_3: Self = Bitboard(0x0404040404040404);
    pub const RANK_4: Self = Bitboard(0x0808080808080808);
    pub const RANK_5: Self = Bitboard(0x1010101010101010);
    pub const RANK_6: Self = Bitboard(0x2020202020202020);
    pub const RANK_7: Self = Bitboard(0x4040404040404040);
    pub const RANK_8: Self = Bitboard(0x8080808080808080);

    pub const A1: Self = Bitboard(0x0000000000000001 << Self::IDX_A1);
    pub const A2: Self = Bitboard(0x0000000000000001 << Self::IDX_A2);
    pub const A3: Self = Bitboard(0x0000000000000001 << Self::IDX_A3);
    pub const A4: Self = Bitboard(0x0000000000000001 << Self::IDX_A4);
    pub const A5: Self = Bitboard(0x0000000000000001 << Self::IDX_A5);
    pub const A6: Self = Bitboard(0x0000000000000001 << Self::IDX_A6);
    pub const A7: Self = Bitboard(0x0000000000000001 << Self::IDX_A7);
    pub const A8: Self = Bitboard(0x0000000000000001 << Self::IDX_A8);
    pub const B1: Self = Bitboard(0x0000000000000001 << Self::IDX_B1);
    pub const B2: Self = Bitboard(0x0000000000000001 << Self::IDX_B2);
    pub const B3: Self = Bitboard(0x0000000000000001 << Self::IDX_B3);
    pub const B4: Self = Bitboard(0x0000000000000001 << Self::IDX_B4);
    pub const B5: Self = Bitboard(0x0000000000000001 << Self::IDX_B5);
    pub const B6: Self = Bitboard(0x0000000000000001 << Self::IDX_B6);
    pub const B7: Self = Bitboard(0x0000000000000001 << Self::IDX_B7);
    pub const B8: Self = Bitboard(0x0000000000000001 << Self::IDX_B8);
    pub const C1: Self = Bitboard(0x0000000000000001 << Self::IDX_C1);
    pub const C2: Self = Bitboard(0x0000000000000001 << Self::IDX_C2);
    pub const C3: Self = Bitboard(0x0000000000000001 << Self::IDX_C3);
    pub const C4: Self = Bitboard(0x0000000000000001 << Self::IDX_C4);
    pub const C5: Self = Bitboard(0x0000000000000001 << Self::IDX_C5);
    pub const C6: Self = Bitboard(0x0000000000000001 << Self::IDX_C6);
    pub const C7: Self = Bitboard(0x0000000000000001 << Self::IDX_C7);
    pub const C8: Self = Bitboard(0x0000000000000001 << Self::IDX_C8);
    pub const D1: Self = Bitboard(0x0000000000000001 << Self::IDX_D1);
    pub const D2: Self = Bitboard(0x0000000000000001 << Self::IDX_D2);
    pub const D3: Self = Bitboard(0x0000000000000001 << Self::IDX_D3);
    pub const D4: Self = Bitboard(0x0000000000000001 << Self::IDX_D4);
    pub const D5: Self = Bitboard(0x0000000000000001 << Self::IDX_D5);
    pub const D6: Self = Bitboard(0x0000000000000001 << Self::IDX_D6);
    pub const D7: Self = Bitboard(0x0000000000000001 << Self::IDX_D7);
    pub const D8: Self = Bitboard(0x0000000000000001 << Self::IDX_D8);
    pub const E1: Self = Bitboard(0x0000000000000001 << Self::IDX_E1);
    pub const E2: Self = Bitboard(0x0000000000000001 << Self::IDX_E2);
    pub const E3: Self = Bitboard(0x0000000000000001 << Self::IDX_E3);
    pub const E4: Self = Bitboard(0x0000000000000001 << Self::IDX_E4);
    pub const E5: Self = Bitboard(0x0000000000000001 << Self::IDX_E5);
    pub const E6: Self = Bitboard(0x0000000000000001 << Self::IDX_E6);
    pub const E7: Self = Bitboard(0x0000000000000001 << Self::IDX_E7);
    pub const E8: Self = Bitboard(0x0000000000000001 << Self::IDX_E8);
    pub const F1: Self = Bitboard(0x0000000000000001 << Self::IDX_F1);
    pub const F2: Self = Bitboard(0x0000000000000001 << Self::IDX_F2);
    pub const F3: Self = Bitboard(0x0000000000000001 << Self::IDX_F3);
    pub const F4: Self = Bitboard(0x0000000000000001 << Self::IDX_F4);
    pub const F5: Self = Bitboard(0x0000000000000001 << Self::IDX_F5);
    pub const F6: Self = Bitboard(0x0000000000000001 << Self::IDX_F6);
    pub const F7: Self = Bitboard(0x0000000000000001 << Self::IDX_F7);
    pub const F8: Self = Bitboard(0x0000000000000001 << Self::IDX_F8);
    pub const G1: Self = Bitboard(0x0000000000000001 << Self::IDX_G1);
    pub const G2: Self = Bitboard(0x0000000000000001 << Self::IDX_G2);
    pub const G3: Self = Bitboard(0x0000000000000001 << Self::IDX_G3);
    pub const G4: Self = Bitboard(0x0000000000000001 << Self::IDX_G4);
    pub const G5: Self = Bitboard(0x0000000000000001 << Self::IDX_G5);
    pub const G6: Self = Bitboard(0x0000000000000001 << Self::IDX_G6);
    pub const G7: Self = Bitboard(0x0000000000000001 << Self::IDX_G7);
    pub const G8: Self = Bitboard(0x0000000000000001 << Self::IDX_G8);
    pub const H1: Self = Bitboard(0x0000000000000001 << Self::IDX_H1);
    pub const H2: Self = Bitboard(0x0000000000000001 << Self::IDX_H2);
    pub const H3: Self = Bitboard(0x0000000000000001 << Self::IDX_H3);
    pub const H4: Self = Bitboard(0x0000000000000001 << Self::IDX_H4);
    pub const H5: Self = Bitboard(0x0000000000000001 << Self::IDX_H5);
    pub const H6: Self = Bitboard(0x0000000000000001 << Self::IDX_H6);
    pub const H7: Self = Bitboard(0x0000000000000001 << Self::IDX_H7);
    pub const H8: Self = Bitboard(0x0000000000000001 << Self::IDX_H8);

    pub const IDX_A1: usize = 0;
    pub const IDX_A2: usize = 1;
    pub const IDX_A3: usize = 2;
    pub const IDX_A4: usize = 3;
    pub const IDX_A5: usize = 4;
    pub const IDX_A6: usize = 5;
    pub const IDX_A7: usize = 6;
    pub const IDX_A8: usize = 7;
    pub const IDX_B1: usize = 8;
    pub const IDX_B2: usize = 9;
    pub const IDX_B3: usize = 10;
    pub const IDX_B4: usize = 11;
    pub const IDX_B5: usize = 12;
    pub const IDX_B6: usize = 13;
    pub const IDX_B7: usize = 14;
    pub const IDX_B8: usize = 15;
    pub const IDX_C1: usize = 16;
    pub const IDX_C2: usize = 17;
    pub const IDX_C3: usize = 18;
    pub const IDX_C4: usize = 19;
    pub const IDX_C5: usize = 20;
    pub const IDX_C6: usize = 21;
    pub const IDX_C7: usize = 22;
    pub const IDX_C8: usize = 23;
    pub const IDX_D1: usize = 24;
    pub const IDX_D2: usize = 25;
    pub const IDX_D3: usize = 26;
    pub const IDX_D4: usize = 27;
    pub const IDX_D5: usize = 28;
    pub const IDX_D6: usize = 29;
    pub const IDX_D7: usize = 30;
    pub const IDX_D8: usize = 31;
    pub const IDX_E1: usize = 32;
    pub const IDX_E2: usize = 33;
    pub const IDX_E3: usize = 34;
    pub const IDX_E4: usize = 35;
    pub const IDX_E5: usize = 36;
    pub const IDX_E6: usize = 37;
    pub const IDX_E7: usize = 38;
    pub const IDX_E8: usize = 39;
    pub const IDX_F1: usize = 40;
    pub const IDX_F2: usize = 41;
    pub const IDX_F3: usize = 42;
    pub const IDX_F4: usize = 43;
    pub const IDX_F5: usize = 44;
    pub const IDX_F6: usize = 45;
    pub const IDX_F7: usize = 46;
    pub const IDX_F8: usize = 47;
    pub const IDX_G1: usize = 48;
    pub const IDX_G2: usize = 49;
    pub const IDX_G3: usize = 50;
    pub const IDX_G4: usize = 51;
    pub const IDX_G5: usize = 52;
    pub const IDX_G6: usize = 53;
    pub const IDX_G7: usize = 54;
    pub const IDX_G8: usize = 55;
    pub const IDX_H1: usize = 56;
    pub const IDX_H2: usize = 57;
    pub const IDX_H3: usize = 58;
    pub const IDX_H4: usize = 59;
    pub const IDX_H5: usize = 60;
    pub const IDX_H6: usize = 61;
    pub const IDX_H7: usize = 62;
    pub const IDX_H8: usize = 63;

    pub fn to_square(rank: usize, file: usize) -> usize {
        debug_assert!(rank < Self::NUM_RANKS);
        debug_assert!(file < Self::NUM_FILES);
        file * Self::NUM_RANKS + rank
    }

    fn to_rank(square: usize) -> usize {
        debug_assert!(square < Self::NUM_SQUARES);
        square % Self::NUM_RANKS
    }

    fn to_file(square: usize) -> usize {
        debug_assert!(square < Self::NUM_SQUARES);
        square / Self::NUM_RANKS
    }

    pub fn north_one(self) -> Self {
        self << 1 & !Self::RANK_1
    }

    pub fn north_one_if_rank_8_empty(self) -> Self {
        debug_assert_eq!(Bitboard::EMPTY, self & Bitboard::RANK_8);
        self << 1
    }

    pub fn north_east_one(self) -> Self {
        self << 9 & !Self::RANK_1
    }

    pub fn north_east_one_if_rank_8_empty(self) -> Self {
        debug_assert_eq!(Bitboard::EMPTY, self & Bitboard::RANK_8);
        self << 9
    }

    pub fn east_one(self) -> Self {
        self << 8
    }

    pub fn south_east_one(self) -> Self {
        self << 7 & !Self::RANK_8
    }

    pub fn south_east_one_if_rank_1_empty(self) -> Self {
        debug_assert_eq!(Bitboard::EMPTY, self & Bitboard::RANK_1);
        self << 7
    }

    pub fn south_one(self) -> Self {
        self >> 1 & !Self::RANK_8
    }

    pub fn south_one_if_rank_1_empty(self) -> Self {
        debug_assert_eq!(Bitboard::EMPTY, self & Bitboard::RANK_1);
        self >> 1
    }

    pub fn south_west_one(self) -> Self {
        self >> 9 & !Self::RANK_8
    }

    pub fn south_west_one_if_rank_1_empty(self) -> Self {
        debug_assert_eq!(Bitboard::EMPTY, self & Bitboard::RANK_1);
        self >> 9
    }

    pub fn west_one(self) -> Self {
        self >> 8
    }

    pub fn north_west_one(self) -> Self {
        self >> 7 & !Self::RANK_1
    }

    pub fn north_west_one_if_rank_8_empty(self) -> Self {
        debug_assert_eq!(Bitboard::EMPTY, self & Bitboard::RANK_8);
        self >> 7
    }

    pub fn north_two_east_one(self) -> Self {
        self << 10 & !(Self::RANK_1 | Self::RANK_2)
    }

    pub fn north_one_east_two(self) -> Self {
        self << 17 & !Self::RANK_1
    }

    pub fn south_one_east_two(self) -> Self {
        self << 15 & !Self::RANK_8
    }

    pub fn south_two_east_one(self) -> Self {
        self << 6 & !(Self::RANK_8 | Self::RANK_7)
    }

    pub fn south_two_west_one(self) -> Self {
        self >> 10 & !(Self::RANK_8 | Self::RANK_7)
    }

    pub fn south_one_west_two(self) -> Self {
        self >> 17 & !Self::RANK_8
    }

    pub fn north_one_west_two(self) -> Self {
        self >> 15 & !Self::RANK_1
    }

    pub fn north_two_west_one(self) -> Self {
        self >> 6 & !(Self::RANK_1 | Self::RANK_2)
    }

    fn north_fill(self) -> Self {
        let mut board = self;
        board = board | board.north_one();
        let next_bits = board << 2 & !Self::RANK_1 & !Self::RANK_2;
        board = board | next_bits;
        let next_bits = board << 4 & !Self::RANK_1 & !Self::RANK_2 & !Self::RANK_3 & !Self::RANK_4;
        board = board | next_bits;
        board
    }

    fn north_east_fill(self) -> Self {
        let mut board = self;
        board = board | board.north_east_one();
        let next_bits = board << 18 & !Self::RANK_1 & !Self::RANK_2;
        board = board | next_bits;
        let next_bits = board << 36 & !Self::RANK_1 & !Self::RANK_2 & !Self::RANK_3 & !Self::RANK_4;
        board = board | next_bits;
        board
    }

    fn east_fill(self) -> Self {
        let mut board = self;
        board = board | board.east_one();
        board = board | board << 16;
        board = board | board << 32;
        board
    }

    fn south_east_fill(self) -> Self {
        let mut board = self;
        board = board | board.south_east_one();
        let next_bits = board << 14 & !Self::RANK_8 & !Self::RANK_7;
        board = board | next_bits;
        let next_bits = board << 28 & !Self::RANK_8 & !Self::RANK_7 & !Self::RANK_6 & !Self::RANK_5;
        board = board | next_bits;
        board
    }

    fn south_fill(self) -> Self {
        let mut board = self;
        board = board | board.south_one();
        let next_bits = board >> 2 & !Self::RANK_8 & !Self::RANK_7;
        board = board | next_bits;
        let next_bits = board >> 4 & !Self::RANK_8 & !Self::RANK_7 & !Self::RANK_6 & !Self::RANK_5;
        board = board | next_bits;
        board
    }

    fn south_west_fill(self) -> Self {
        let mut board = self;
        board = board | board.south_west_one();
        let next_bits = board >> 18 & !Self::RANK_8 & !Self::RANK_7;
        board = board | next_bits;
        let next_bits = board >> 36 & !Self::RANK_8 & !Self::RANK_7 & !Self::RANK_6 & !Self::RANK_5;
        board = board | next_bits;
        board
    }

    fn west_fill(self) -> Self {
        let mut board = self;
        board = board | board.west_one();
        board = board | board >> 16;
        board = board | board >> 32;
        board
    }

    fn north_west_fill(self) -> Self {
        let mut board = self;
        board = board | board.north_west_one();
        let next_bits = board >> 14 & !Self::RANK_1 & !Self::RANK_2;
        board = board | next_bits;
        let next_bits = board >> 28 & !Self::RANK_1 & !Self::RANK_2 & !Self::RANK_3 & !Self::RANK_4;
        board = board | next_bits;
        board
    }

    fn least_significant_1_bit(self) -> Self {
        debug_assert!(self != Self::EMPTY);
        self & Bitboard(0_u64.wrapping_sub(self.0))
    }

    fn below_least_significant_1_bit(self) -> Self {
        debug_assert!(self != Self::EMPTY);
        !self & Bitboard(self.0 - 1)
    }

    fn below_least_significant_1_bit_inclusive(self) -> Self {
        debug_assert!(self != Self::EMPTY);
        self ^ Bitboard(self.0 - 1)
    }

    fn above_least_significant_1_bit(self) -> Self {
        debug_assert!(self != Self::EMPTY);
        self ^ Bitboard(0_u64.wrapping_sub(self.0))
    }

    fn most_significant_1_bit(self) -> Self {
        debug_assert!(self != Self::EMPTY);
        Bitboard(self.below_most_significant_1_bit().0 + 1)
    }

    fn below_most_significant_1_bit(self) -> Self {
        debug_assert!(self != Self::EMPTY);
        self.below_most_significant_1_bit_inclusive() >> 1
    }

    fn below_most_significant_1_bit_inclusive(self) -> Self {
        debug_assert!(self != Self::EMPTY);
        let mut x = self.0;
        x |= x >> 1;
        x |= x >> 2;
        x |= x >> 4;
        x |= x >> 8;
        x |= x >> 16;
        x |= x >> 32;
        Bitboard(x)
    }

    fn bit_idx(self) -> usize {
        debug_assert!(self.0.count_ones() == 1);
        const DEBRUIJN_SEQ: u64 = 0x0218a392cd3d5dbf;
        const LEN_SEQ: usize = 64;
        const LEN_VAL: usize = 6;
        const BIT_IDX_LOOKUP: [u8; LEN_SEQ] = [
            0, 1, 2, 7, 3, 13, 8, 19, 4, 25, 14, 28, 9, 34, 20, 40, 5, 17, 26, 38, 15, 46, 29, 48,
            10, 31, 35, 54, 21, 50, 41, 57, 63, 6, 12, 18, 24, 27, 33, 39, 16, 37, 45, 47, 30, 53,
            49, 56, 62, 11, 23, 32, 36, 44, 52, 55, 61, 22, 43, 51, 60, 42, 59, 58,
        ];
        BIT_IDX_LOOKUP[DEBRUIJN_SEQ.wrapping_mul(self.0) as usize >> (LEN_SEQ - LEN_VAL)] as usize
    }

    pub fn bit_scan_forward(self) -> usize {
        debug_assert!(self != Bitboard::EMPTY);
        self.least_significant_1_bit().bit_idx()
    }

    fn bit_scan_forward_reset(&mut self) -> usize {
        debug_assert!(*self != Bitboard::EMPTY);
        let ls1b = self.least_significant_1_bit();
        let bit_idx = ls1b.bit_idx();
        *self = Bitboard(self.0 & self.0 - 1);
        bit_idx
    }

    pub fn bit_scan_reverse(self) -> usize {
        debug_assert!(self != Bitboard::EMPTY);
        self.most_significant_1_bit().bit_idx()
    }

    fn pop_count(self) -> usize {
        let mut x = self.0;
        x = x - ((x >> 1) & 0x5555555555555555);
        x = (x & 0x3333333333333333) + ((x >> 2) & 0x3333333333333333);
        x = (x + (x >> 4)) & 0x0f0f0f0f0f0f0f0f;
        x = x.wrapping_mul(0x0101010101010101);
        x >>= 56;
        x as usize
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl<'a> BitAnd<&'a Self> for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: &Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl<'a> BitAnd for &'a Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl<'a> BitAnd<Bitboard> for &'a Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl<'a> BitOr<&'a Self> for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: &Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl<'a> BitOr for &'a Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl<'a> BitOr<Bitboard> for &'a Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl<'a> BitXor<&'a Self> for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: &Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl<'a> BitXor for &'a Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl<'a> BitXor<Bitboard> for &'a Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl<'a> Not for &'a Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl Shl<usize> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl<'a> Shl<usize> for &'a Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<usize> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl<'a> Shr<usize> for &'a Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const EMPTY_SQUARE: u8 = b'-';
        const OCCUPIED_SQUARE: u8 = b'X';
        const SPACE: u8 = b' ';
        let mut squares_in_rank = [SPACE; 2 * Self::NUM_FILES - 1];
        for rank in (0..Self::NUM_RANKS).rev() {
            for file in 0..Self::NUM_FILES {
                let square = Self::to_square(rank, file);
                let square_bit = Bitboard(0x1) << square;
                squares_in_rank[2 * file] = match self & square_bit {
                    Self::EMPTY => EMPTY_SQUARE,
                    _ => OCCUPIED_SQUARE,
                };
            }
            let rank_str = str::from_utf8(&squares_in_rank).unwrap();
            writeln!(f, "{}", rank_str).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_file_square_indexing() {
        // A1
        assert_eq!(0, Bitboard::to_square(0, 0));
        assert_eq!(0, Bitboard::to_rank(0));
        assert_eq!(0, Bitboard::to_file(0));
        // A2
        assert_eq!(1, Bitboard::to_square(1, 0));
        assert_eq!(1, Bitboard::to_rank(1));
        assert_eq!(0, Bitboard::to_file(1));
        // B1
        assert_eq!(8, Bitboard::to_square(0, 1));
        assert_eq!(0, Bitboard::to_rank(8));
        assert_eq!(1, Bitboard::to_file(8));
        // B2
        assert_eq!(9, Bitboard::to_square(1, 1));
        assert_eq!(1, Bitboard::to_rank(9));
        assert_eq!(1, Bitboard::to_file(9));
        // E6
        assert_eq!(37, Bitboard::to_square(5, 4));
        assert_eq!(5, Bitboard::to_rank(37));
        assert_eq!(4, Bitboard::to_file(37));
        // F8
        assert_eq!(47, Bitboard::to_square(7, 5));
        assert_eq!(7, Bitboard::to_rank(47));
        assert_eq!(5, Bitboard::to_file(47));
        // H7
        assert_eq!(62, Bitboard::to_square(6, 7));
        assert_eq!(6, Bitboard::to_rank(62));
        assert_eq!(7, Bitboard::to_file(62));
        // H8
        assert_eq!(63, Bitboard::to_square(7, 7));
        assert_eq!(7, Bitboard::to_rank(63));
        assert_eq!(7, Bitboard::to_file(63));

        for square in 0..Bitboard::NUM_SQUARES {
            assert_eq!(
                square,
                Bitboard::to_square(Bitboard::to_rank(square), Bitboard::to_file(square))
            );
        }
    }

    #[test]
    fn north_one() {
        let board = Bitboard::RANK_1;
        let board = board.north_one();
        assert_eq!(Bitboard::RANK_2, board);
        let board = board.north_one();
        assert_eq!(Bitboard::RANK_3, board);
        let board = board.north_one();
        assert_eq!(Bitboard::RANK_4, board);
        let board = board.north_one();
        assert_eq!(Bitboard::RANK_5, board);
        let board = board.north_one();
        assert_eq!(Bitboard::RANK_6, board);
        let board = board.north_one();
        assert_eq!(Bitboard::RANK_7, board);
        let board = board.north_one();
        assert_eq!(Bitboard::RANK_8, board);
        let board = board.north_one();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn north_one_if_rank_8_empty() {
        let board = Bitboard::RANK_1;
        let board = board.north_one_if_rank_8_empty();
        assert_eq!(Bitboard::RANK_2, board);
        let board = board.north_one_if_rank_8_empty();
        assert_eq!(Bitboard::RANK_3, board);
        let board = board.north_one_if_rank_8_empty();
        assert_eq!(Bitboard::RANK_4, board);
        let board = board.north_one_if_rank_8_empty();
        assert_eq!(Bitboard::RANK_5, board);
        let board = board.north_one_if_rank_8_empty();
        assert_eq!(Bitboard::RANK_6, board);
        let board = board.north_one_if_rank_8_empty();
        assert_eq!(Bitboard::RANK_7, board);
        let board = board.north_one_if_rank_8_empty();
        assert_eq!(Bitboard::RANK_8, board);
    }

    #[test]
    fn north_east_one() {
        let board = Bitboard::FILE_A | Bitboard::RANK_1;
        assert_eq!(Bitboard(0x01010101010101ff), board);
        let board = board.north_east_one();
        assert_eq!(Bitboard(0x020202020202fe00), board);
        let board = board.north_east_one();
        assert_eq!(Bitboard(0x0404040404fc0000), board);
        let board = board.north_east_one();
        assert_eq!(Bitboard(0x08080808f8000000), board);
        let board = board.north_east_one();
        assert_eq!(Bitboard(0x101010f000000000), board);
        let board = board.north_east_one();
        assert_eq!(Bitboard(0x2020e00000000000), board);
        let board = board.north_east_one();
        assert_eq!(Bitboard(0x40c0000000000000), board);
        let board = board.north_east_one();
        assert_eq!(Bitboard(0x8000000000000000), board);
        let board = board.north_east_one();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn north_east_one_if_rank_8_empty() {
        let board = (Bitboard::FILE_A | Bitboard::RANK_1) & !Bitboard::RANK_8;
        assert_eq!(Bitboard(0x010101010101017f), board);
        let board = (board & !Bitboard::RANK_8).north_east_one_if_rank_8_empty();
        assert_eq!(Bitboard(0x020202020202fe00), board);
        let board = (board & !Bitboard::RANK_8).north_east_one_if_rank_8_empty();
        assert_eq!(Bitboard(0x0404040404fc0000), board);
        let board = (board & !Bitboard::RANK_8).north_east_one_if_rank_8_empty();
        assert_eq!(Bitboard(0x08080808f8000000), board);
        let board = (board & !Bitboard::RANK_8).north_east_one_if_rank_8_empty();
        assert_eq!(Bitboard(0x101010f000000000), board);
        let board = (board & !Bitboard::RANK_8).north_east_one_if_rank_8_empty();
        assert_eq!(Bitboard(0x2020e00000000000), board);
        let board = (board & !Bitboard::RANK_8).north_east_one_if_rank_8_empty();
        assert_eq!(Bitboard(0x40c0000000000000), board);
        let board = (board & !Bitboard::RANK_8).north_east_one_if_rank_8_empty();
        assert_eq!(Bitboard(0x8000000000000000), board);
    }

    #[test]
    fn east_one() {
        let board = Bitboard::FILE_A;
        let board = board.east_one();
        assert_eq!(Bitboard::FILE_B, board);
        let board = board.east_one();
        assert_eq!(Bitboard::FILE_C, board);
        let board = board.east_one();
        assert_eq!(Bitboard::FILE_D, board);
        let board = board.east_one();
        assert_eq!(Bitboard::FILE_E, board);
        let board = board.east_one();
        assert_eq!(Bitboard::FILE_F, board);
        let board = board.east_one();
        assert_eq!(Bitboard::FILE_G, board);
        let board = board.east_one();
        assert_eq!(Bitboard::FILE_H, board);
        let board = board.east_one();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn south_east_one() {
        let board = Bitboard::FILE_A | Bitboard::RANK_8;
        assert_eq!(Bitboard(0x80808080808080ff), board);
        let board = board.south_east_one();
        assert_eq!(Bitboard(0x4040404040407f00), board);
        let board = board.south_east_one();
        assert_eq!(Bitboard(0x20202020203f0000), board);
        let board = board.south_east_one();
        assert_eq!(Bitboard(0x101010101f000000), board);
        let board = board.south_east_one();
        assert_eq!(Bitboard(0x0808080f00000000), board);
        let board = board.south_east_one();
        assert_eq!(Bitboard(0x0404070000000000), board);
        let board = board.south_east_one();
        assert_eq!(Bitboard(0x0203000000000000), board);
        let board = board.south_east_one();
        assert_eq!(Bitboard(0x0100000000000000), board);
        let board = board.south_east_one();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn south_east_one_if_rank_1_empty() {
        let board = (Bitboard::FILE_A | Bitboard::RANK_8) & !Bitboard::RANK_1;
        assert_eq!(Bitboard(0x80808080808080fe), board);
        let board = (board & !Bitboard::RANK_1).south_east_one_if_rank_1_empty();
        assert_eq!(Bitboard(0x4040404040407f00), board);
        let board = (board & !Bitboard::RANK_1).south_east_one_if_rank_1_empty();
        assert_eq!(Bitboard(0x20202020203f0000), board);
        let board = (board & !Bitboard::RANK_1).south_east_one_if_rank_1_empty();
        assert_eq!(Bitboard(0x101010101f000000), board);
        let board = (board & !Bitboard::RANK_1).south_east_one_if_rank_1_empty();
        assert_eq!(Bitboard(0x0808080f00000000), board);
        let board = (board & !Bitboard::RANK_1).south_east_one_if_rank_1_empty();
        assert_eq!(Bitboard(0x0404070000000000), board);
        let board = (board & !Bitboard::RANK_1).south_east_one_if_rank_1_empty();
        assert_eq!(Bitboard(0x0203000000000000), board);
        let board = (board & !Bitboard::RANK_1).south_east_one_if_rank_1_empty();
        assert_eq!(Bitboard(0x0100000000000000), board);
    }

    #[test]
    fn south_one() {
        let board = Bitboard::RANK_8;
        let board = board.south_one();
        assert_eq!(Bitboard::RANK_7, board);
        let board = board.south_one();
        assert_eq!(Bitboard::RANK_6, board);
        let board = board.south_one();
        assert_eq!(Bitboard::RANK_5, board);
        let board = board.south_one();
        assert_eq!(Bitboard::RANK_4, board);
        let board = board.south_one();
        assert_eq!(Bitboard::RANK_3, board);
        let board = board.south_one();
        assert_eq!(Bitboard::RANK_2, board);
        let board = board.south_one();
        assert_eq!(Bitboard::RANK_1, board);
        let board = board.south_one();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn south_one_if_rank_1_empty() {
        let board = Bitboard::RANK_8;
        let board = board.south_one_if_rank_1_empty();
        assert_eq!(Bitboard::RANK_7, board);
        let board = board.south_one_if_rank_1_empty();
        assert_eq!(Bitboard::RANK_6, board);
        let board = board.south_one_if_rank_1_empty();
        assert_eq!(Bitboard::RANK_5, board);
        let board = board.south_one_if_rank_1_empty();
        assert_eq!(Bitboard::RANK_4, board);
        let board = board.south_one_if_rank_1_empty();
        assert_eq!(Bitboard::RANK_3, board);
        let board = board.south_one_if_rank_1_empty();
        assert_eq!(Bitboard::RANK_2, board);
        let board = board.south_one_if_rank_1_empty();
        assert_eq!(Bitboard::RANK_1, board);
    }

    #[test]
    fn south_west_one() {
        let board = Bitboard::FILE_H | Bitboard::RANK_8;
        assert_eq!(Bitboard(0xff80808080808080), board);
        let board = board.south_west_one();
        assert_eq!(Bitboard(0x007f404040404040), board);
        let board = board.south_west_one();
        assert_eq!(Bitboard(0x00003f2020202020), board);
        let board = board.south_west_one();
        assert_eq!(Bitboard(0x0000001f10101010), board);
        let board = board.south_west_one();
        assert_eq!(Bitboard(0x000000000f080808), board);
        let board = board.south_west_one();
        assert_eq!(Bitboard(0x0000000000070404), board);
        let board = board.south_west_one();
        assert_eq!(Bitboard(0x0000000000000302), board);
        let board = board.south_west_one();
        assert_eq!(Bitboard(0x0000000000000001), board);
        let board = board.south_west_one();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn south_west_one_if_rank_1_empty() {
        let board = (Bitboard::FILE_H | Bitboard::RANK_8) & !Bitboard::RANK_1;
        assert_eq!(Bitboard(0xfe80808080808080), board);
        let board = (board & !Bitboard::RANK_1).south_west_one_if_rank_1_empty();
        assert_eq!(Bitboard(0x007f404040404040), board);
        let board = (board & !Bitboard::RANK_1).south_west_one_if_rank_1_empty();
        assert_eq!(Bitboard(0x00003f2020202020), board);
        let board = (board & !Bitboard::RANK_1).south_west_one_if_rank_1_empty();
        assert_eq!(Bitboard(0x0000001f10101010), board);
        let board = (board & !Bitboard::RANK_1).south_west_one_if_rank_1_empty();
        assert_eq!(Bitboard(0x000000000f080808), board);
        let board = (board & !Bitboard::RANK_1).south_west_one_if_rank_1_empty();
        assert_eq!(Bitboard(0x0000000000070404), board);
        let board = (board & !Bitboard::RANK_1).south_west_one_if_rank_1_empty();
        assert_eq!(Bitboard(0x0000000000000302), board);
        let board = (board & !Bitboard::RANK_1).south_west_one_if_rank_1_empty();
        assert_eq!(Bitboard(0x0000000000000001), board);
    }

    #[test]
    fn west_one() {
        let board = Bitboard::FILE_H;
        let board = board.west_one();
        assert_eq!(Bitboard::FILE_G, board);
        let board = board.west_one();
        assert_eq!(Bitboard::FILE_F, board);
        let board = board.west_one();
        assert_eq!(Bitboard::FILE_E, board);
        let board = board.west_one();
        assert_eq!(Bitboard::FILE_D, board);
        let board = board.west_one();
        assert_eq!(Bitboard::FILE_C, board);
        let board = board.west_one();
        assert_eq!(Bitboard::FILE_B, board);
        let board = board.west_one();
        assert_eq!(Bitboard::FILE_A, board);
        let board = board.west_one();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn north_west_one() {
        let board = Bitboard::FILE_H | Bitboard::RANK_1;
        assert_eq!(Bitboard(0xff01010101010101), board);
        let board = board.north_west_one();
        assert_eq!(Bitboard(0x00fe020202020202), board);
        let board = board.north_west_one();
        assert_eq!(Bitboard(0x0000fc0404040404), board);
        let board = board.north_west_one();
        assert_eq!(Bitboard(0x000000f808080808), board);
        let board = board.north_west_one();
        assert_eq!(Bitboard(0x00000000f0101010), board);
        let board = board.north_west_one();
        assert_eq!(Bitboard(0x0000000000e02020), board);
        let board = board.north_west_one();
        assert_eq!(Bitboard(0x000000000000c040), board);
        let board = board.north_west_one();
        assert_eq!(Bitboard(0x0000000000000080), board);
        let board = board.north_west_one();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn north_west_one_if_rank_8_empty() {
        let board = (Bitboard::FILE_H | Bitboard::RANK_1) & !Bitboard::RANK_8;
        assert_eq!(Bitboard(0x7f01010101010101), board);
        let board = (board & !Bitboard::RANK_8).north_west_one_if_rank_8_empty();
        assert_eq!(Bitboard(0x00fe020202020202), board);
        let board = (board & !Bitboard::RANK_8).north_west_one_if_rank_8_empty();
        assert_eq!(Bitboard(0x0000fc0404040404), board);
        let board = (board & !Bitboard::RANK_8).north_west_one_if_rank_8_empty();
        assert_eq!(Bitboard(0x000000f808080808), board);
        let board = (board & !Bitboard::RANK_8).north_west_one_if_rank_8_empty();
        assert_eq!(Bitboard(0x00000000f0101010), board);
        let board = (board & !Bitboard::RANK_8).north_west_one_if_rank_8_empty();
        assert_eq!(Bitboard(0x0000000000e02020), board);
        let board = (board & !Bitboard::RANK_8).north_west_one_if_rank_8_empty();
        assert_eq!(Bitboard(0x000000000000c040), board);
        let board = (board & !Bitboard::RANK_8).north_west_one_if_rank_8_empty();
        assert_eq!(Bitboard(0x0000000000000080), board);
    }

    #[test]
    fn north_two_east_one() {
        let board = Bitboard::FILE_A | Bitboard::RANK_1;
        assert_eq!(Bitboard(0x01010101010101ff), board);
        let board = board.north_two_east_one();
        assert_eq!(Bitboard(0x040404040404fc00), board);
        let board = board.north_two_east_one();
        assert_eq!(Bitboard(0x1010101010f00000), board);
        let board = board.north_two_east_one();
        assert_eq!(Bitboard(0x40404040c0000000), board);
        let board = board.north_two_east_one();
        assert_eq!(Bitboard::EMPTY, board);
        let board = Bitboard::FILE_H | Bitboard::RANK_7 | Bitboard::RANK_8;
        assert_eq!(Bitboard(0xffc0c0c0c0c0c0c0), board);
        let board = board.north_two_east_one();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn north_one_east_two() {
        let board = Bitboard::FILE_A | Bitboard::RANK_1;
        assert_eq!(Bitboard(0x01010101010101ff), board);
        let board = board.north_one_east_two();
        assert_eq!(Bitboard(0x0202020202fe0000), board);
        let board = board.north_one_east_two();
        assert_eq!(Bitboard(0x040404fc00000000), board);
        let board = board.north_one_east_two();
        assert_eq!(Bitboard(0x08f8000000000000), board);
        let board = board.north_one_east_two();
        assert_eq!(Bitboard::EMPTY, board);
        let board = Bitboard::FILE_G | Bitboard::FILE_H | Bitboard::RANK_8;
        assert_eq!(Bitboard(0xffff808080808080), board);
        let board = board.north_one_east_two();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn south_one_east_two() {
        let board = Bitboard::FILE_A | Bitboard::RANK_8;
        assert_eq!(Bitboard(0x80808080808080ff), board);
        let board = board.south_one_east_two();
        assert_eq!(Bitboard(0x40404040407f0000), board);
        let board = board.south_one_east_two();
        assert_eq!(Bitboard(0x2020203f00000000), board);
        let board = board.south_one_east_two();
        assert_eq!(Bitboard(0x101f000000000000), board);
        let board = board.south_one_east_two();
        assert_eq!(Bitboard::EMPTY, board);
        let board = Bitboard::FILE_G | Bitboard::FILE_H | Bitboard::RANK_1;
        assert_eq!(Bitboard(0xffff010101010101), board);
        let board = board.south_one_east_two();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn south_two_east_one() {
        let board = Bitboard::FILE_A | Bitboard::RANK_8;
        assert_eq!(Bitboard(0x80808080808080ff), board);
        let board = board.south_two_east_one();
        assert_eq!(Bitboard(0x2020202020203f00), board);
        let board = board.south_two_east_one();
        assert_eq!(Bitboard(0x08080808080f0000), board);
        let board = board.south_two_east_one();
        assert_eq!(Bitboard(0x0202020203000000), board);
        let board = board.south_two_east_one();
        assert_eq!(Bitboard::EMPTY, board);
        let board = Bitboard::FILE_H | Bitboard::RANK_1 | Bitboard::RANK_2;
        assert_eq!(Bitboard(0xff03030303030303), board);
        let board = board.south_two_east_one();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn south_two_west_one() {
        let board = Bitboard::FILE_H | Bitboard::RANK_8;
        assert_eq!(Bitboard(0xff80808080808080), board);
        let board = board.south_two_west_one();
        assert_eq!(Bitboard(0x003f202020202020), board);
        let board = board.south_two_west_one();
        assert_eq!(Bitboard(0x00000f0808080808), board);
        let board = board.south_two_west_one();
        assert_eq!(Bitboard(0x0000000302020202), board);
        let board = board.south_two_west_one();
        assert_eq!(Bitboard::EMPTY, board);
        let board = Bitboard::FILE_A | Bitboard::RANK_1 | Bitboard::RANK_2;
        assert_eq!(Bitboard(0x03030303030303ff), board);
        let board = board.south_two_west_one();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn south_one_west_two() {
        let board = Bitboard::FILE_H | Bitboard::RANK_8;
        assert_eq!(Bitboard(0xff80808080808080), board);
        let board = board.south_one_west_two();
        assert_eq!(Bitboard(0x00007f4040404040), board);
        let board = board.south_one_west_two();
        assert_eq!(Bitboard(0x000000003f202020), board);
        let board = board.south_one_west_two();
        assert_eq!(Bitboard(0x0000000000001f10), board);
        let board = board.south_one_west_two();
        assert_eq!(Bitboard::EMPTY, board);
        let board = Bitboard::FILE_A | Bitboard::FILE_B | Bitboard::RANK_1;
        assert_eq!(Bitboard(0x010101010101ffff), board);
        let board = board.south_one_west_two();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn north_one_west_two() {
        let board = Bitboard::FILE_H | Bitboard::RANK_1;
        assert_eq!(Bitboard(0xff01010101010101), board);
        let board = board.north_one_west_two();
        assert_eq!(Bitboard(0x0000fe0202020202), board);
        let board = board.north_one_west_two();
        assert_eq!(Bitboard(0x00000000fc040404), board);
        let board = board.north_one_west_two();
        assert_eq!(Bitboard(0x000000000000f808), board);
        let board = board.north_one_west_two();
        assert_eq!(Bitboard::EMPTY, board);
        let board = Bitboard::FILE_A | Bitboard::FILE_B | Bitboard::RANK_8;
        assert_eq!(Bitboard(0x808080808080ffff), board);
        let board = board.north_one_west_two();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn north_two_west_one() {
        let board = Bitboard::FILE_H | Bitboard::RANK_1;
        assert_eq!(Bitboard(0xff01010101010101), board);
        let board = board.north_two_west_one();
        assert_eq!(Bitboard(0x00fc040404040404), board);
        let board = board.north_two_west_one();
        assert_eq!(Bitboard(0x0000f01010101010), board);
        let board = board.north_two_west_one();
        assert_eq!(Bitboard(0x000000c040404040), board);
        let board = board.north_two_west_one();
        assert_eq!(Bitboard::EMPTY, board);
        let board = Bitboard::FILE_A | Bitboard::RANK_7 | Bitboard::RANK_8;
        assert_eq!(Bitboard(0xc0c0c0c0c0c0c0ff), board);
        let board = board.north_two_west_one();
        assert_eq!(Bitboard::EMPTY, board);
    }

    #[test]
    fn north_fill() {
        let board = Bitboard::EMPTY;
        assert_eq!(Bitboard::EMPTY, board.north_fill());
        let board = Bitboard::A1;
        assert_eq!(Bitboard::FILE_A, board.north_fill());
        let board = Bitboard::A2;
        assert_eq!(Bitboard::FILE_A & !Bitboard::A1, board.north_fill());
        let board = Bitboard::A7;
        assert_eq!(Bitboard::A7 | Bitboard::A8, board.north_fill());
        let board = Bitboard::A8;
        assert_eq!(Bitboard::A8, board.north_fill());
        let board = Bitboard::B1;
        assert_eq!(Bitboard::FILE_B, board.north_fill());
        let board = Bitboard::G1 | Bitboard::H2;
        assert_eq!(
            Bitboard::FILE_G | Bitboard::FILE_H & !Bitboard::H1,
            board.north_fill()
        );
        let board = Bitboard::RANK_1;
        assert_eq!(Bitboard::UNIVERSE, board.north_fill());
    }

    #[test]
    fn north_east_fill() {
        let board = Bitboard::EMPTY;
        assert_eq!(Bitboard::EMPTY, board.north_east_fill());
        let board = Bitboard::A1;
        assert_eq!(
            Bitboard::A1
                | Bitboard::B2
                | Bitboard::C3
                | Bitboard::D4
                | Bitboard::E5
                | Bitboard::F6
                | Bitboard::G7
                | Bitboard::H8,
            board.north_east_fill()
        );
        let board = Bitboard::B1;
        assert_eq!(
            Bitboard::B1
                | Bitboard::C2
                | Bitboard::D3
                | Bitboard::E4
                | Bitboard::F5
                | Bitboard::G6
                | Bitboard::H7,
            board.north_east_fill()
        );
        let board = Bitboard::A7;
        assert_eq!(Bitboard::A7 | Bitboard::B8, board.north_east_fill());
        let board = Bitboard::RANK_8 | Bitboard::FILE_H;
        assert_eq!(Bitboard::RANK_8 | Bitboard::FILE_H, board.north_east_fill());
        let board = Bitboard::RANK_1 | Bitboard::FILE_A;
        assert_eq!(Bitboard::UNIVERSE, board.north_east_fill());
    }

    #[test]
    fn east_fill() {
        let board = Bitboard::EMPTY;
        assert_eq!(Bitboard::EMPTY, board.east_fill());
        let board = Bitboard::A1;
        assert_eq!(Bitboard::RANK_1, board.east_fill());
        let board = Bitboard::B1;
        assert_eq!(Bitboard::RANK_1 & !Bitboard::A1, board.east_fill());
        let board = Bitboard::G1;
        assert_eq!(Bitboard::G1 | Bitboard::H1, board.east_fill());
        let board = Bitboard::H1;
        assert_eq!(Bitboard::H1, board.east_fill());
        let board = Bitboard::A2;
        assert_eq!(Bitboard::RANK_2, board.east_fill());
        let board = Bitboard::A7 | Bitboard::B8;
        assert_eq!(
            Bitboard::RANK_7 | Bitboard::RANK_8 & !Bitboard::A8,
            board.east_fill()
        );
        let board = Bitboard::FILE_A;
        assert_eq!(Bitboard::UNIVERSE, board.east_fill());
    }

    #[test]
    fn south_east_fill() {
        let board = Bitboard::EMPTY;
        assert_eq!(Bitboard::EMPTY, board.south_east_fill());
        let board = Bitboard::A8;
        assert_eq!(
            Bitboard::A8
                | Bitboard::B7
                | Bitboard::C6
                | Bitboard::D5
                | Bitboard::E4
                | Bitboard::F3
                | Bitboard::G2
                | Bitboard::H1,
            board.south_east_fill()
        );
        let board = Bitboard::B8;
        assert_eq!(
            Bitboard::B8
                | Bitboard::C7
                | Bitboard::D6
                | Bitboard::E5
                | Bitboard::F4
                | Bitboard::G3
                | Bitboard::H2,
            board.south_east_fill()
        );
        let board = Bitboard::A2;
        assert_eq!(Bitboard::A2 | Bitboard::B1, board.south_east_fill());
        let board = Bitboard::RANK_1 | Bitboard::FILE_H;
        assert_eq!(Bitboard::RANK_1 | Bitboard::FILE_H, board.south_east_fill());
        let board = Bitboard::RANK_8 | Bitboard::FILE_A;
        assert_eq!(Bitboard::UNIVERSE, board.south_east_fill());
    }

    #[test]
    fn south_fill() {
        let board = Bitboard::EMPTY;
        assert_eq!(Bitboard::EMPTY, board.south_fill());
        let board = Bitboard::A8;
        assert_eq!(Bitboard::FILE_A, board.south_fill());
        let board = Bitboard::A7;
        assert_eq!(Bitboard::FILE_A & !Bitboard::A8, board.south_fill());
        let board = Bitboard::A2;
        assert_eq!(Bitboard::A1 | Bitboard::A2, board.south_fill());
        let board = Bitboard::A1;
        assert_eq!(Bitboard::A1, board.south_fill());
        let board = Bitboard::B8;
        assert_eq!(Bitboard::FILE_B, board.south_fill());
        let board = Bitboard::G8 | Bitboard::H7;
        assert_eq!(
            Bitboard::FILE_G | Bitboard::FILE_H & !Bitboard::H8,
            board.south_fill()
        );
        let board = Bitboard::RANK_8;
        assert_eq!(Bitboard::UNIVERSE, board.south_fill());
    }

    #[test]
    fn south_west_fill() {
        let board = Bitboard::EMPTY;
        assert_eq!(Bitboard::EMPTY, board.south_west_fill());
        let board = Bitboard::H8;
        assert_eq!(
            Bitboard::H8
                | Bitboard::G7
                | Bitboard::F6
                | Bitboard::E5
                | Bitboard::D4
                | Bitboard::C3
                | Bitboard::B2
                | Bitboard::A1,
            board.south_west_fill()
        );
        let board = Bitboard::G8;
        assert_eq!(
            Bitboard::G8
                | Bitboard::F7
                | Bitboard::E6
                | Bitboard::D5
                | Bitboard::C4
                | Bitboard::B3
                | Bitboard::A2,
            board.south_west_fill()
        );
        let board = Bitboard::H2;
        assert_eq!(Bitboard::H2 | Bitboard::G1, board.south_west_fill());
        let board = Bitboard::RANK_1 | Bitboard::FILE_A;
        assert_eq!(Bitboard::RANK_1 | Bitboard::FILE_A, board.south_west_fill());
        let board = Bitboard::RANK_8 | Bitboard::FILE_H;
        assert_eq!(Bitboard::UNIVERSE, board.south_west_fill());
    }

    #[test]
    fn west_fill() {
        let board = Bitboard::EMPTY;
        assert_eq!(Bitboard::EMPTY, board.west_fill());
        let board = Bitboard::H1;
        assert_eq!(Bitboard::RANK_1, board.west_fill());
        let board = Bitboard::G1;
        assert_eq!(Bitboard::RANK_1 & !Bitboard::H1, board.west_fill());
        let board = Bitboard::B1;
        assert_eq!(Bitboard::A1 | Bitboard::B1, board.west_fill());
        let board = Bitboard::A1;
        assert_eq!(Bitboard::A1, board.west_fill());
        let board = Bitboard::H2;
        assert_eq!(Bitboard::RANK_2, board.west_fill());
        let board = Bitboard::H7 | Bitboard::G8;
        assert_eq!(
            Bitboard::RANK_7 | Bitboard::RANK_8 & !Bitboard::H8,
            board.west_fill()
        );
        let board = Bitboard::FILE_H;
        assert_eq!(Bitboard::UNIVERSE, board.west_fill());
    }

    #[test]
    fn north_west_fill() {
        let board = Bitboard::EMPTY;
        assert_eq!(Bitboard::EMPTY, board.north_west_fill());
        let board = Bitboard::H1;
        assert_eq!(
            Bitboard::H1
                | Bitboard::G2
                | Bitboard::F3
                | Bitboard::E4
                | Bitboard::D5
                | Bitboard::C6
                | Bitboard::B7
                | Bitboard::A8,
            board.north_west_fill()
        );
        let board = Bitboard::G1;
        assert_eq!(
            Bitboard::G1
                | Bitboard::F2
                | Bitboard::E3
                | Bitboard::D4
                | Bitboard::C5
                | Bitboard::B6
                | Bitboard::A7,
            board.north_west_fill()
        );
        let board = Bitboard::H7;
        assert_eq!(Bitboard::H7 | Bitboard::G8, board.north_west_fill());
        let board = Bitboard::RANK_8 | Bitboard::FILE_A;
        assert_eq!(Bitboard::RANK_8 | Bitboard::FILE_A, board.north_west_fill());
        let board = Bitboard::RANK_1 | Bitboard::FILE_H;
        assert_eq!(Bitboard::UNIVERSE, board.north_west_fill());
    }

    #[test]
    fn least_significant_1_bit() {
        assert_eq!(
            Bitboard(0x1),
            Bitboard::least_significant_1_bit(Bitboard(0x1))
        );
        assert_eq!(
            Bitboard(0x2),
            Bitboard::least_significant_1_bit(Bitboard(0x2))
        );
        assert_eq!(
            Bitboard(0x1),
            Bitboard::least_significant_1_bit(Bitboard(0x3))
        );
        assert_eq!(
            Bitboard(0x4),
            Bitboard::least_significant_1_bit(Bitboard(0x4))
        );
        assert_eq!(
            Bitboard(0x1),
            Bitboard::least_significant_1_bit(Bitboard(0x5))
        );
        assert_eq!(
            Bitboard(0x2),
            Bitboard::least_significant_1_bit(Bitboard(0x6))
        );
        assert_eq!(
            Bitboard(0x1),
            Bitboard::least_significant_1_bit(Bitboard(0x7))
        );
        assert_eq!(
            Bitboard(0x8),
            Bitboard::least_significant_1_bit(Bitboard(0x8))
        );
        assert_eq!(
            Bitboard(0x8000000000000000),
            Bitboard::least_significant_1_bit(Bitboard(0x8000000000000000))
        );
        assert_eq!(
            Bitboard(0x0000000080000000),
            Bitboard::least_significant_1_bit(Bitboard(0x8000000080000000))
        );
        assert_eq!(
            Bitboard(0x0000000000000001),
            Bitboard::least_significant_1_bit(Bitboard(0xffffffffffffffff))
        );
    }

    #[test]
    fn below_least_significant_1_bit() {
        assert_eq!(
            Bitboard(0x0),
            Bitboard::below_least_significant_1_bit(Bitboard(0x1))
        );
        assert_eq!(
            Bitboard(0x1),
            Bitboard::below_least_significant_1_bit(Bitboard(0x2))
        );
        assert_eq!(
            Bitboard(0x0),
            Bitboard::below_least_significant_1_bit(Bitboard(0x3))
        );
        assert_eq!(
            Bitboard(0x3),
            Bitboard::below_least_significant_1_bit(Bitboard(0x4))
        );
        assert_eq!(
            Bitboard(0x0),
            Bitboard::below_least_significant_1_bit(Bitboard(0x5))
        );
        assert_eq!(
            Bitboard(0x1),
            Bitboard::below_least_significant_1_bit(Bitboard(0x6))
        );
        assert_eq!(
            Bitboard(0x0),
            Bitboard::below_least_significant_1_bit(Bitboard(0x7))
        );
        assert_eq!(
            Bitboard(0x7),
            Bitboard::below_least_significant_1_bit(Bitboard(0x8))
        );
        assert_eq!(
            Bitboard(0x7fffffffffffffff),
            Bitboard::below_least_significant_1_bit(Bitboard(0x8000000000000000))
        );
        assert_eq!(
            Bitboard(0x000000007fffffff),
            Bitboard::below_least_significant_1_bit(Bitboard(0x8000000080000000))
        );
        assert_eq!(
            Bitboard(0x0000000000000000),
            Bitboard::below_least_significant_1_bit(Bitboard(0xffffffffffffffff))
        );
    }

    #[test]
    fn below_least_significant_1_bit_inclusive() {
        assert_eq!(
            Bitboard(0x1),
            Bitboard::below_least_significant_1_bit_inclusive(Bitboard(0x1))
        );
        assert_eq!(
            Bitboard(0x3),
            Bitboard::below_least_significant_1_bit_inclusive(Bitboard(0x2))
        );
        assert_eq!(
            Bitboard(0x1),
            Bitboard::below_least_significant_1_bit_inclusive(Bitboard(0x3))
        );
        assert_eq!(
            Bitboard(0x7),
            Bitboard::below_least_significant_1_bit_inclusive(Bitboard(0x4))
        );
        assert_eq!(
            Bitboard(0x1),
            Bitboard::below_least_significant_1_bit_inclusive(Bitboard(0x5))
        );
        assert_eq!(
            Bitboard(0x3),
            Bitboard::below_least_significant_1_bit_inclusive(Bitboard(0x6))
        );
        assert_eq!(
            Bitboard(0x1),
            Bitboard::below_least_significant_1_bit_inclusive(Bitboard(0x7))
        );
        assert_eq!(
            Bitboard(0xf),
            Bitboard::below_least_significant_1_bit_inclusive(Bitboard(0x8))
        );
        assert_eq!(
            Bitboard(0xffffffffffffffff),
            Bitboard::below_least_significant_1_bit_inclusive(Bitboard(0x8000000000000000))
        );
        assert_eq!(
            Bitboard(0x00000000ffffffff),
            Bitboard::below_least_significant_1_bit_inclusive(Bitboard(0x8000000080000000))
        );
        assert_eq!(
            Bitboard(0x0000000000000001),
            Bitboard::below_least_significant_1_bit_inclusive(Bitboard(0xffffffffffffffff))
        );
    }

    #[test]
    fn above_least_significant_1_bit() {
        assert_eq!(
            Bitboard(0xfffffffffffffffe),
            Bitboard::above_least_significant_1_bit(Bitboard(0x1))
        );
        assert_eq!(
            Bitboard(0xfffffffffffffffc),
            Bitboard::above_least_significant_1_bit(Bitboard(0x2))
        );
        assert_eq!(
            Bitboard(0xfffffffffffffffe),
            Bitboard::above_least_significant_1_bit(Bitboard(0x3))
        );
        assert_eq!(
            Bitboard(0xfffffffffffffff8),
            Bitboard::above_least_significant_1_bit(Bitboard(0x4))
        );
        assert_eq!(
            Bitboard(0xfffffffffffffffe),
            Bitboard::above_least_significant_1_bit(Bitboard(0x5))
        );
        assert_eq!(
            Bitboard(0xfffffffffffffffc),
            Bitboard::above_least_significant_1_bit(Bitboard(0x6))
        );
        assert_eq!(
            Bitboard(0xfffffffffffffffe),
            Bitboard::above_least_significant_1_bit(Bitboard(0x7))
        );
        assert_eq!(
            Bitboard(0xfffffffffffffff0),
            Bitboard::above_least_significant_1_bit(Bitboard(0x8))
        );
        assert_eq!(
            Bitboard(0x0),
            Bitboard::above_least_significant_1_bit(Bitboard(0x8000000000000000))
        );
        assert_eq!(
            Bitboard(0xffffffff00000000),
            Bitboard::above_least_significant_1_bit(Bitboard(0x8000000080000000))
        );
        assert_eq!(
            Bitboard(0xfffffffffffffffe),
            Bitboard::above_least_significant_1_bit(Bitboard(0xffffffffffffffff))
        );
    }

    #[test]
    fn most_significant_1_bit() {
        assert_eq!(
            Bitboard(0x1),
            Bitboard::most_significant_1_bit(Bitboard(0x1))
        );
        assert_eq!(
            Bitboard(0x2),
            Bitboard::most_significant_1_bit(Bitboard(0x2))
        );
        assert_eq!(
            Bitboard(0x2),
            Bitboard::most_significant_1_bit(Bitboard(0x3))
        );
        assert_eq!(
            Bitboard(0x4),
            Bitboard::most_significant_1_bit(Bitboard(0x4))
        );
        assert_eq!(
            Bitboard(0x4),
            Bitboard::most_significant_1_bit(Bitboard(0x5))
        );
        assert_eq!(
            Bitboard(0x4),
            Bitboard::most_significant_1_bit(Bitboard(0x6))
        );
        assert_eq!(
            Bitboard(0x4),
            Bitboard::most_significant_1_bit(Bitboard(0x7))
        );
        assert_eq!(
            Bitboard(0x8),
            Bitboard::most_significant_1_bit(Bitboard(0x8))
        );
        assert_eq!(
            Bitboard(0x8000000000000000),
            Bitboard::most_significant_1_bit(Bitboard(0x8000000000000000))
        );
        assert_eq!(
            Bitboard(0x8000000000000000),
            Bitboard::most_significant_1_bit(Bitboard(0x8000000080000000))
        );
        assert_eq!(
            Bitboard(0x8000000000000000),
            Bitboard::most_significant_1_bit(Bitboard(0xffffffffffffffff))
        );
    }

    #[test]
    fn below_most_significant_1_bit() {
        assert_eq!(
            Bitboard(0x0),
            Bitboard::below_most_significant_1_bit(Bitboard(0x1))
        );
        assert_eq!(
            Bitboard(0x1),
            Bitboard::below_most_significant_1_bit(Bitboard(0x2))
        );
        assert_eq!(
            Bitboard(0x1),
            Bitboard::below_most_significant_1_bit(Bitboard(0x3))
        );
        assert_eq!(
            Bitboard(0x3),
            Bitboard::below_most_significant_1_bit(Bitboard(0x4))
        );
        assert_eq!(
            Bitboard(0x3),
            Bitboard::below_most_significant_1_bit(Bitboard(0x5))
        );
        assert_eq!(
            Bitboard(0x3),
            Bitboard::below_most_significant_1_bit(Bitboard(0x6))
        );
        assert_eq!(
            Bitboard(0x3),
            Bitboard::below_most_significant_1_bit(Bitboard(0x7))
        );
        assert_eq!(
            Bitboard(0x7),
            Bitboard::below_most_significant_1_bit(Bitboard(0x8))
        );
        assert_eq!(
            Bitboard(0x7fffffffffffffff),
            Bitboard::below_most_significant_1_bit(Bitboard(0x8000000000000000))
        );
        assert_eq!(
            Bitboard(0x7fffffffffffffff),
            Bitboard::below_most_significant_1_bit(Bitboard(0x8000000080000000))
        );
        assert_eq!(
            Bitboard(0x7fffffffffffffff),
            Bitboard::below_most_significant_1_bit(Bitboard(0xffffffffffffffff))
        );
    }

    #[test]
    fn below_most_significant_1_bit_inclusive() {
        assert_eq!(
            Bitboard(0x1),
            Bitboard::below_most_significant_1_bit_inclusive(Bitboard(0x1))
        );
        assert_eq!(
            Bitboard(0x3),
            Bitboard::below_most_significant_1_bit_inclusive(Bitboard(0x2))
        );
        assert_eq!(
            Bitboard(0x3),
            Bitboard::below_most_significant_1_bit_inclusive(Bitboard(0x3))
        );
        assert_eq!(
            Bitboard(0x7),
            Bitboard::below_most_significant_1_bit_inclusive(Bitboard(0x4))
        );
        assert_eq!(
            Bitboard(0x7),
            Bitboard::below_most_significant_1_bit_inclusive(Bitboard(0x5))
        );
        assert_eq!(
            Bitboard(0x7),
            Bitboard::below_most_significant_1_bit_inclusive(Bitboard(0x6))
        );
        assert_eq!(
            Bitboard(0x7),
            Bitboard::below_most_significant_1_bit_inclusive(Bitboard(0x7))
        );
        assert_eq!(
            Bitboard(0xf),
            Bitboard::below_most_significant_1_bit_inclusive(Bitboard(0x8))
        );
        assert_eq!(
            Bitboard(0xffffffffffffffff),
            Bitboard::below_most_significant_1_bit_inclusive(Bitboard(0x8000000000000000))
        );
        assert_eq!(
            Bitboard(0xffffffffffffffff),
            Bitboard::below_most_significant_1_bit_inclusive(Bitboard(0x8000000080000000))
        );
        assert_eq!(
            Bitboard(0xffffffffffffffff),
            Bitboard::below_most_significant_1_bit_inclusive(Bitboard(0xffffffffffffffff))
        );
    }

    #[test]
    fn bit_idx() {
        for idx in 0..64 {
            let bit: u64 = 1 << idx;
            assert_eq!(idx, Bitboard::bit_idx(Bitboard(bit)));
        }
    }

    #[test]
    fn bit_scan_forward() {
        assert_eq!(0, Bitboard::bit_scan_forward(Bitboard(0x1)));
        assert_eq!(1, Bitboard::bit_scan_forward(Bitboard(0x2)));
        assert_eq!(0, Bitboard::bit_scan_forward(Bitboard(0x3)));
        assert_eq!(2, Bitboard::bit_scan_forward(Bitboard(0x4)));
        assert_eq!(0, Bitboard::bit_scan_forward(Bitboard(0x5)));
        assert_eq!(1, Bitboard::bit_scan_forward(Bitboard(0x6)));
        assert_eq!(0, Bitboard::bit_scan_forward(Bitboard(0x7)));
        assert_eq!(3, Bitboard::bit_scan_forward(Bitboard(0x8)));
        assert_eq!(63, Bitboard::bit_scan_forward(Bitboard(0x8000000000000000)));
        assert_eq!(31, Bitboard::bit_scan_forward(Bitboard(0x8000000080000000)));
        assert_eq!(0, Bitboard::bit_scan_forward(Bitboard(0xffffffffffffffff)));
    }

    #[test]
    fn bit_scan_forward_reset() {
        let mut bb = Bitboard(0x1);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(0, bit_idx);
        assert_eq!(Bitboard::EMPTY, bb);

        let mut bb = Bitboard(0x2);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(1, bit_idx);
        assert_eq!(Bitboard::EMPTY, bb);

        let mut bb = Bitboard(0x3);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(0, bit_idx);
        assert_eq!(Bitboard(0x2), bb);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(1, bit_idx);
        assert_eq!(Bitboard::EMPTY, bb);

        let mut bb = Bitboard(0x4);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(2, bit_idx);
        assert_eq!(Bitboard::EMPTY, bb);

        let mut bb = Bitboard(0x5);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(0, bit_idx);
        assert_eq!(Bitboard(0x4), bb);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(2, bit_idx);
        assert_eq!(Bitboard::EMPTY, bb);

        let mut bb = Bitboard(0x6);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(1, bit_idx);
        assert_eq!(Bitboard(0x4), bb);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(2, bit_idx);
        assert_eq!(Bitboard::EMPTY, bb);

        let mut bb = Bitboard(0x7);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(0, bit_idx);
        assert_eq!(Bitboard(0x6), bb);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(1, bit_idx);
        assert_eq!(Bitboard(0x4), bb);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(2, bit_idx);
        assert_eq!(Bitboard::EMPTY, bb);

        let mut bb = Bitboard(0x8);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(3, bit_idx);
        assert_eq!(Bitboard::EMPTY, bb);

        let mut bb = Bitboard(0x8000000000000000);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(63, bit_idx);
        assert_eq!(Bitboard::EMPTY, bb);

        let mut bb = Bitboard(0x8000000080000000);
        let bit_idx = bb.bit_scan_forward_reset();
        assert_eq!(31, bit_idx);
        assert_eq!(Bitboard(0x8000000000000000), bb);

        let mut bb = Bitboard(0xffffffffffffffff);
        for expected_idx in 0..64 {
            let bit_idx = bb.bit_scan_forward_reset();
            assert_eq!(expected_idx, bit_idx);
        }
        assert_eq!(Bitboard::EMPTY, bb);
    }

    #[test]
    fn bit_scan_reverse() {
        assert_eq!(0, Bitboard::bit_scan_reverse(Bitboard(0x1)));
        assert_eq!(1, Bitboard::bit_scan_reverse(Bitboard(0x2)));
        assert_eq!(1, Bitboard::bit_scan_reverse(Bitboard(0x3)));
        assert_eq!(2, Bitboard::bit_scan_reverse(Bitboard(0x4)));
        assert_eq!(2, Bitboard::bit_scan_reverse(Bitboard(0x5)));
        assert_eq!(2, Bitboard::bit_scan_reverse(Bitboard(0x6)));
        assert_eq!(2, Bitboard::bit_scan_reverse(Bitboard(0x7)));
        assert_eq!(3, Bitboard::bit_scan_reverse(Bitboard(0x8)));
        assert_eq!(63, Bitboard::bit_scan_reverse(Bitboard(0x8000000000000000)));
        assert_eq!(63, Bitboard::bit_scan_reverse(Bitboard(0x8000000080000000)));
        assert_eq!(63, Bitboard::bit_scan_reverse(Bitboard(0xffffffffffffffff)));
    }

    #[test]
    fn pop_count() {
        assert_eq!(0, Bitboard::pop_count(Bitboard(0x0000000000000000)));
        assert_eq!(1, Bitboard::pop_count(Bitboard(0x0000000000000001)));
        assert_eq!(1, Bitboard::pop_count(Bitboard(0x0000000000000002)));
        assert_eq!(2, Bitboard::pop_count(Bitboard(0x0000000000000003)));
        assert_eq!(1, Bitboard::pop_count(Bitboard(0x0000000000000004)));
        assert_eq!(8, Bitboard::pop_count(Bitboard(0x00000000000000ff)));
        assert_eq!(1, Bitboard::pop_count(Bitboard(0x0000000000000100)));
        assert_eq!(32, Bitboard::pop_count(Bitboard(0xfedcba9876543210)));
        assert_eq!(64, Bitboard::pop_count(Bitboard(0xffffffffffffffff)));
    }

    #[test]
    fn fmt() {
        let expected_str = "\
            - - X - - - - -\n\
            - - X - - - - -\n\
            - - X - - - - -\n\
            - - X - - - - -\n\
            - - X - - - - -\n\
            X X X X X X X X\n\
            - - X - - - - -\n\
            - - X - - - - -\n\
        ";
        assert_eq!(
            expected_str,
            format!("{}", Bitboard::RANK_3 | Bitboard::FILE_C)
        );
    }
}
