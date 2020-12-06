use std::ops::Not;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Side {
    White = 0,
    Black = 1,
}

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl<'a> Not for &'a Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Self::Output::White => Self::Output::Black,
            Self::Output::Black => Self::Output::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not() {
        assert_eq!(Side::Black, !Side::White);
        assert_eq!(Side::White, !Side::Black);
        assert_eq!(Side::Black, !&Side::White);
        assert_eq!(Side::White, !&Side::Black);
    }
}
