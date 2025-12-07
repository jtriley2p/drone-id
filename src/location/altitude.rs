/// Altitude
///
/// Altitude value which can represent geodetic altitude based on a line from the WGS-84 ellipsoid
/// through the aircraft or pressure altitude based on barometric pressure.
///
/// Altitude MUST be in meters with a resolution of 1 meter.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Altitude {
    /// Invalid altitude value.
    Invalid,
    /// No value provided.
    NoValue,
    /// Unknown value (indicated by -1000).
    Unknown,
    /// Known, valid value.
    Known(f32),
}

impl Altitude {
    /// Special value representing the maximum value of "unknown".
    pub const UNKNOWN_CODE: f32 = -1_000.0;

    /// Returns the adjusted altitude value.
    pub fn altitude(&self) -> f32 {
        match self {
            Altitude::Known(n) => *n,
            _ => 0.0,
        }
    }
}

impl From<u16> for Altitude {
    fn from(value: u16) -> Self {
        let value = value as f32;

        match value * 0.5 - 1_000.0 {
            Self::UNKNOWN_CODE => Self::Unknown,
            n => Self::Known(n),
        }
    }
}

impl From<Altitude> for u16 {
    fn from(value: Altitude) -> Self {
        let n = match value {
            Altitude::Invalid | Altitude::NoValue => 0.0,
            Altitude::Unknown => Altitude::UNKNOWN_CODE,
            Altitude::Known(n) => n,
        };

        ((n + 1_000.0) / 0.5) as u16
    }
}

#[cfg(test)]
mod tests {
    use crate::location::Altitude;

    #[test]
    fn test_encode() {
        let value = 5f32;
        let altitude = Altitude::Known(value);

        let encoded = u16::from(altitude);

        let expected = ((value + 1_000.0) / 0.5) as u16;

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_unknown() {
        let unknown_code = 0;

        let altitude = Altitude::try_from(unknown_code).unwrap();

        assert_eq!(altitude, Altitude::Unknown);
    }

    #[test]
    fn test_decode() {
        let expected = 5f32;
        let value = ((expected + 1_000.0) / 0.5) as u16;

        let altitude = Altitude::from(value);

        assert_eq!(altitude.altitude(), expected);
    }

    #[test]
    fn test_decode_unknown() {
        let altitude = Altitude::Unknown;

        let unknown_code = 0;

        assert_eq!(u16::from(altitude), unknown_code);
    }
}
