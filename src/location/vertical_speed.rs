/// Vertical Speed
///
/// Represents speed upward relative to the WSG-84 datum measued in meters per second.
///
/// A value of 63.0 indicates the vertical speed is unknown, while any other values beyond the lower
/// boundary of -63.0 and the upper boundary of 63.0 are clamped to their respective boundaries.
///
/// > NOTICE: Specification calls for multiplication by 0.5 to decode and divide by 0.5 to encode.
/// > Though it would be more readable to invert these (`n * 0.5 == n / 2`), we leave it as-is to
/// > more explicitly conform to the specification.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum VerticalSpeed {
    /// Invalid value.
    /// 
    /// Specification calls for this, but there seems to be no way to construct this through
    /// deserialization.
    Invalid,
    /// No value.
    NoValue,
    /// Unknown value (indicated by 63.0).
    Unknown,
    /// Known, valid value.
    Known(f32),
}

impl VerticalSpeed {
    /// Special value indicating the vertical speed is "unknown".
    pub const UNKNOWN_CODE: f32 = 63.0;

    /// Multiplier to encode/decode speed.
    pub const MULTIPLIER: f32 = 0.5;

    /// Returns the inner vertical speed.
    pub fn vertical_speed(&self) -> f32 {
        match self {
            Self::Invalid | Self::NoValue => 0.0,
            Self::Unknown => Self::UNKNOWN_CODE,
            Self::Known(n) => n.clamp(-62.0, 62.0),
        }
    }
}

impl From<u8> for VerticalSpeed {
    fn from(value: u8) -> Self {
        // encoded is a u8; we go u8 -> i8 -> f32 to account for 2's complement
        let value: f32 = value as i8 as f32 * Self::MULTIPLIER;

        match value {
            Self::UNKNOWN_CODE => Self::Unknown,
            n => Self::Known(n.clamp(-62.0, 62.0)),
        }
    }
}

impl From<VerticalSpeed> for u8 {
    fn from(value: VerticalSpeed) -> Self {
        let value = value.vertical_speed() / VerticalSpeed::MULTIPLIER;

        // UNSAFE REASON: There is no safe API for rounding the `VerticalSpeed::Known` branch. This
        // is safe because the value is checked prior to this expression.
        unsafe { f32::to_int_unchecked(value) }
    }
}

#[cfg(test)]
mod tests {
    use crate::location::VerticalSpeed;

    #[test]
    fn test_vertical_speed() {
        assert_eq!(VerticalSpeed::Invalid.vertical_speed(), 0.0);
        assert_eq!(VerticalSpeed::NoValue.vertical_speed(), 0.0);
        assert_eq!(VerticalSpeed::Unknown.vertical_speed(), VerticalSpeed::UNKNOWN_CODE);
        assert_eq!(VerticalSpeed::Known(1.0).vertical_speed(), 1.0);

    }

    #[test]
    fn test_encode() {
        let vertical_speed = VerticalSpeed::Known(1.0);

        assert_eq!(u8::from(vertical_speed), 2);
    }

    #[test]
    fn test_encode_invalid() {
        let invalid = VerticalSpeed::Invalid;

        assert_eq!(u8::from(invalid), 0);
    }

    #[test]
    fn test_encode_unknown() {
        let unknown = VerticalSpeed::Unknown;

        assert_eq!(u8::from(unknown), (VerticalSpeed::UNKNOWN_CODE * 2.0) as u8);
    }

    #[test]
    fn test_encode_no_value() {
        let no_value = VerticalSpeed::NoValue;

        assert_eq!(u8::from(no_value), 0);
    }

    #[test]
    fn test_encode_invalid_state() {
        let invalid_state = VerticalSpeed::Known(63.0);

        assert_eq!(u8::from(invalid_state), 124);
    }

    #[test]
    fn test_decode() {
        let decoded = VerticalSpeed::from(2);

        assert_eq!(decoded, VerticalSpeed::Known(1.0));
    }

    #[test]
    fn test_decode_unknown() {
        let decoded = VerticalSpeed::from(126);

        assert_eq!(decoded, VerticalSpeed::Unknown);
    }
}
