/// Ground Speed
///
/// Measured in meters per second, minimum resolution is 0.25m/s.
///
/// A value of 255m/s is treated as "unknown", anything else above 254.25m/s is clamped down to
/// 254.25m/s. Encoding the 32-bit floating point number depends on if it's greater than the
/// [`GroundSpeed::PRECISION_THRESHOLD`] (63.75 m/s).
///
/// If the value is greater than the [`GroundSpeed::PRECISION_THRESHOLD`], we encode the value as an
/// 8-bit integer repreesnting 0.75 m/s increments and return a flag value of `true` which tells the
/// decoder to use low precision.
///
/// If the value is less than the [`GroundSpeed::PRECISION_THRESHOLD`], we encode the value as an
/// 8-bit integer representing 0.25 m/s increments and return a flag value of `false` which tells
/// the decoder to use high precision.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GroundSpeed {
    /// Invalid ground speed.
    Invalid,
    /// No value provided.
    NoValue,
    /// Unknown value (indicated by 255).
    Unknown,
    /// Known, valid value.
    Known(f32),
}

impl GroundSpeed {
    /// Special value if the actual value is unknown.
    pub const UNKNOWN_CODE: f32 = 255.0;

    /// Maximum speed value; anything greater than this is clamped down to this.
    pub const MAX: f32 = 254.0;

    /// If using high precision, the value represents increments of 0.25 m/s.
    pub const HIGH_PRECISION_UNIT: f32 = 0.25;

    /// If using low precision, the value represents increments of 0.75 m/s.
    pub const LOW_PRECISION_UNIT: f32 = 0.75;

    /// Threshold where a value greater than this uses low precision and one less than or equal to
    /// it will use high precision.
    pub const PRECISION_THRESHOLD: f32 = 63.75;

    fn encode_speed(speed: f32) -> (bool, u8) {
        // if speed is greater than or equal to 254.25: use low precision, clamp to 254
        if speed >= Self::MAX {
            return (true, 254);
        }

        // if speed is less than or equal to 63.75: use high precision
        if speed <= Self::PRECISION_THRESHOLD {
            // UNSAFE REASON: A safe API is not included without the std library. This is safe
            // because the max `speed` value at this point is 63.75, implying the max encoded value
            // is 255.
            return unsafe { (false, f32::to_int_unchecked(speed / 0.25)) };
        }

        // otherwise, value is between 63.75 and 254.25; use low precision
        //
        // UNSAFE REASON: A safe API is not included without the std library. This is safe because
        // the max `speed` value at this point is 254.24, implying the max encoded value is 253.987,
        // which rounds up to 254.
        return unsafe {
            (
                true,
                f32::to_int_unchecked((speed - Self::PRECISION_THRESHOLD) / 0.75),
            )
        };
    }
}

impl From<(bool, u8)> for GroundSpeed {
    fn from(value: (bool, u8)) -> Self {
        let (use_low_precision, speed) = (value.0, value.1 as f32);

        if speed == Self::UNKNOWN_CODE {
            return Self::Unknown;
        }

        let speed = match use_low_precision {
            true => speed * Self::LOW_PRECISION_UNIT + Self::PRECISION_THRESHOLD,
            false => speed * Self::HIGH_PRECISION_UNIT,
        };

        Self::Known(speed)
    }
}

impl From<GroundSpeed> for (bool, u8) {
    fn from(value: GroundSpeed) -> Self {
        match value {
            GroundSpeed::Invalid | GroundSpeed::NoValue => (false, 0),
            GroundSpeed::Unknown => (false, 255),
            GroundSpeed::Known(speed) => GroundSpeed::encode_speed(speed),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::location::GroundSpeed;

    #[test]
    fn test_encode_low_precision() {
        let speed = GroundSpeed::PRECISION_THRESHOLD + 0.75;

        let ground_speed = GroundSpeed::Known(speed);

        let (use_low_precision, encoded_speed) = ground_speed.into();

        let expected_speed: u8 =
            unsafe { f32::to_int_unchecked((speed - GroundSpeed::PRECISION_THRESHOLD) / 0.75) };

        assert!(use_low_precision);
        assert_eq!(encoded_speed, expected_speed);
    }

    #[test]
    fn test_encode_high_precision() {
        let speed = GroundSpeed::PRECISION_THRESHOLD;

        let ground_speed = GroundSpeed::Known(speed);

        let (use_low_precision, encoded_speed) = ground_speed.into();

        let expected_speed: u8 = unsafe { f32::to_int_unchecked(speed / 0.25) };

        assert!(!use_low_precision);
        assert_eq!(encoded_speed, expected_speed);
    }

    #[test]
    fn test_encode_unknown() {
        let unknown_ground_speed = GroundSpeed::Unknown;
        let also_unknown_ground_speed = GroundSpeed::Unknown;

        let (use_low_precision, encoded_speed) = unknown_ground_speed.into();
        let (also_use_low_precision, also_encoded_speed) = also_unknown_ground_speed.into();

        let expected = 255;

        assert!(!use_low_precision);
        assert_eq!(encoded_speed, expected);
        assert!(!also_use_low_precision);
        assert_eq!(also_encoded_speed, expected);
    }

    #[test]
    fn test_encode_clamped() {
        let big_speed = GroundSpeed::MAX + 1.0;
        let ground_speed = GroundSpeed::Known(big_speed);

        let (use_low_precision, encoded_speed) = ground_speed.into();

        assert!(use_low_precision);
        assert_eq!(encoded_speed, GroundSpeed::MAX as u8);
    }

    #[test]
    fn test_decode_low_precision() {
        let encoded = 1u8;
        let speed = GroundSpeed::PRECISION_THRESHOLD + 0.75;

        let ground_speed = GroundSpeed::from((true, encoded));

        assert_eq!(ground_speed, GroundSpeed::Known(speed));
    }

    #[test]
    fn test_decode_high_precision() {
        let encoded = 1u8;
        let speed = 0.25;

        let ground_speed = GroundSpeed::from((false, encoded));

        assert_eq!(ground_speed, GroundSpeed::Known(speed));
    }

    #[test]
    fn test_decode_unknown() {
        let unknown_code = GroundSpeed::UNKNOWN_CODE as u8;
        let unknown_ground_speed = GroundSpeed::from((true, unknown_code));
        let also_unknown_ground_speed = GroundSpeed::from((false, unknown_code));

        let (use_low_precision, encoded_speed) = unknown_ground_speed.into();
        let (also_use_low_precision, also_encoded_speed) = also_unknown_ground_speed.into();

        let expected = 255;

        assert!(!use_low_precision);
        assert_eq!(encoded_speed, expected);
        assert!(!also_use_low_precision);
        assert_eq!(also_encoded_speed, expected);
    }
}
