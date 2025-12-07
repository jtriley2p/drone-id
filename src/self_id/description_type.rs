use crate::error::Error;

/// Type of Description
///
/// Reserved values are `3` to `200`, private use values are `201` to `255`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DescriptionType {
    /// Free-text ASCII.
    Text,
    /// Emergency text.
    Emergency,
    /// Extended status.
    ///
    /// Included in the specification, but no details about it.
    ExtendedStatus,
    /// Reserved.
    Reserved,
    /// Available for private use.
    PrivateUse,
}

impl DescriptionType {
    /// Special value representing the minimum value for private use.
    pub const PRIVATE_USE_THRESHOLD: u8 = 201;
    /// Special value representing the minimum reserved value.
    ///
    /// > NOTICE: Range for reserved values is only up to 200, above this is for private use.
    pub const RESERVED_THRESHOLD: u8 = 3;
}

impl From<u8> for DescriptionType {
    fn from(value: u8) -> Self {
        if value >= Self::PRIVATE_USE_THRESHOLD {
            return Self::PrivateUse;
        }

        if value >= Self::RESERVED_THRESHOLD {
            return Self::Reserved;
        }

        match value {
            0 => Self::Text,
            1 => Self::Emergency,
            2 => Self::ExtendedStatus,
            _ => Err(Error::Unreachable).unwrap(),
        }
    }
}

impl From<DescriptionType> for u8 {
    fn from(value: DescriptionType) -> Self {
        match value {
            DescriptionType::Text => 0,
            DescriptionType::Emergency => 1,
            DescriptionType::ExtendedStatus => 2,
            DescriptionType::Reserved => DescriptionType::RESERVED_THRESHOLD,
            DescriptionType::PrivateUse => DescriptionType::PRIVATE_USE_THRESHOLD,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::self_id::DescriptionType;

    #[test]
    fn test_encode() {
        let description_type = DescriptionType::Text;

        assert_eq!(u8::from(description_type), 0);
    }

    #[test]
    fn test_encode_reserved() {
        let reserved = DescriptionType::Reserved;

        assert_eq!(u8::from(reserved), DescriptionType::RESERVED_THRESHOLD);
    }

    #[test]
    fn test_encode_private_use() {
        let private_use = DescriptionType::PrivateUse;

        assert_eq!(u8::from(private_use), DescriptionType::PRIVATE_USE_THRESHOLD);
    }

    #[test]
    fn test_decode() {
        let decoded = DescriptionType::from(0);

        assert_eq!(decoded, DescriptionType::Text);
    }

    #[test]
    fn test_decode_reserved() {
        let decoded = DescriptionType::from(DescriptionType::RESERVED_THRESHOLD);

        assert_eq!(decoded, DescriptionType::Reserved);
    }

    #[test]
    fn test_decode_private_use() {
        let decoded = DescriptionType::from(DescriptionType::PRIVATE_USE_THRESHOLD);

        assert_eq!(decoded, DescriptionType::PrivateUse);
    }


}
