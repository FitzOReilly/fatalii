use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct File(u8);

impl File {
    pub const NUM_FILES: usize = 8;

    pub const A: Self = File(0);
    pub const B: Self = File(1);
    pub const C: Self = File(2);
    pub const D: Self = File(3);
    pub const E: Self = File(4);
    pub const F: Self = File(5);
    pub const G: Self = File(6);
    pub const H: Self = File(7);

    pub fn from_idx(idx: usize) -> File {
        debug_assert!(idx < Self::NUM_FILES);
        File(idx as u8)
    }

    pub const fn idx(&self) -> usize {
        self.0 as usize
    }

    pub fn from_ascii(c: u8) -> Result<Self, String> {
        match c {
            b'a'..=b'h' => Ok(File::from_idx((c - b'a') as usize)),
            _ => Err(format!("Invalid file `{}`", c as char)),
        }
    }

    pub fn to_ascii(self) -> u8 {
        debug_assert!(self.idx() < Self::NUM_FILES);
        self.0 + b'a'
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let file_char = self.to_ascii() as char;
        write!(f, "{}", file_char).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_idx() {
        assert_eq!(File::A, File::from_idx(0));
        assert_eq!(File::B, File::from_idx(1));
        assert_eq!(File::C, File::from_idx(2));
        assert_eq!(File::D, File::from_idx(3));
        assert_eq!(File::E, File::from_idx(4));
        assert_eq!(File::F, File::from_idx(5));
        assert_eq!(File::G, File::from_idx(6));
        assert_eq!(File::H, File::from_idx(7));
    }

    #[test]
    fn idx() {
        assert_eq!(0, File::A.idx());
        assert_eq!(1, File::B.idx());
        assert_eq!(2, File::C.idx());
        assert_eq!(3, File::D.idx());
        assert_eq!(4, File::E.idx());
        assert_eq!(5, File::F.idx());
        assert_eq!(6, File::G.idx());
        assert_eq!(7, File::H.idx());
    }

    #[test]
    fn from_ascii() {
        assert_eq!(Ok(File::A), File::from_ascii(b'a'));
        assert_eq!(Ok(File::B), File::from_ascii(b'b'));
        assert_eq!(Ok(File::C), File::from_ascii(b'c'));
        assert_eq!(Ok(File::D), File::from_ascii(b'd'));
        assert_eq!(Ok(File::E), File::from_ascii(b'e'));
        assert_eq!(Ok(File::F), File::from_ascii(b'f'));
        assert_eq!(Ok(File::G), File::from_ascii(b'g'));
        assert_eq!(Ok(File::H), File::from_ascii(b'h'));
        assert_eq!(
            Err(String::from("Invalid file `i`")),
            File::from_ascii(b'i')
        );
    }

    #[test]
    fn to_ascii() {
        assert_eq!(b'a', File::A.to_ascii());
        assert_eq!(b'b', File::B.to_ascii());
        assert_eq!(b'c', File::C.to_ascii());
        assert_eq!(b'd', File::D.to_ascii());
        assert_eq!(b'e', File::E.to_ascii());
        assert_eq!(b'f', File::F.to_ascii());
        assert_eq!(b'g', File::G.to_ascii());
        assert_eq!(b'h', File::H.to_ascii());
    }

    #[test]
    fn fmt() {
        assert_eq!("a", format!("{}", File::A));
        assert_eq!("b", format!("{}", File::B));
        assert_eq!("c", format!("{}", File::C));
        assert_eq!("d", format!("{}", File::D));
        assert_eq!("e", format!("{}", File::E));
        assert_eq!("f", format!("{}", File::F));
        assert_eq!("g", format!("{}", File::G));
        assert_eq!("h", format!("{}", File::H));
    }
}
