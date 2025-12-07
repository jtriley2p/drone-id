/// Longitude
///
/// Specification calls for the decoded value to be a 64-bit floating point number and the encoded
/// value to be a 32-bit signed integer. Encoding entails multiplying by 10,000,000 and decoding
/// entails dividing by 10,000,000.
///
/// It's worth noting that [`Longitude::Unknown`] technically should only be set if both latitude
/// and longitude are set to zero. However, fully checking this in the type system would make these
/// values dependent on one another and we have to draw the line somewhere and that line is right
/// here.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Longitude {
    /// Invalid value (beyond the -180 to 180 degree bound).
    Invalid,
    /// Unknown value (indicated by 0.0)
    Unknown,
    /// Known, valid value.
    Known(f64),
}

impl Longitude {
    /// Special value if the actual value is unknown.
    pub const UNKNOWN_CODE: f64 = 0.0;

    /// Minimum Longitude value is -180 degrees (Prime Meridian).
    pub const MIN: f64 = -180.0;

    /// Maximum Longitude value is 180 degrees (Also the Prime Meridian).
    pub const MAX: f64 = 180.0;

    /// Multiplier allows for 7 digits of resolution when encoding/decoding as required by the
    /// specification.
    pub const MULTIPLIER: f64 = 10_000_000.0;

    /// Returns the inner value as a 64-bit float.
    ///
    /// In the event a library consumer manually constructs an invalid value, ie
    /// `Longitude::Known(91.0)`, the [`Longitude::longitude`] function clamps the value down to the
    /// valid range.
    pub fn longitude(&self) -> f64 {
        match self {
            Self::Invalid | Self::Unknown => 0.0,
            Self::Known(n) => n.clamp(Self::MIN, Self::MAX),
        }
    }
}

impl From<i32> for Longitude {
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

impl From<Longitude> for i32 {
    fn from(value: Longitude) -> Self {
        (value.longitude() * Longitude::MULTIPLIER) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::location::Longitude;

    #[test]
    fn test_inner_longitude() {
        assert_eq!(Longitude::Invalid.longitude(), Longitude::UNKNOWN_CODE);
        assert_eq!(Longitude::Unknown.longitude(), Longitude::UNKNOWN_CODE);
        assert_eq!(Longitude::Known(1f64).longitude(), 1f64);
    }

    #[test]
    fn test_encode() {
        let value = 5f64;

        let longitude = Longitude::Known(value);

        let encoded = i32::from(longitude);

        assert_eq!(encoded, (value * Longitude::MULTIPLIER) as i32);
    }

    #[test]
    fn test_encode_unknown() {
        let unknown_value = 0i32;

        let longitude = Longitude::Known(unknown_value as f64 / Longitude::MULTIPLIER);

        let encoded = i32::from(longitude);

        assert_eq!(encoded, unknown_value);
    }

    #[test]
    fn test_encode_invalid() {
        let longitude = Longitude::Invalid;

        let encoded = i32::from(longitude);

        assert_eq!(encoded, 0i32);
    }

    #[test]
    fn test_encode_invalid_state() {
        let invalid_state = Longitude::Known(181f64 * Longitude::MULTIPLIER);

        assert_eq!(
            i32::from(invalid_state),
            (180f64 * Longitude::MULTIPLIER) as i32
        );
    }

    #[test]
    fn test_decode() {
        let value = 5i32;
        let longitude = Longitude::from(value);

        assert_eq!(
            longitude,
            Longitude::Known(value as f64 / Longitude::MULTIPLIER)
        );
    }

    #[test]
    fn test_decode_invalid() {
        let too_small = -181.0 * Longitude::MULTIPLIER;
        let too_large = 181.0 * Longitude::MULTIPLIER;

        assert_eq!(Longitude::from(too_small as i32), Longitude::Invalid);
        assert_eq!(Longitude::from(too_large as i32), Longitude::Invalid);
    }

    #[test]
    fn test_decode_unknown() {
        let unknown = 0i32;
        let longitude = Longitude::from(unknown);

        assert_eq!(longitude, Longitude::Unknown);
    }
}
