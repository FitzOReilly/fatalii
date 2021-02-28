use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rank(u8);

impl Rank {
    pub const NUM_RANKS: usize = 8;

    pub const R1: Self = Rank(0);
    pub const R2: Self = Rank(1);
    pub const R3: Self = Rank(2);
    pub const R4: Self = Rank(3);
    pub const R5: Self = Rank(4);
    pub const R6: Self = Rank(5);
    pub const R7: Self = Rank(6);
    pub const R8: Self = Rank(7);

    pub fn from_idx(idx: usize) -> Rank {
        debug_assert!(idx < Self::NUM_RANKS);
        Rank(idx as u8)
    }

    pub const fn idx(&self) -> usize {
        self.0 as usize
    }

    pub fn from_ascii(c: u8) -> Result<Self, String> {
        match c {
            b'1'..=b'8' => Ok(Rank::from_idx((c - b'1') as usize)),
            _ => Err(format!("Invalid rank `{}`", c as char)),
        }
    }

    pub fn to_ascii(&self) -> u8 {
        debug_assert!(self.idx() < Self::NUM_RANKS);
        self.0 + b'1'
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank_char = self.to_ascii() as char;
        write!(f, "{}", rank_char).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_idx() {
        assert_eq!(Rank::R1, Rank::from_idx(0));
        assert_eq!(Rank::R2, Rank::from_idx(1));
        assert_eq!(Rank::R3, Rank::from_idx(2));
        assert_eq!(Rank::R4, Rank::from_idx(3));
        assert_eq!(Rank::R5, Rank::from_idx(4));
        assert_eq!(Rank::R6, Rank::from_idx(5));
        assert_eq!(Rank::R7, Rank::from_idx(6));
        assert_eq!(Rank::R8, Rank::from_idx(7));
    }

    #[test]
    fn idx() {
        assert_eq!(0, Rank::R1.idx());
        assert_eq!(1, Rank::R2.idx());
        assert_eq!(2, Rank::R3.idx());
        assert_eq!(3, Rank::R4.idx());
        assert_eq!(4, Rank::R5.idx());
        assert_eq!(5, Rank::R6.idx());
        assert_eq!(6, Rank::R7.idx());
        assert_eq!(7, Rank::R8.idx());
    }

    #[test]
    fn from_ascii() {
        assert_eq!(Ok(Rank::R1), Rank::from_ascii(b'1'));
        assert_eq!(Ok(Rank::R2), Rank::from_ascii(b'2'));
        assert_eq!(Ok(Rank::R3), Rank::from_ascii(b'3'));
        assert_eq!(Ok(Rank::R4), Rank::from_ascii(b'4'));
        assert_eq!(Ok(Rank::R5), Rank::from_ascii(b'5'));
        assert_eq!(Ok(Rank::R6), Rank::from_ascii(b'6'));
        assert_eq!(Ok(Rank::R7), Rank::from_ascii(b'7'));
        assert_eq!(Ok(Rank::R8), Rank::from_ascii(b'8'));
        assert_eq!(
            Err(String::from("Invalid rank `9`")),
            Rank::from_ascii(b'9')
        );
    }

    #[test]
    fn to_ascii() {
        assert_eq!(b'1', Rank::R1.to_ascii());
        assert_eq!(b'2', Rank::R2.to_ascii());
        assert_eq!(b'3', Rank::R3.to_ascii());
        assert_eq!(b'4', Rank::R4.to_ascii());
        assert_eq!(b'5', Rank::R5.to_ascii());
        assert_eq!(b'6', Rank::R6.to_ascii());
        assert_eq!(b'7', Rank::R7.to_ascii());
        assert_eq!(b'8', Rank::R8.to_ascii());
    }

    #[test]
    fn fmt() {
        assert_eq!("1", format!("{}", Rank::R1));
        assert_eq!("2", format!("{}", Rank::R2));
        assert_eq!("3", format!("{}", Rank::R3));
        assert_eq!("4", format!("{}", Rank::R4));
        assert_eq!("5", format!("{}", Rank::R5));
        assert_eq!("6", format!("{}", Rank::R6));
        assert_eq!("7", format!("{}", Rank::R7));
        assert_eq!("8", format!("{}", Rank::R8));
    }
}
