/// Horizontal Accuracy
///
/// Accuracy on the horizontal axis. This is based on the Navigation Accuracy Category for Position
/// (NACP) enumeration from the Automatic Dependent Surveillance-Broadcast (ADS-B) specification.
///
/// An enumerated value of zero implies either the accuracy is greater than or equal to 18.52km or
/// that the accuracy is unknown.
///
/// Ideally, this would be fully enumerated, but since the values to enumerate are also numeric,
/// writing out "EightteenPointFiveTwoKm" etc would be obnoxious.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HorizontalAccuracy {
    /// Reserved.
    Reserved,
    /// Unknown value (indicated by 0).
    Unknown,
    /// Known, valid value.
    Known(u8),
}

impl HorizontalAccuracy {
    /// Special value if the actual value is unknown.
    pub const UNKNOWN_CODE: u8 = 0;

    /// Special value representing the minimum "reserved" value.
    pub const RESERVED_THRESHOLD: u8 = 13;

    /// Maximum accuracy, implying the value is reserved or unknown
    pub const MAX: f32 = 18_521.0;

    /// Returns the accuracy, in meters.
    ///
    /// A value of `18_521` implies the accuracy is either greater than or equal to 18.52km or the
    /// value is unknown. All other values returned implies the accuracy is less than the returned
    /// value.
    ///
    /// While constructing [`HorizontalAccuracy`] through the deseralizer should never allow a known
    /// value greater than 13, a library consumer may construct this manually, ie
    /// `HorizontalAccuracy::Known(13)`, which is technically invalid, but to avoid a panic, we
    /// return `accuracy`
    pub fn accuracy_meters(&self) -> f32 {
        match self {
            Self::Unknown | Self::Reserved => Self::MAX,
            Self::Known(n) => match n {
                1 => Self::MAX,
                2 => 7_408.0,
                3 => 3_704.0,
                4 => 1_852.0,
                5 => 926.0,
                6 => 555.6,
                7 => 185.2,
                8 => 92.6,
                9 => 30.0,
                10 => 10.0,
                11 => 3.0,
                12 => 1.0,
                _ => Self::MAX,
            },
        }
    }
}

impl From<u8> for HorizontalAccuracy {
    fn from(value: u8) -> Self {
        if value >= Self::RESERVED_THRESHOLD {
            return Self::Reserved;
        }

        match value {
            0 => Self::Unknown,
            n => Self::Known(n),
        }
    }
}

impl From<HorizontalAccuracy> for u8 {
    fn from(value: HorizontalAccuracy) -> Self {
        let value = match value {
            HorizontalAccuracy::Reserved => HorizontalAccuracy::RESERVED_THRESHOLD,
            HorizontalAccuracy::Unknown => HorizontalAccuracy::UNKNOWN_CODE,
            HorizontalAccuracy::Known(n) => n,
        };

        // we clamp the value in the event that a library consumer constructs an invalid state such
        // as `HorizontalAccuracy::Known(13)`.
        if value > HorizontalAccuracy::RESERVED_THRESHOLD {
            return HorizontalAccuracy::RESERVED_THRESHOLD;
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use crate::location::HorizontalAccuracy;

    #[test]
    fn test_accuracy_meters() {
        let max = HorizontalAccuracy::MAX;

        assert_eq!(HorizontalAccuracy::from(0).accuracy_meters(), max);
        assert_eq!(HorizontalAccuracy::from(1).accuracy_meters(), max);
        assert_eq!(HorizontalAccuracy::from(2).accuracy_meters(), 7_408.0);
        assert_eq!(HorizontalAccuracy::from(3).accuracy_meters(), 3_704.0);
        assert_eq!(HorizontalAccuracy::from(4).accuracy_meters(), 1_852.0);
        assert_eq!(HorizontalAccuracy::from(5).accuracy_meters(), 926.0);
        assert_eq!(HorizontalAccuracy::from(6).accuracy_meters(), 555.6);
        assert_eq!(HorizontalAccuracy::from(7).accuracy_meters(), 185.2);
        assert_eq!(HorizontalAccuracy::from(8).accuracy_meters(), 92.6);
        assert_eq!(HorizontalAccuracy::from(9).accuracy_meters(), 30.0);
        assert_eq!(HorizontalAccuracy::from(10).accuracy_meters(), 10.0);
        assert_eq!(HorizontalAccuracy::from(11).accuracy_meters(), 3.0);
        assert_eq!(HorizontalAccuracy::from(12).accuracy_meters(), 1.0);
        assert_eq!(HorizontalAccuracy::from(13).accuracy_meters(), max);
    }

    #[test]
    fn test_encode() {
        let accuracy_code = 1;

        let horizontal_accuracy = HorizontalAccuracy::Known(accuracy_code);

        let encoded = u8::from(horizontal_accuracy);

        assert_eq!(encoded, accuracy_code);
    }

    #[test]
    fn test_encode_reserved() {
        let reserved_code = HorizontalAccuracy::RESERVED_THRESHOLD;

        let horizontal_accuracy = HorizontalAccuracy::Known(reserved_code);

        let encoded = u8::from(horizontal_accuracy);

        assert_eq!(encoded, reserved_code);
    }

    #[test]
    fn test_encode_unknown() {
        let unknown_code = HorizontalAccuracy::UNKNOWN_CODE;

        let horizontal_accuracy = HorizontalAccuracy::Known(unknown_code);

        let encoded = u8::from(horizontal_accuracy);

        assert_eq!(encoded, unknown_code);
        
    }

    #[test]
    fn test_encode_from_invalid_state() {
        let invalid_state = HorizontalAccuracy::Known(13);

        assert_eq!(invalid_state.accuracy_meters(), HorizontalAccuracy::MAX);
        assert_eq!(u8::from(invalid_state), 13);
    }

    #[test]
    fn test_decode() {
        let encoded = 1;

        let decoded = HorizontalAccuracy::from(encoded);

        assert_eq!(decoded, HorizontalAccuracy::Known(encoded));
    }

    #[test]
    fn test_decode_reserved() {
        let reserved_encoded = HorizontalAccuracy::RESERVED_THRESHOLD;

        let decoded = HorizontalAccuracy::from(reserved_encoded);

        assert_eq!(decoded, HorizontalAccuracy::Reserved);
    }

    #[test]
    fn test_decode_unknown() {
        let unknown_encoded = HorizontalAccuracy::UNKNOWN_CODE;

        let decoded = HorizontalAccuracy::from(unknown_encoded);

        assert_eq!(decoded, HorizontalAccuracy::Unknown);
    }
}
