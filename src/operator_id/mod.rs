//! ## Operator ID Message
//!
//! [`OperatorID`] messages contain an identifier unique to the UAS operator, often issued by the
//! operator's respective Civil Aviation Authority.
mod operator_id_type;

pub use operator_id_type::OperatorIDType;

use crate::error::Error;
use crate::try_serialize::TrySerialize;

/// Operator ID Message
///
/// Identifies the operator with a unique identifier issued by their respective Civil Aviation
/// Authority.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct OperatorID {
    operator_id_type: OperatorIDType,
    id: [u8; 20],
}

impl OperatorID {
    /// Constructs a new Operator ID
    pub fn new(operator_id_type: OperatorIDType, id: [u8; 20]) -> Self {
        Self {
            operator_id_type,
            id,
        }
    }

    /// Returns the operator ID type.
    pub fn operator_id_type(&self) -> OperatorIDType {
        self.operator_id_type
    }

    /// Returns the raw ID data.
    pub fn id(&self) -> &[u8; 20] {
        &self.id
    }
}

impl TryFrom<&[u8]> for OperatorID {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        let operator_id_type = value[0].into();

        let id = value[1..21]
            .try_into()
            .map_err(|_| Error::Unreachable)
            .unwrap();

        Ok(Self {
            operator_id_type,
            id,
        })
    }
}

impl TrySerialize for OperatorID {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        if buffer.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        buffer[0] = u8::from(self.operator_id_type);
        buffer[1..21].clone_from_slice(&self.id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{operator_id::{OperatorID, OperatorIDType}, try_serialize::TrySerialize};

    #[test]
    fn test_getters() {
        let operator_id_type = OperatorIDType::OperatorID;
        let id = [2u8; 20];

        let operator_id = OperatorID::new(operator_id_type, id);

        assert_eq!(operator_id.operator_id_type(), operator_id_type);
        assert_eq!(operator_id.id(), &id);
    }

    #[test]
    fn test_encode() {
        let operator_id_type = OperatorIDType::OperatorID;
        let id = [2u8; 20];

        let operator_id = OperatorID::new(operator_id_type, id);

        let mut encoded = [0u8; 24];
        operator_id.try_serialize(&mut encoded).unwrap();

        assert_eq!(encoded[0], u8::from(operator_id_type));
        assert_eq!(encoded[1..21], id);
    }

    #[test]
    fn test_encode_fails_invalid_data_length() {
        let mut too_short = [0u8; 19];
        let mut too_long = [0u8; 21];

        let operator_id = OperatorID::new(OperatorIDType::OperatorID, [2u8; 20]);

        assert!(operator_id.try_serialize(&mut too_short).is_err());
        assert!(operator_id.try_serialize(&mut too_long).is_err());
    }

    #[test]
    fn test_decode() {
        let mut encoded = [0u8; 24];
        encoded[1..21].clone_from_slice(&[2u8; 20]);

        let expected = OperatorID::new(OperatorIDType::OperatorID, [2u8; 20]);

        assert_eq!(OperatorID::try_from(encoded.as_ref()).unwrap(), expected);
    }

    #[test]
    fn test_decode_fails_invalid_data_length() {
        let too_short = [0u8; 19];
        let too_long = [0u8; 21];

        assert!(OperatorID::try_from(too_short.as_ref()).is_err());
        assert!(OperatorID::try_from(too_long.as_ref()).is_err());
    }
}
