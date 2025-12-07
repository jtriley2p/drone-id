use crate::error::Error;
use crate::try_serialize::TrySerialize;

/// Session ID Type
///
/// First byte of the [`SessionID`], signals which format it is using.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SessionIDType {
    /// Reserved.
    Reserved,
    /// Internet Engineering Task Force (IETF) Drone Remote Id Protocol (DRIP) entity ID.
    IETFDroneRemoteIDProtocol,
    /// IEEE 1609.2-2016 HashedID8.
    IEEE16092HashedID8,
}

impl TryFrom<u8> for SessionIDType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Reserved),
            1 => Ok(Self::IETFDroneRemoteIDProtocol),
            2 => Ok(Self::IEEE16092HashedID8),
            _ => Err(Error::InvalidInteger),
        }
    }
}

impl From<SessionIDType> for u8 {
    fn from(value: SessionIDType) -> Self {
        value as u8
    }
}

/// Session ID
///
/// Consists of one byte indicating the [`SessionIDType`](SessionIDType) followed by 19 bytes of the unique session
/// ID.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SessionID {
    session_id_type: SessionIDType,
    id: [u8; 19],
}

impl SessionID {
    /// Constructs a new session ID.
    pub fn new(session_id_type: SessionIDType, id: [u8; 19]) -> Self {
        Self {
            session_id_type,
            id,
        }
    }

    /// Returns the session ID type.
    pub fn session_id_type(&self) -> SessionIDType {
        self.session_id_type
    }

    /// Returns the inner bytes (including the session ID type).
    pub fn id(&self) -> &[u8; 19] {
        &self.id
    }
}

impl TryFrom<&[u8]> for SessionID {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 20 {
            return Err(Error::InvalidDataLength);
        }

        let session_id_type = value[0].try_into()?;

        let id = value[1..20]
            .try_into()
            .map_err(|_| Error::Unreachable)
            .unwrap();

        Ok(Self {
            session_id_type,
            id,
        })
    }
}

impl TrySerialize for SessionID {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        if buffer.len() != 20 {
            return Err(Error::InvalidDataLength);
        }

        buffer[0] = u8::from(self.session_id_type);
        buffer[1..].clone_from_slice(&self.id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        basic_id::{SessionID, SessionIDType},
        try_serialize::TrySerialize,
    };

    #[test]
    fn test_session_id_type_encode_decode() {
        let ietf_drip_byte = 1;

        let decoded = SessionIDType::try_from(ietf_drip_byte).unwrap();

        let encoded = u8::from(decoded);

        assert_eq!(ietf_drip_byte, encoded);
    }

    #[test]
    fn test_session_id_type_decode_invalid() {
        let invalid = 3;

        assert!(SessionIDType::try_from(invalid).is_err());
    }

    #[test]
    fn test_new_session_id() {
        let ietf_drip = SessionIDType::IETFDroneRemoteIDProtocol;
        let id = [2u8; 19];

        let session_id = SessionID::new(ietf_drip, id);

        assert_eq!(session_id.session_id_type(), ietf_drip);
        assert_eq!(session_id.id(), &id);
    }

    #[test]
    fn test_session_id_encode() {
        let ietf_drip = SessionIDType::IETFDroneRemoteIDProtocol;
        let id = [2u8; 19];

        let session_id = SessionID::new(ietf_drip, id);

        let mut encoded = [0u8; 20];
        session_id.try_serialize(&mut encoded).unwrap();

        assert_eq!(encoded[0], ietf_drip.into());
        assert_eq!(&encoded[1..], id.as_ref());
    }

    #[test]
    fn test_session_id_decode() {
        let ietf_drip = SessionIDType::IETFDroneRemoteIDProtocol;
        let id = [2u8; 19];

        let session_id = SessionID::new(ietf_drip, id);

        let mut encoded = [0u8; 20];
        session_id.try_serialize(&mut encoded).unwrap();

        let decoded = SessionID::try_from(encoded.as_ref()).unwrap();

        assert_eq!(decoded, session_id);
    }

    #[test]
    fn test_session_id_encode_fails_invalid_length() {
        let mut too_short = [0u8; 19];
        let mut too_long = [0u8; 21];

        let ietf_drip = SessionIDType::IETFDroneRemoteIDProtocol;
        let id = [2u8; 19];

        let session_id = SessionID::new(ietf_drip, id);

        assert!(session_id.try_serialize(&mut too_short).is_err());
        assert!(session_id.try_serialize(&mut too_long).is_err());
    }

    #[test]
    fn test_session_id_decode_fails_invalid_length() {
        let too_short = [0u8; 19];
        let too_long = [0u8; 21];

        assert!(SessionID::try_from(too_short.as_ref()).is_err());
        assert!(SessionID::try_from(too_long.as_ref()).is_err());
    }
}
