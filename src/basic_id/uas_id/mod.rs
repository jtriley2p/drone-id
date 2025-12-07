//! ## UAS Identification
//!
//! Unmanned Aerial Systems

mod registration_id;
mod serial_number;
mod session_id;
mod utm_assigned_uuid;

pub use registration_id::RegistrationID;
pub use serial_number::SerialNumber;
pub use session_id::SessionID;
pub use session_id::SessionIDType;
pub use utm_assigned_uuid::UTMAssignedUUID;

use crate::error::Error;
use crate::try_serialize::TrySerialize;

/// Unmanned Aerial System Identifier
///
/// Enumerates one of several possible identifiers.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UASID {
    /// No identifier provided.
    None,
    /// Manufacturer-issued code and serial number.
    SerialNumber(SerialNumber),
    /// Civil Aviation Authority-issued registration number.
    RegistrationID(RegistrationID),
    /// UAS Traffic Management system-issued identifier.
    UTMAssignedUUID(UTMAssignedUUID),
    /// Session ID
    ///
    /// Either IETF DRIP entity ID or IEEE 1609.2-2016 HashedID8.
    ///
    /// See [`SessionID`] for more.
    SessionID(SessionID),
}

impl TryFrom<&[u8]> for UASID {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 21 {
            return Err(Error::InvalidDataLength);
        }
        let id_type = value[0] >> 4;

        let id_bytes = &value[1..];

        match id_type {
            0 => Ok(Self::None),
            1 => Ok(Self::SerialNumber(id_bytes.try_into()?)),
            2 => Ok(Self::RegistrationID(id_bytes.try_into()?)),
            3 => Ok(Self::UTMAssignedUUID(id_bytes.try_into()?)),
            4 => Ok(Self::SessionID(id_bytes.try_into()?)),
            _ => Err(Error::InvalidInteger),
        }
    }
}

impl TrySerialize for UASID {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        if buffer.len() != 21 {
            return Err(Error::InvalidDataLength);
        }

        match self {
            Self::None => (),
            Self::SerialNumber(serial_number) => {
                buffer[0] |= 1 << 4;
                serial_number.try_serialize(&mut buffer[1..])?;
            }
            Self::RegistrationID(registration_id) => {
                buffer[0] |= 2 << 4;
                registration_id.try_serialize(&mut buffer[1..])?;
            }
            Self::UTMAssignedUUID(utm_assigned_uuid) => {
                buffer[0] |= 3 << 4;
                utm_assigned_uuid.try_serialize(&mut buffer[1..])?;
            }
            Self::SessionID(session_id) => {
                buffer[0] |= 4 << 4;
                session_id.try_serialize(&mut buffer[1..])?;
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{basic_id::{UASID, UTMAssignedUUID}, try_serialize::TrySerialize};

    #[test]
    fn test_encode() {
        let uuid = [2u8; 20];

        let utm_assigned_uuid = UTMAssignedUUID::try_from(uuid.as_ref()).unwrap();

        let uas_id = UASID::UTMAssignedUUID(utm_assigned_uuid);

        let mut encoded = [0u8; 21];
        uas_id.try_serialize(&mut encoded).unwrap();

        assert_eq!(encoded[0] >> 4, 3);
        assert_eq!(encoded[1..], uuid);
    }

    #[test]
    fn test_decode() {
        let uuid = [2u8; 20];

        let utm_assigned_uuid = UTMAssignedUUID::try_from(uuid.as_ref()).unwrap();

        let uas_id = UASID::UTMAssignedUUID(utm_assigned_uuid);

        let mut encoded = [0u8; 21];
        uas_id.try_serialize(&mut encoded).unwrap();

        let decoded = UASID::try_from(encoded.as_ref()).unwrap();

        assert_eq!(uas_id, decoded);
    }

    #[test]
    fn test_encode_fails_invalid_length() {
        let mut too_short = [0u8; 20];
        let mut too_long = [0u8; 22];

        let utm_assigned_uuid = UTMAssignedUUID::try_from([2u8; 20].as_ref()).unwrap();

        let uas_id = UASID::UTMAssignedUUID(utm_assigned_uuid);

        assert!(uas_id.try_serialize(&mut too_short).is_err());
        assert!(uas_id.try_serialize(&mut too_long).is_err());
    }

    #[test]
    fn test_decode_fails_invalid_length() {
        let too_short = [0u8; 20];
        let too_long = [0u8; 22];

        assert!(UASID::try_from(too_short.as_ref()).is_err());
        assert!(UASID::try_from(too_long.as_ref()).is_err());
    }

    #[test]
    fn test_decode_fails_invalid_uas_id_type() {
        let invalid = 5;

        let mut encoded = [0u8; 21];
        encoded[0] = invalid << 4;

        assert!(UASID::try_from(encoded.as_ref()).is_err());
    }
}
