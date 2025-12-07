//! ## Basic ID Message
//!
//! [`BasicID`] messages indicate what type of UAS is used, as well as a unique identifier,
//! generally either a manufacturer-assigned serial number or a registration identifier issued by
//! the pilot's Civil Aviation Authority.
//!
//! One of the following identifier types is encapsulated in the [`BasicID`] data type:
//!
//! 1. [`SerialNumber`] contains the manufacturer's code and serial number (ANSI/CTA-2063-A).
//! 2. [`RegistrationID`] contains the Civil Aviation Authority's registration number.
//! 3. [`UTMAssignedUUID`] contains a session-level identifier for UAS Traffic Management systems.
//! 4. [`SessionID`] contains a [`SessionIDType`] and its relevant identifier information.
mod ua_type;
mod uas_id;

pub use ua_type::UAType;
pub use uas_id::RegistrationID;
pub use uas_id::SerialNumber;
pub use uas_id::SessionID;
pub use uas_id::SessionIDType;
pub use uas_id::UASID;
pub use uas_id::UTMAssignedUUID;

use crate::error::Error;
use crate::try_serialize::TrySerialize;

/// Basic ID Message
///
/// Encapsulates a unmanned aircraft type and an enumerated, unique identifier.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BasicID {
    /// Unmanned aircraft type.
    ua_type: UAType,
    /// Enumerated Identifier.
    uas_id: UASID,
}

impl BasicID {
    /// Creates a new BasicID manually.
    pub fn new(ua_type: UAType, uas_id: UASID) -> Self {
        Self { ua_type, uas_id }
    }

    /// Returns the type of unmanned aircraft.
    pub fn ua_type(&self) -> UAType {
        self.ua_type
    }

    /// Returns the unmanned aerial system identifier.
    pub fn uas_id(&self) -> UASID {
        self.uas_id
    }
}

impl TryFrom<&[u8]> for BasicID {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        let ua_type = (value[0] & 0b0000_1111).try_into()?;

        let uas_id = value[..21].try_into()?;

        Ok(Self { ua_type, uas_id })
    }
}

impl TrySerialize for BasicID {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        if buffer.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        buffer[0] = u8::from(self.ua_type);

        self.uas_id.try_serialize(&mut buffer[..21])
    }
}

#[cfg(test)]
mod tests {
    use crate::{basic_id::{BasicID, UASID, UAType}, try_serialize::TrySerialize};

    #[test]
    fn test_new() {
        let ua_type = UAType::Ornithopter;
        let uas_id = UASID::None;

        let basic_id = BasicID::new(ua_type, uas_id);

        assert_eq!(basic_id.ua_type(), ua_type);
        assert_eq!(basic_id.uas_id(), uas_id);
    }

    #[test]
    fn test_encode() {
        let ua_type = UAType::Ornithopter;
        let uas_id = UASID::None;

        let basic_id = BasicID::new(ua_type, uas_id);

        let mut encoded = [0u8; 24];
        basic_id.try_serialize(&mut encoded).unwrap();

        assert_eq!(encoded[0], u8::from(ua_type));
        assert_eq!(&encoded[1..], &[0u8; 23]);
    }

    #[test]
    fn test_decode() {
        let ua_type = UAType::Ornithopter;
        let uas_id = UASID::None;

        let basic_id = BasicID::new(ua_type, uas_id);

        let mut encoded = [0u8; 24];
        basic_id.try_serialize(&mut encoded).unwrap();

        let decoded = BasicID::try_from(encoded.as_ref()).unwrap();

        assert_eq!(basic_id, decoded);
    }

    #[test]
    fn test_encode_fails_invalid_length() {
        let mut too_short = [0u8; 23];
        let mut too_long = [0u8; 25];

        let basic_id = BasicID::new(UAType::Ornithopter, UASID::None);

        assert!(basic_id.try_serialize(&mut too_short).is_err());
        assert!(basic_id.try_serialize(&mut too_long).is_err());
    }

    #[test]
    fn test_decode_fails_invalid_length() {
        let too_short = [0u8; 23];
        let too_long = [0u8; 25];

        assert!(BasicID::try_from(too_short.as_ref()).is_err());
        assert!(BasicID::try_from(too_long.as_ref()).is_err());
    }
}
