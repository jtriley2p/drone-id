//! ## Self ID Message
//!
//! [`SelfID`] is an optional message which can self identify or describe operations for observers
//! of the UAS operation. This message is simply a description type and an associated free-text
//! ASCII field. Types could be general text, emergency, or other private uses.
//!
//! Specification lists an example of a realtor sending a Self ID message indicating the operation
//! is for a property photo-shoot for a fictional concerned neighbor that's alarmed at the sight of
//! a drone yet has hardware and software capable of receiving, decoding, and displaying such
//! information.
//!
//! Nonetheless, the option is here.
mod description_type;

pub use description_type::DescriptionType;

use crate::error::Error;
use crate::try_serialize::TrySerialize;

/// Optional, Self Identifying Message
///
/// Description is a free-form ASCII text field, this can be any description of operations limited
/// to 23 characters.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SelfID {
    description_type: DescriptionType,
    description: [u8; 23],
}

impl SelfID {
    /// Constructs a new Self ID.
    pub fn new(description_type: DescriptionType, description: [u8; 23]) -> Self {
        Self {
            description_type,
            description,
        }
    }

    /// Returns the description type.
    pub fn description_type(&self) -> DescriptionType {
        self.description_type
    }

    /// Returns the raw description.
    ///
    /// Returns bytes, should be decodable to ASCII.
    pub fn description(&self) -> &[u8; 23] {
        &self.description
    }
}

impl TryFrom<&[u8]> for SelfID {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        let description_type = value[0].into();

        let description = value[1..]
            .try_into()
            .map_err(|_| Error::Unreachable)
            .unwrap();

        Ok(Self {
            description_type,
            description,
        })
    }
}

impl TrySerialize for SelfID {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        if buffer.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        buffer[0] = u8::from(self.description_type);

        buffer[1..].clone_from_slice(&self.description);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        self_id::{DescriptionType, SelfID},
        try_serialize::TrySerialize,
    };

    const TEXT: [u8; 23] = [
        97, 98, 111, 108, 105, 115, 104, 32, 105, 99, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    #[test]
    fn test_getters() {
        let description_type = DescriptionType::Text;
        let description = TEXT;

        let self_id = SelfID::new(description_type, description);

        assert_eq!(self_id.description_type(), description_type);
        assert_eq!(self_id.description(), &description);
    }

    #[test]
    fn test_encode() {
        let description_type = DescriptionType::Text;
        let description = TEXT;

        let self_id = SelfID::new(DescriptionType::Text, TEXT);

        let mut encoded = [0u8; 24];
        self_id.try_serialize(&mut encoded).unwrap();

        let mut expected = [0u8; 24];
        expected[0] = u8::from(description_type);
        expected[1..].clone_from_slice(&description);

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_fails_invalid_data_length() {
        let mut too_short = [0u8; 23];
        let mut too_long = [0u8; 25];

        let self_id = SelfID::new(DescriptionType::Text, TEXT);

        assert!(self_id.try_serialize(&mut too_short).is_err());
        assert!(self_id.try_serialize(&mut too_long).is_err());
    }

    #[test]
    fn test_decode() {
        let description_type = DescriptionType::Text;
        let description = TEXT;

        let mut encoded = [0u8; 24];
        encoded[0] = u8::from(description_type);
        encoded[1..].clone_from_slice(&description);

        let expected = SelfID::new(DescriptionType::Text, TEXT);

        assert_eq!(SelfID::try_from(encoded.as_ref()).unwrap(), expected);
    }

    #[test]
    fn test_decode_fails_invalid_data_length() {
        let too_short = [0u8; 23];
        let too_long = [0u8; 25];

        assert!(SelfID::try_from(too_short.as_ref()).is_err());
        assert!(SelfID::try_from(too_long.as_ref()).is_err());
    }
}
