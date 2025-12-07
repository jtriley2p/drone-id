use crate::error::Error;

/// Height Type
///
/// Enumerates relative height based on takeoff height versus height above ground level (AGL).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HeightType {
    /// Height relative to take-off altitude.
    TakeOff,
    /// Above ground level.
    AGL,
}

impl TryFrom<u8> for HeightType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::TakeOff),
            1 => Ok(Self::AGL),
            _ => Err(Error::InvalidInteger),
        }
    }
}

impl From<HeightType> for u8 {
    fn from(value: HeightType) -> Self {
        value as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::location::HeightType;

    #[test]
    fn test_encode() {
        let agl = HeightType::AGL;

        assert_eq!(u8::from(agl), 1);
    }

    #[test]
    fn test_decode() {
        let bit = 1;

        let agl = HeightType::try_from(bit).unwrap();

        assert_eq!(agl, HeightType::AGL);
    }

    #[test]
    fn test_decode_fails_invalid_value() {
        let invalid = 2;

        assert!(HeightType::try_from(invalid).is_err());
    }
}
