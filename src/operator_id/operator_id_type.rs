/// Type of Operator ID
///
/// Generally set to [`OperatorIDType::OperatorID`] (0).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OperatorIDType {
    /// Operator ID
    ///
    /// Most common option.
    OperatorID,
    /// Reserved.
    Reserved,
    /// Available for private use.
    PrivateUse,
}

impl OperatorIDType {
    /// Special value representing the minimum "reserved" value.
    ///
    /// > NOTICE: Maximum reserved value is [`OperatorIDType::PRIVATE_USE_THRESHOLD`] minus one.
    pub const RESERVED_THRESHOLD: u8 = 1;

    /// Special value representing the minimum "private use" value.
    pub const PRIVATE_USE_THRESHOLD: u8 = 201;
}

impl From<u8> for OperatorIDType {
    fn from(value: u8) -> Self {
        if value == 0 {
            return Self::OperatorID;
        }

        if value < Self::PRIVATE_USE_THRESHOLD {
            return Self::Reserved;
        }

        Self::PrivateUse
    }
}

impl From<OperatorIDType> for u8 {
    fn from(value: OperatorIDType) -> Self {
        match value {
            OperatorIDType::OperatorID => 0,
            OperatorIDType::Reserved => OperatorIDType::RESERVED_THRESHOLD,
            OperatorIDType::PrivateUse => OperatorIDType::PRIVATE_USE_THRESHOLD,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::operator_id::OperatorIDType;

    #[test]
    fn test_encode() {
        let operator_id_type = OperatorIDType::OperatorID;

        assert_eq!(u8::from(operator_id_type), 0);
    }

    #[test]
    fn test_encode_reserved() {
        let reserved = OperatorIDType::Reserved;

        assert_eq!(u8::from(reserved), OperatorIDType::RESERVED_THRESHOLD);
    }

    #[test]
    fn test_encode_private_use() {
        let private_use = OperatorIDType::PrivateUse;

        assert_eq!(u8::from(private_use), OperatorIDType::PRIVATE_USE_THRESHOLD);
    }

    #[test]
    fn test_decode() {
        let decoded = OperatorIDType::from(0);

        assert_eq!(decoded, OperatorIDType::OperatorID);
    }

    #[test]
    fn test_decode_reserved() {
        let decoded = OperatorIDType::from(OperatorIDType::RESERVED_THRESHOLD);

        assert_eq!(decoded, OperatorIDType::Reserved);
    }

    #[test]
    fn test_decode_private_use() {
        let decoded = OperatorIDType::from(OperatorIDType::PRIVATE_USE_THRESHOLD);

        assert_eq!(decoded, OperatorIDType::PrivateUse);
    }
}
