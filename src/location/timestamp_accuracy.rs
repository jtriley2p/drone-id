/// Timestamp Accuracy
///
/// Accuracy is measured from a range of 0.1s to 1.5s, anything beyond these bounds are labelled
/// "unknown".
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TimestampAccuracy {
    /// Unknown value (indicated by a number greater than 15).
    Unknown,
    /// Known, valid value.
    Known(f32),
}

impl TimestampAccuracy {
    /// Special value representing the "unknown" value.
    pub const UNKNOWN_CODE: u8 = 0;

    /// Returns the internal accuracy value.
    pub fn accuracy(&self) -> f32 {
        match self {
            Self::Unknown => 0.0,
            Self::Known(n) => n.clamp(0.0, 1.5),
        }
    }
}

impl From<u8> for TimestampAccuracy {
    fn from(value: u8) -> Self {
        match value.clamp(0, 15) {
            Self::UNKNOWN_CODE => Self::Unknown,
            n => Self::Known(n as f32 / 10.0),
        }
    }
}

impl From<TimestampAccuracy> for u8 {
    fn from(value: TimestampAccuracy) -> Self {
        (value.accuracy() * 10.0) as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::location::TimestampAccuracy;

    #[test]
    fn test_accuracy() {
        assert_eq!(TimestampAccuracy::Known(0.1).accuracy(), 0.1);
        assert_eq!(TimestampAccuracy::Unknown.accuracy(), 0.0);
    }

    #[test]
    fn test_encode() {
        let accuracy = TimestampAccuracy::Known(0.2);

        assert_eq!(u8::from(accuracy), 2);
    }

    #[test]
    fn test_encode_unknown() {
        let unknown = TimestampAccuracy::Unknown;

        assert_eq!(u8::from(unknown), TimestampAccuracy::UNKNOWN_CODE);
    }

    #[test]
    fn test_decode() {
        let decoded = TimestampAccuracy::from(1);

        assert_eq!(decoded, TimestampAccuracy::Known(0.1));
    }

    #[test]
    fn test_decode_unknown() {
        let decoded = TimestampAccuracy::from(0);

        assert_eq!(decoded, TimestampAccuracy::Unknown);
    }
}
