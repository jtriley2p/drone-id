/// Speed Accuracy
///
/// Accuracy of the reported speed. This is based on the Navigation Accuracy Category for Velocity
/// (NACV) enumeration from the Automatic Dependent Surveillance-Broadcast (ADS-B) specification.
///
/// Ideally, this would be fully enumerated, but since the values to enumerate are also numeric,
/// writing out "ZeroPointThree" etc would be obnoxious.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SpeedAccuracy {
    /// Reserved.
    Reserved,
    /// Unknown value (indicated by 0).
    Unknown,
    /// Known, valid value.
    Known(u8),
}

impl SpeedAccuracy {
    /// Special value if the actual value is unknown.
    pub const UNKNOWN_CODE: u8 = 0;

    /// Special value representing the minimum "reserved" value.
    pub const RESERVED_THRESHOLD: u8 = 5;

    /// Maximum speed accuracy.
    pub const MAX: f32 = 11.0;

    /// Returns the raw enumerated code.
    pub fn code(&self) -> u8 {
        match self {
            Self::Known(n) => *n,
            Self::Unknown => Self::UNKNOWN_CODE,
            Self::Reserved => Self::RESERVED_THRESHOLD,
        }
    }

    /// Returns the speed accuracy, in meters per second.
    ///
    /// A value of `11` implies the accuracy is either greater than or equal to 10m/s or the value
    /// is unknown. All other values returned implies the accuracy is less than the returned value.
    pub fn meters_per_second(&self) -> f32 {
        match self {
            Self::Unknown | Self::Reserved => Self::MAX,
            Self::Known(n) => match n {
                1 => 10.0,
                2 => 3.0,
                3 => 1.0,
                4 => 0.3,
                _ => Self::MAX,
            },
        }
    }
}

impl From<u8> for SpeedAccuracy {
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

impl From<SpeedAccuracy> for u8 {
    fn from(value: SpeedAccuracy) -> Self {
        // clamp the value down to max valid value in case a library consumer constructs an invalid
        // state manually.
        match value {
            SpeedAccuracy::Reserved => SpeedAccuracy::RESERVED_THRESHOLD,
            SpeedAccuracy::Unknown => SpeedAccuracy::UNKNOWN_CODE,
            SpeedAccuracy::Known(n) => n.clamp(0, SpeedAccuracy::RESERVED_THRESHOLD),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::location::SpeedAccuracy;

    #[test]
    fn test_meters_per_second() {
        assert_eq!(SpeedAccuracy::Known(1).meters_per_second(), 10.0);
        assert_eq!(
            SpeedAccuracy::Unknown.meters_per_second(),
            SpeedAccuracy::MAX
        );
        assert_eq!(
            SpeedAccuracy::Reserved.meters_per_second(),
            SpeedAccuracy::MAX
        );
    }

    #[test]
    fn test_encode() {
        let accuracy = SpeedAccuracy::Known(1);

        assert_eq!(u8::from(accuracy), 1);
    }

    #[test]
    fn test_enocde_reserved() {
        let reserved = SpeedAccuracy::Reserved;

        assert_eq!(u8::from(reserved), SpeedAccuracy::RESERVED_THRESHOLD);
    }

    #[test]
    fn test_encode_unknown() {
        let unknown = SpeedAccuracy::Unknown;

        assert_eq!(u8::from(unknown), SpeedAccuracy::UNKNOWN_CODE);
    }

    #[test]
    fn test_encode_invalid_state() {
        let invalid_state = SpeedAccuracy::Known(5);

        assert_eq!(u8::from(invalid_state), SpeedAccuracy::RESERVED_THRESHOLD);
    }

    #[test]
    fn test_decode() {
        let decoded = SpeedAccuracy::from(1);

        assert_eq!(decoded, SpeedAccuracy::Known(1));
    }

    #[test]
    fn test_deocde_reserved() {
        let reserved = SpeedAccuracy::from(SpeedAccuracy::RESERVED_THRESHOLD);

        assert_eq!(reserved, SpeedAccuracy::Reserved);
    }

    #[test]
    fn test_decode_unknown() {
        let unknown = SpeedAccuracy::from(SpeedAccuracy::UNKNOWN_CODE);

        assert_eq!(unknown, SpeedAccuracy::Unknown);
    }
}
