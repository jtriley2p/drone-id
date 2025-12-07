/// Timestamp for Location Messages
///
/// Differs from [`crate::system::Timestamp`], as this encapsulates a 16-bit unsigned
/// integer representing the number of tenths of a second since the start of the current hour.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Timestamp {
    /// Invalid value (greater than 36,000).
    Invalid,
    /// No value.
    NoValue,
    /// Unknown value (indicated by 0xFFFF).
    Unknown,
    /// Known, valid value.
    Known(u16),
}

impl Timestamp {
    /// Special value if the actual value is unknown.
    pub const UNKNOWN_CODE: u16 = 0xffff;

    /// Maximum number of seconds since the last hour.
    pub const MAX: u16 = 36_000;

    /// Returns the inner timestamp value.
    pub fn timestamp(&self) -> u16 {
        let value = match self {
            Timestamp::NoValue => 0,
            Timestamp::Unknown => Timestamp::UNKNOWN_CODE,
            Timestamp::Invalid => Timestamp::MAX + 1,
            Timestamp::Known(n) => *n,
        };

        // in the event a library consumer constructs an invalid state manually, we clamp the value
        // down to the invalid value.
        if value > Timestamp::MAX && value < Timestamp::UNKNOWN_CODE {
            return Timestamp::MAX + 1;
        }

        value
    }
}

impl From<u16> for Timestamp {
    fn from(value: u16) -> Self {
        if value == Self::UNKNOWN_CODE {
            return Self::Unknown;
        }

        if value > Self::MAX {
            return Self::Invalid;
        }

        Self::Known(value)
    }
}

impl From<Timestamp> for u16 {
    fn from(value: Timestamp) -> Self {
        value.timestamp()
    }
}

#[cfg(test)]
mod tests {
    use crate::location::Timestamp;

    #[test]
    fn test_timestamp() {
        assert_eq!(Timestamp::Invalid.timestamp(), Timestamp::MAX + 1);
        assert_eq!(Timestamp::NoValue.timestamp(), 0);
        assert_eq!(Timestamp::Unknown.timestamp(), Timestamp::UNKNOWN_CODE);
        assert_eq!(Timestamp::Known(1).timestamp(), 1);

    }

    #[test]
    fn test_encode() {
        let timestamp = Timestamp::Known(1);

        assert_eq!(u16::from(timestamp), 1);
    }

    #[test]
    fn test_encode_unknown() {
        let unknown = Timestamp::Unknown;

        assert_eq!(u16::from(unknown), Timestamp::UNKNOWN_CODE);
    }

    #[test]
    fn test_encode_invalid() {
        let invalid = Timestamp::Invalid;

        assert_eq!(u16::from(invalid), Timestamp::MAX + 1);
    }

    #[test]
    fn test_encode_invalid_state() {
        let invalid_state = Timestamp::Known(Timestamp::MAX + 2);

        assert_eq!(u16::from(invalid_state), Timestamp::MAX + 1);
    }

    #[test]
    fn test_decode() {
        let decoded = Timestamp::from(1);

        assert_eq!(decoded, Timestamp::Known(1));
    }

    #[test]
    fn test_decode_unknown() {
        let decoded = Timestamp::from(Timestamp::UNKNOWN_CODE);

        assert_eq!(decoded, Timestamp::Unknown);
    }

    #[test]
    fn test_decode_invalid() {
        let decoded = Timestamp::from(Timestamp::MAX + 1);

        assert_eq!(decoded, Timestamp::Invalid);
    }
}
