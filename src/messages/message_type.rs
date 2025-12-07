use crate::authentication::Authentication;
use crate::basic_id::BasicID;
use crate::error::Error;
use crate::location::Location;
use crate::operator_id::OperatorID;
use crate::pack::Pack;
use crate::self_id::SelfID;
use crate::system::System;
use crate::try_serialize::TrySerialize;

/// Type of Message
///
/// This enumerates the internal message types as well.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MessageType {
    /// Basic ID.
    BasicID(BasicID),
    /// Location.
    Location(Location),
    /// Authentication.
    Authentication(Authentication),
    /// Self ID
    SelfID(SelfID),
    /// System.
    System(System),
    /// Operator ID
    OperatorID(OperatorID),
    /// Message Pack
    Pack(Pack),
}

impl TryFrom<&[u8]> for MessageType {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 25 {
            return Err(Error::InvalidDataLength);
        }

        let message_type = value[0] >> 4;
        let value = &value[1..25];

        // we exit here for pack first because all other message types are only 25 bytes long, while
        // the pack message can extend out to 227 bytes.
        if message_type == 0x0f {
            return Ok(MessageType::Pack(value.try_into()?));
        }

        match message_type {
            0x00 => Ok(MessageType::BasicID(value.try_into()?)),
            0x01 => Ok(MessageType::Location(value.try_into()?)),
            0x02 => Ok(MessageType::Authentication(value.try_into()?)),
            0x03 => Ok(MessageType::SelfID(value.try_into()?)),
            0x04 => Ok(MessageType::System(value.try_into()?)),
            0x05 => Ok(MessageType::OperatorID(value.try_into()?)),
            _ => Err(Error::InvalidInteger),
        }
    }
}

impl TrySerialize for MessageType {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        // we exit here for pack first because all other message types are only 25 bytes long, while
        // the pack message can extend out to 227 bytes.
        if let Self::Pack(pack) = self {
            buffer[0] |= 0x0f << 4;
            return pack.try_serialize(&mut buffer[1..]);
        }

        if buffer.len() != 25 {
            return Err(Error::InvalidDataLength);
        }

        match self {
            Self::BasicID(basic_id) => basic_id.try_serialize(&mut buffer[1..]),
            Self::Location(location) => {
                buffer[0] |= 1 << 4;
                location.try_serialize(&mut buffer[1..])
            }
            Self::Authentication(authentication) => {
                buffer[0] |= 2 << 4;
                authentication.try_serialize(&mut buffer[1..])
            }
            Self::SelfID(self_id) => {
                buffer[0] |= 3 << 4;
                self_id.try_serialize(&mut buffer[1..])
            }
            Self::System(system) => {
                buffer[0] |= 4 << 4;
                system.try_serialize(&mut buffer[1..])
            }
            Self::OperatorID(operator_id) => {
                buffer[0] |= 5 << 4;
                operator_id.try_serialize(&mut buffer[1..])
            }
            _ => Err(Error::Unreachable).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        basic_id::{BasicID, UASID, UAType},
        messages::MessageType,
        try_serialize::TrySerialize,
    };

    #[test]
    fn test_encode() {
        let basic_id = BasicID::new(UAType::Ornithopter, UASID::None);
        let message_type = MessageType::BasicID(basic_id);

        let mut encoded_basic_id = [0u8; 24];
        basic_id.try_serialize(&mut encoded_basic_id).unwrap();

        let mut encoded = [0u8; 25];
        message_type.try_serialize(&mut encoded).unwrap();

        assert_eq!(encoded[0], 0 << 4);
        assert_eq!(encoded[1..], encoded_basic_id);
    }

    #[test]
    fn test_encode_fails_invalid_data_length() {
        let mut too_short = [0u8; 24];
        let mut too_long = [0u8; 26];

        let message_type = MessageType::BasicID(BasicID::new(UAType::Ornithopter, UASID::None));

        assert!(message_type.try_serialize(&mut too_short).is_err());
        assert!(message_type.try_serialize(&mut too_long).is_err());
    }

    #[test]
    fn test_decode() {
        let basic_id = BasicID::new(UAType::Ornithopter, UASID::None);

        let mut encoded = [0u8; 25];
        basic_id.try_serialize(&mut encoded[1..]).unwrap();

        let decoded = MessageType::try_from(encoded.as_ref()).unwrap();

        assert_eq!(decoded, MessageType::BasicID(basic_id));
    }

    #[test]
    fn test_decode_fails_invalid_data_length() {
        let too_short = [0u8; 24];
        let too_long = [0u8; 26];

        assert!(MessageType::try_from(too_short.as_ref()).is_err());
        assert!(MessageType::try_from(too_long.as_ref()).is_err());
    }

    #[test]
    fn test_decode_fails_invalid_message_type() {
        let mut invalid = [0u8; 25];
        invalid[0] = 0x06 << 4;

        assert!(MessageType::try_from(invalid.as_ref()).is_err());
    }
}
