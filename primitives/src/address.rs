
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Address {
    pub bytes: [u8; 32],
}

impl Address {
    pub fn new(bytes: [u8; 32]) -> Address {
        Address { bytes }
    }

    pub fn from(s: &str) -> Address {
        let mut bytes = [0; 32];
        for (i, c) in s.chars().enumerate() {
            if i >= 32 {
                break;
            }
            bytes[i] = c as u8;
        }
        Address { bytes }
    }

}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x")?;
        for byte in self.bytes.iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}