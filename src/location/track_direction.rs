use core::cmp::Ordering;

/// Flight Direction
///
/// Measured from True North, clockwise degrees with a resolution of 1 degree. The
/// [`TrackDirection::Unknown`] value is 361 degrees. If the aircraft is not moving horizontally,
/// return the unknown value.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TrackDirection {
    /// Invalid value (values greater than 361).
    Invalid,
    /// No value.
    NoValue,
    /// Unknown value (indicated by 361).
    Unknown,
    /// Known, valid value.
    Known(u16),
}

impl TrackDirection {
    /// Special value which represents [`TrackDirection::Unknown`].
    pub const UNKNOWN_CODE: u16 = 361;

    /// Special value which represents [`TrackDirection::Invalid`].
    pub const INVALID_CODE: u16 = 362;

    /// Offset value for when the `east_west_bit` is set to true.
    pub const EAST_WEST_OFFSET: u16 = 180;

    /// Returns the inner direction.
    pub fn direction(&self) -> u16 {
        // in the event a library consumer manually constructs an invalid state, we return `362`,
        // the invalid value.
        match self {
            TrackDirection::Invalid | TrackDirection::NoValue => Self::INVALID_CODE,
            TrackDirection::Unknown => Self::UNKNOWN_CODE,
            TrackDirection::Known(n) => (*n).clamp(0, Self::INVALID_CODE),
        }
    }
}

impl From<(bool, u8)> for TrackDirection {
    fn from(value: (bool, u8)) -> Self {
        let (east_west_bit, angle) = value;

        let angle = match east_west_bit {
            true => angle as u16 + Self::EAST_WEST_OFFSET,
            false => angle as u16,
        };

        match angle.cmp(&Self::UNKNOWN_CODE) {
            Ordering::Greater => Self::Invalid,
            Ordering::Equal => Self::Unknown,
            Ordering::Less => Self::Known(angle),
        }
    }
}

impl From<TrackDirection> for (bool, u8) {
    fn from(value: TrackDirection) -> Self {
        let value = match value {
            TrackDirection::Invalid | TrackDirection::NoValue => 362,
            TrackDirection::Unknown => 361,
            TrackDirection::Known(n) => n,
        };

        let value = if value > 362 { 362 } else { value };

        match value > TrackDirection::EAST_WEST_OFFSET {
            true => (true, (value - TrackDirection::EAST_WEST_OFFSET) as u8),
            false => (false, value as u8),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::location::TrackDirection;

    #[test]
    fn test_direction() {
        assert_eq!(TrackDirection::Invalid.direction(), TrackDirection::INVALID_CODE);
        assert_eq!(TrackDirection::NoValue.direction(), TrackDirection::INVALID_CODE);
        assert_eq!(
            TrackDirection::Unknown.direction(),
            TrackDirection::UNKNOWN_CODE
        );
        assert_eq!(TrackDirection::Known(1).direction(), 1);
    }

    #[test]
    fn test_encode() {
        let (east_west_bit, angle) = TrackDirection::Known(1).into();

        assert!(!east_west_bit);
        assert_eq!(angle, 1);
    }

    #[test]
    fn test_encode_unknown() {
        let (east_west_bit, angle) = TrackDirection::Unknown.into();

        assert!(east_west_bit);
        assert_eq!(
            angle,
            (TrackDirection::UNKNOWN_CODE - TrackDirection::EAST_WEST_OFFSET) as u8
        );
    }

    #[test]
    fn test_encode_invalid() {
        let (east_west_bit, angle) = TrackDirection::Invalid.into();

        assert!(east_west_bit);
        assert_eq!(
            angle,
            (TrackDirection::INVALID_CODE - TrackDirection::EAST_WEST_OFFSET) as u8
        );
    }

    #[test]
    fn test_encode_invalid_state() {
        let (east_west_bit, angle) = TrackDirection::Known(363).into();

        assert!(east_west_bit);
        assert_eq!(
            angle,
            (TrackDirection::INVALID_CODE - TrackDirection::EAST_WEST_OFFSET) as u8
        );
    }

    #[test]
    fn test_decode() {
        let decoded = TrackDirection::from((false, 1));

        assert_eq!(decoded, TrackDirection::Known(1));
    }

    #[test]
    fn test_decode_unknown() {
        let decoded = TrackDirection::from((
            true,
            (TrackDirection::UNKNOWN_CODE - TrackDirection::EAST_WEST_OFFSET) as u8,
        ));

        assert_eq!(decoded, TrackDirection::Unknown);
    }

    #[test]
    fn test_decode_invalid() {
        let decoded = TrackDirection::from((
            true,
            (TrackDirection::INVALID_CODE - TrackDirection::EAST_WEST_OFFSET) as u8,
        ));

        assert_eq!(decoded, TrackDirection::Invalid);
    }
}
