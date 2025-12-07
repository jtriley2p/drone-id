use crate::error::Error;

/// Unmanned Aircraft Classification
///
/// If classification is set to [`UAClassification::Open`] (`1`), it includes an encoded form of
/// [`OpenClassification`] internally. Otherwise it is empty.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UAClassification {
    /// Undefined classification.
    Undefined,
    /// Open classification.
    ///
    /// See [`OpenClassification`] for internal enumeration.
    Open(OpenClassification),
    /// Specific classification.
    ///
    /// Included in specification but does not elaborate.
    Specific,
    /// Certified classification.
    ///
    /// Included in specification but does not elaborate.
    Certified,
    /// Reserved.
    Reserved,
}

impl From<u8> for UAClassification {
    fn from(value: u8) -> Self {
        let ua_classification = value >> 4;
        // never fails when `OpenClassification` only receives 4 bits
        let open_classification = (value & 0b0000_1111)
            .try_into()
            .map_err(|_| Error::Unreachable)
            .unwrap();

        match ua_classification {
            0 => Self::Undefined,
            1 => Self::Open(open_classification),
            2 => Self::Specific,
            3 => Self::Certified,
            _ => Self::Reserved,
        }
    }
}

impl From<UAClassification> for u8 {
    fn from(value: UAClassification) -> Self {
        match value {
            UAClassification::Undefined => 0,
            UAClassification::Open(open_classification) => 1 << 4 | u8::from(open_classification),
            UAClassification::Specific => 2 << 4,
            UAClassification::Certified => 3 << 4,
            UAClassification::Reserved => 4 << 4,
        }
    }
}

/// Open Classification
///
/// Generic system which can also be converted to region specific classification.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OpenClassification {
    /// Undefined open classification.
    Undefined,
    /// Class 0.
    Class0,
    /// Class 1.
    Class1,
    /// Class 2.
    Class2,
    /// Class 3.
    Class3,
    /// Class 4.
    Class4,
    /// Class 5.
    Class5,
    /// Class 6.
    Class6,
    /// Reserved.
    Reserved,
}

impl TryFrom<u8> for OpenClassification {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 7 && value < 16 {
            return Ok(Self::Reserved);
        }

        match value {
            0 => Ok(Self::Undefined),
            1 => Ok(Self::Class0),
            2 => Ok(Self::Class1),
            3 => Ok(Self::Class2),
            4 => Ok(Self::Class3),
            5 => Ok(Self::Class4),
            6 => Ok(Self::Class5),
            7 => Ok(Self::Class6),
            _ => Err(Error::InvalidInteger),
        }
    }
}

impl From<OpenClassification> for u8 {
    fn from(value: OpenClassification) -> Self {
        value as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::system::{OpenClassification, UAClassification};

    #[test]
    fn test_encode_ua_classification() {
        let ua_classification = UAClassification::Specific;

        assert_eq!(u8::from(ua_classification), 2 << 4);
    }

    #[test]
    fn test_encode_ua_classification_reserved() {
        let reserved = UAClassification::Reserved;

        assert_eq!(u8::from(reserved), 4 << 4);
    }

    #[test]
    fn test_encode_ua_classification_open() {
        let class0 = OpenClassification::Class0;
        let open = UAClassification::Open(class0);

        let expected = 1 << 4 | u8::from(class0);

        assert_eq!(u8::from(open), expected);
    }

    #[test]
    fn test_decode_ua_classification() {
        let decoded = UAClassification::from(2 << 4);

        assert_eq!(decoded, UAClassification::Specific);
    }

    #[test]
    fn test_decode_ua_classification_reserved() {
        let decoded = UAClassification::from(4 << 4);

        assert_eq!(decoded, UAClassification::Reserved);
    }

    #[test]
    fn test_decode_ua_classification_open() {
        let decoded = UAClassification::from(1 << 4 | 1);

        assert_eq!(decoded, UAClassification::Open(OpenClassification::Class0));
    }

    #[test]
    fn test_encode_open_classification() {
        let open_classification = OpenClassification::Class0;

        assert_eq!(u8::from(open_classification), 1);
    }

    #[test]
    fn test_decode_open_classification() {
        let decoded = OpenClassification::try_from(1).unwrap();

        assert_eq!(decoded, OpenClassification::Class0);
    }

    #[test]
    fn test_decode_open_classification_fails_invalid_integer() {
        assert!(OpenClassification::try_from(16).is_err());
    }
}
