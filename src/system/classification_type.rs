use crate::error::Error;

/// Classification Type
///
/// Determines the classification type for a given region.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ClassificationType {
    /// Undeclared.
    Undeclared,
    /// European Union-specific classification.
    EuropeanUnion,
    /// Reserved.
    Reserved,
}

impl ClassificationType {
    /// Special value representing the minimum "reserved" value.
    pub const RESERVED_THRESHOLD: u8 = 2;

    /// Special value representing the maxmimum valid value.
    pub const MAX: u8 = 7;
}

impl TryFrom<u8> for ClassificationType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > Self::MAX {
            return Err(Error::InvalidInteger);
        }

        match value {
            0 => Ok(Self::Undeclared),
            1 => Ok(Self::EuropeanUnion),
            _ => Ok(Self::Reserved),
        }
    }
}

impl From<ClassificationType> for u8 {
    fn from(value: ClassificationType) -> Self {
        value as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::system::ClassificationType;

    #[test]
    fn test_encode() {
        let classification_type = ClassificationType::EuropeanUnion;

        assert_eq!(u8::from(classification_type), 1);
    }

    #[test]
    fn test_encode_reserved() {
        let reserved = ClassificationType::Reserved;

        assert_eq!(u8::from(reserved), 2);
    }

    #[test]
    fn test_decode() {
        let decoded = ClassificationType::try_from(1).unwrap();

        assert_eq!(decoded, ClassificationType::EuropeanUnion);
    }

    #[test]
    fn test_decode_fails_invalid_integer() {
        assert!(ClassificationType::try_from(ClassificationType::MAX + 1).is_err());
    }
}
