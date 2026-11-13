//! TACTIL core: six-dot braille cells and the BRF (North American braille ASCII) mapping.
//!
//! Basis: Unicode block U+2800 (dot n = bit n-1) and the public braille-ASCII table.

/// BRF character for each of the 64 six-dot cells; the index is the dot bit-set (dot 1 = 1 … dot 6 = 32).
const BRF: &str = " A1B'K2L@CIF/MSP\"E3H9O6R^DJG>NTQ,*5<-U8V.%[$+X!&;:4\\0Z7(_?W]#Y)=";

/// A six-dot braille cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell(u8);

impl Cell {
    /// From a bit-set (dot 1 = bit 0 … dot 6 = bit 5); `None` above 63.
    pub fn from_bits(bits: u8) -> Option<Cell> {
        (bits < 64).then_some(Cell(bits))
    }

    /// From dot numbers 1..=6; `None` if any dot is outside that range.
    pub fn from_dots(dots: &[u8]) -> Option<Cell> {
        let mut bits = 0u8;
        for &d in dots {
            if !(1..=6).contains(&d) {
                return None;
            }
            bits |= 1 << (d - 1);
        }
        Some(Cell(bits))
    }

    pub fn bits(self) -> u8 {
        self.0
    }

    /// The Unicode braille pattern character.
    pub fn to_unicode(self) -> char {
        char::from_u32(0x2800 + u32::from(self.0)).unwrap_or(' ')
    }

    /// The BRF (braille ASCII) character.
    pub fn to_brf(self) -> char {
        BRF.as_bytes()[usize::from(self.0)] as char
    }

    /// From a BRF character (letters are case-insensitive).
    pub fn from_brf(c: char) -> Option<Cell> {
        BRF.find(c.to_ascii_uppercase()).map(|i| Cell(i as u8))
    }

    /// From a Unicode braille pattern in U+2800..=U+283F.
    pub fn from_unicode(c: char) -> Option<Cell> {
        let v = u32::from(c);
        (0x2800..0x2840)
            .contains(&v)
            .then(|| Cell((v - 0x2800) as u8))
    }
}

/// Convert BRF text to Unicode braille; characters outside the table (e.g. newlines) pass through.
pub fn brf_to_unicode(s: &str) -> String {
    s.chars()
        .map(|c| Cell::from_brf(c).map_or(c, Cell::to_unicode))
        .collect()
}

/// Convert Unicode braille to BRF text; other characters pass through.
pub fn unicode_to_brf(s: &str) -> String {
    s.chars()
        .map(|c| Cell::from_unicode(c).map_or(c, Cell::to_brf))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_complete() {
        assert_eq!(BRF.chars().count(), 64);
        let mut seen: Vec<char> = BRF.chars().collect();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), 64);
    }

    #[test]
    fn round_trips() {
        for bits in 0..64u8 {
            let c = Cell::from_bits(bits).unwrap();
            assert_eq!(Cell::from_brf(c.to_brf()), Some(c));
            assert_eq!(Cell::from_unicode(c.to_unicode()), Some(c));
        }
    }

    #[test]
    fn known_cells() {
        assert_eq!(brf_to_unicode("ABC"), "⠁⠃⠉");
        assert_eq!(unicode_to_brf("⠁⠃⠉"), "ABC");
        let k = Cell::from_dots(&[1, 3]).unwrap();
        assert_eq!((k.to_unicode(), k.to_brf()), ('\u{2805}', 'K'));
        assert_eq!(Cell::from_dots(&[7]), None);
        assert_eq!(Cell::from_bits(64), None);
        assert_eq!(brf_to_unicode("a\nb"), "⠁\n⠃");
    }
}
