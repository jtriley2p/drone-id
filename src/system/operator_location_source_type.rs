use crate::error::Error;

/// Operator Location Source Type
///
/// Represents the type of operator location that is transmitted; it may represent the take-off
/// location of the aircraft, as well as a different location which can be fixed or dynamic.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OperatorLocationSourceType {
    /// Take-off location.
    TakeOff,
    /// Dynamic location.
    Dynamic,
    /// Fixed location.
    Fixed,
}

impl TryFrom<u8> for OperatorLocationSourceType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::TakeOff),
            1 => Ok(Self::Dynamic),
            2 => Ok(Self::Fixed),
            _ => Err(Error::InvalidInteger),
        }
    }
}

impl From<OperatorLocationSourceType> for u8 {
    fn from(value: OperatorLocationSourceType) -> Self {
        value as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::system::OperatorLocationSourceType;

    #[test]
    fn test_encode() {
        let src_type = OperatorLocationSourceType::TakeOff;

        assert_eq!(u8::from(src_type), 0);
    }

    #[test]
    fn test_decode() {
        let decoded = OperatorLocationSourceType::try_from(1).unwrap();

        assert_eq!(decoded, OperatorLocationSourceType::Dynamic);
    }

    #[test]
    fn test_decode_fails_invalid_integer() {
        assert!(OperatorLocationSourceType::try_from(3).is_err());
    }
}
