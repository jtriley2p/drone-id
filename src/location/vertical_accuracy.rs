/// Vertical Accuracy
///
/// Accuracy on the vertical axis. This is based on the Geometric Vertical Accuracy (GVA)
/// enumeration from the Automatic Dependent Surveillance-Broadcast (ADS-B) specification.
///
/// An enumerated value of zero implies either the accuracy is greater than or equal to 150m or that
/// the accuracy is unknown.
///
/// Ideally, this would be fully enumerated, but since the values to enumerate are also numeric,
/// writing out "OneHundredFiftyM" etc would be obnoxious.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum VerticalAccuracy {
    /// Reserved.
    Reserved,
    /// Unknown value (indicated by 0).
    Unknown,
    /// Known, valid value.
    Known(u8),
}

impl VerticalAccuracy {
    /// Special value if the actual value is unknown.
    pub const UNKNOWN_CODE: u8 = 0;

    /// Special value representing the minimum "reserved" value.
    pub const RESERVED_THRESHOLD: u8 = 7;

    /// Special value representing the maximum accuracy.
    pub const MAX: f32 = 151.0;

    /// Returns the raw enumerated code.
    pub fn code(&self) -> u8 {
        match self {
            Self::Known(n) => *n,
            Self::Unknown | Self::Reserved => 0,
        }
    }

    /// Returns the accuracy, in meters.
    ///
    /// A value of `151` implies the accuracy is either greater than or equal to 150m or the value
    /// is unknown. All other values returned implies the accuracy is less than the returned value.
    ///
    /// In the event an invalid state is constructed manually ie `VerticalAccuracy::Known(7)`, we
    /// treat the value the same as if it were [`VerticalAccuracy::Reserved`].
    pub fn accuracy(&self) -> f32 {
        match self {
            Self::Unknown | Self::Reserved => Self::MAX,
            Self::Known(n) => match n {
                1 => 150.0,
                2 => 45.0,
                3 => 25.0,
                4 => 10.0,
                5 => 3.0,
                6 => 1.0,
                _ => Self::MAX,
            },
        }
    }
}

impl From<u8> for VerticalAccuracy {
    fn from(value: u8) -> Self {
        if value >= Self::RESERVED_THRESHOLD {
            return Self::Reserved;
        }

        match value {
            Self::UNKNOWN_CODE => Self::Unknown,
            n => Self::Known(n),
        }
    }
}

impl From<VerticalAccuracy> for u8 {
    fn from(value: VerticalAccuracy) -> Self {
        match value {
            VerticalAccuracy::Reserved => VerticalAccuracy::RESERVED_THRESHOLD,
            VerticalAccuracy::Unknown => VerticalAccuracy::UNKNOWN_CODE,
            VerticalAccuracy::Known(n) => n.clamp(0, VerticalAccuracy::RESERVED_THRESHOLD),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::location::VerticalAccuracy;

    #[test]
    fn test_accuracy() {
        assert_eq!(VerticalAccuracy::Reserved.accuracy(), VerticalAccuracy::MAX);
        assert_eq!(VerticalAccuracy::Unknown.accuracy(), VerticalAccuracy::MAX);
        assert_eq!(VerticalAccuracy::Known(1).accuracy(), 150.0);
    }

    #[test]
    fn test_encode() {
        let accuracy = VerticalAccuracy::Known(1);

        assert_eq!(u8::from(accuracy), 1);
    }

    #[test]
    fn test_encode_unknown() {
        let unknown = VerticalAccuracy::Unknown;

        assert_eq!(u8::from(unknown), VerticalAccuracy::UNKNOWN_CODE);
    }

    #[test]
    fn test_encode_reserved() {
        let reserved = VerticalAccuracy::Reserved;

        assert_eq!(u8::from(reserved), VerticalAccuracy::RESERVED_THRESHOLD);
    }

    #[test]
    fn test_encode_invalid_state() {
        let invalid_state = VerticalAccuracy::Known(7);

        assert_eq!(
            u8::from(invalid_state),
            VerticalAccuracy::RESERVED_THRESHOLD
        );
    }

    #[test]
    fn test_decode() {
        let decoded = VerticalAccuracy::from(1);

        assert_eq!(decoded, VerticalAccuracy::Known(1));
    }

    #[test]
    fn test_decode_unknown() {
        let decoded = VerticalAccuracy::from(VerticalAccuracy::UNKNOWN_CODE);

        assert_eq!(decoded, VerticalAccuracy::Unknown);
    }

    #[test]
    fn test_decode_reserved() {
        let decoded = VerticalAccuracy::from(VerticalAccuracy::RESERVED_THRESHOLD);

        assert_eq!(decoded, VerticalAccuracy::Reserved);
    }
}
