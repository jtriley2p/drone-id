use crate::error::Error;
use crate::try_serialize::TrySerialize;

/// Unmanned Aerial System (UAS) Traffic Management (UTM) session-issued Unique User ID (UUID).
///
/// The format appears to be unspecified, though [`crate::basic_id::BasicID`] message payloads are
/// always limited to 20 bytes.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct UTMAssignedUUID([u8; 20]);

impl UTMAssignedUUID {
    /// Constructs a new UTM Assigned UUID
    pub fn new(uuid: [u8; 20]) -> Self {
        Self(uuid)
    }

    /// Returns the UUID.
    pub fn uuid(&self) -> &[u8] {
        &self.0
    }
}

impl TrySerialize for UTMAssignedUUID {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        if buffer.len() != 20 {
            return Err(Error::InvalidDataLength);
        }

        buffer.clone_from_slice(&self.0);

        Ok(())
    }
}

impl TryFrom<&[u8]> for UTMAssignedUUID {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 20 {
            return Err(Error::InvalidDataLength);
        }

        let value = value.try_into().map_err(|_| Error::Unreachable).unwrap();

        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::{basic_id::UTMAssignedUUID, try_serialize::TrySerialize};

    #[test]
    fn test_encode() {
        let uuid = [2u8; 20];

        let utm_assigned_uuid = UTMAssignedUUID::try_from(uuid.as_ref()).unwrap();

        assert_eq!(utm_assigned_uuid.uuid(), uuid);
    }

    #[test]
    fn test_decode() {
        let encoded = [2u8; 20];

        let decoded = UTMAssignedUUID::try_from(encoded.as_ref()).unwrap();

        assert_eq!(encoded, decoded.uuid());
    }

    #[test]
    fn test_encode_fails_invalid_length() {
        let too_short = [0u8; 19];
        let too_long = [0u8; 21];

        assert!(UTMAssignedUUID::try_from(too_short.as_ref()).is_err());
        assert!(UTMAssignedUUID::try_from(too_long.as_ref()).is_err());
    }

    #[test]
    fn test_decode_fails_invalid_length() {
        let mut too_short = [0u8; 19];
        let mut too_long = [0u8; 21];

        let utm_assigned_uuid = UTMAssignedUUID::try_from([2u8; 20].as_ref()).unwrap();

        assert!(utm_assigned_uuid.try_serialize(&mut too_short).is_err());
        assert!(utm_assigned_uuid.try_serialize(&mut too_long).is_err());
    }
}
