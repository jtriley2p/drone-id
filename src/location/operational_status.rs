use crate::error::Error;

/// Operational Status
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OperationalStatus {
    /// Undeclared status.
    Undeclared,
    /// UAS is on the ground.
    Ground,
    /// UAS is airborne.
    Airborne,
    /// UAS is signalling an emergency.
    Emergency,
    /// UAS Remote ID system is failing.
    RemoteIDSystemFailure,
    /// Reserved.
    Reserved,
}

impl OperationalStatus {
    /// Maximum valid value for the operational status.
    pub const MAX: u8 = 15;

    /// Special value representing the minimum "reserved" value.
    pub const RESERVED_THRESHOLD: u8 = 5;
}

impl From<OperationalStatus> for u8 {
    fn from(value: OperationalStatus) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for OperationalStatus {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > Self::MAX {
            return Err(Error::InvalidInteger);
        }

        match value {
            0 => Ok(Self::Undeclared),
            1 => Ok(Self::Ground),
            2 => Ok(Self::Airborne),
            3 => Ok(Self::Emergency),
            4 => Ok(Self::RemoteIDSystemFailure),
            _ => Ok(Self::Reserved),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::location::OperationalStatus;

    #[test]
    fn test_encode() {
        let status = OperationalStatus::Ground;

        assert_eq!(u8::from(status), 1);
    }

    #[test]
    fn test_decode() {
        let encoded = 1;

        let decoded = OperationalStatus::try_from(encoded).unwrap();

        assert_eq!(decoded, OperationalStatus::Ground);
    }

    #[test]
    fn test_decode_reserved() {
        let reserved = OperationalStatus::RESERVED_THRESHOLD;

        let decoded = OperationalStatus::try_from(reserved).unwrap();

        assert_eq!(decoded, OperationalStatus::Reserved);
    }

    #[test]
    fn test_decode_fails_invalid_value() {
        let invalid = OperationalStatus::MAX + 1;

        assert!(OperationalStatus::try_from(invalid).is_err());
    }
}
