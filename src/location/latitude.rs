/// Latitude
///
/// Specification calls for the decoded value to be a 64-bit floating point number and the encoded
/// value to be a 32-bit signed integer. Encoding entails multiplying by 10,000,000 and decoding
/// entails dividing by 10,000,000.
///
/// It's worth noting that [`Latitude::Unknown`] technically should only be set if both latitude
/// and longitude are set to zero. However, fully checking this in the type system would make these
/// values dependent on one another and we have to draw the line somewhere and that line is right
/// here.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Latitude {
    /// Invalid value.
    Invalid,
    /// Unknown value (indicated by 0).
    Unknown,
    /// Known, valid value.
    Known(f64),
}

impl Latitude {
    /// Special value if the actual value is unknown.
    pub const UNKNOWN_CODE: f64 = 0.0;

    /// Minimum latitude value is -90 degrees (South Pole).
    pub const MIN: f64 = -90.0;

    /// Maximum latitude value is 90 degrees (North Pole).
    pub const MAX: f64 = 90.0;

    /// Multiplier allows for 7 digits of resolution when encoding/decoding as required by the
    /// specification.
    pub const MULTIPLIER: f64 = 10_000_000.0;

    /// Returns the inner latitude value as a 64-bit float.
    ///
    /// In the event a library consumer manually constructs an invalid value, ie
    /// `Latitude::Known(91.0)`, the [`Latitude::latitude`] function clamps the value down to the
    /// valid range.
    pub fn latitude(&self) -> f64 {
        match self {
            Self::Invalid | Self::Unknown => Self::UNKNOWN_CODE,
            Self::Known(n) => n.clamp(Self::MIN, Self::MAX),
        }
    }
}

impl From<i32> for Latitude {
    fn from(value: i32) -> Self {
        let value = value as f64 / Self::MULTIPLIER;

        if value == Self::UNKNOWN_CODE {
            return Self::Unknown;
        }

        if value < Self::MIN || value > Self::MAX {
            return Self::Invalid;
        }

        Self::Known(value)
    }
}

impl From<Latitude> for i32 {
    fn from(value: Latitude) -> Self {
        (value.latitude() * Latitude::MULTIPLIER) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::location::Latitude;

    #[test]
    fn test_inner_latitude() {
        assert_eq!(Latitude::Invalid.latitude(), Latitude::UNKNOWN_CODE);
        assert_eq!(Latitude::Unknown.latitude(), Latitude::UNKNOWN_CODE);
        assert_eq!(Latitude::Known(1f64).latitude(), 1f64);
    }

    #[test]
    fn test_encode() {
        let value = 5f64;

        let latitude = Latitude::Known(value);

        let encoded = i32::from(latitude);

        assert_eq!(encoded, (value * Latitude::MULTIPLIER) as i32);
    }

    #[test]
    fn test_encode_unknown() {
        let unknown_value = 0i32;

        let latitude = Latitude::Known(unknown_value as f64 / Latitude::MULTIPLIER);

        let encoded = i32::from(latitude);

        assert_eq!(encoded, unknown_value);
    }

    #[test]
    fn test_encode_invalid() {
        let latitude = Latitude::Invalid;

        let encoded = i32::from(latitude);

        assert_eq!(encoded, 0i32);
    }

    #[test]
    fn test_encode_invalid_state() {
        let invalid_state = Latitude::Known(91f64 * Latitude::MULTIPLIER);

        assert_eq!(
            i32::from(invalid_state),
            (90f64 * Latitude::MULTIPLIER) as i32
        );
    }

    #[test]
    fn test_decode() {
        let value = 5i32;
        let latitude = Latitude::from(value);

        assert_eq!(
            latitude,
            Latitude::Known(value as f64 / Latitude::MULTIPLIER)
        );
    }

    #[test]
    fn test_decode_invalid() {
        let too_small = -91.0 * Latitude::MULTIPLIER;
        let too_large = 91.0 * Latitude::MULTIPLIER;

        assert_eq!(Latitude::from(too_small as i32), Latitude::Invalid);
        assert_eq!(Latitude::from(too_large as i32), Latitude::Invalid);
    }

    #[test]
    fn test_decode_unknown() {
        let unknown = 0i32;
        let latitude = Latitude::from(unknown);

        assert_eq!(latitude, Latitude::Unknown);
    }
}
