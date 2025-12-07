//! ## Messages
//!
//! The [`Message`] data type in this module encapsulates all other messages into it. This should be
//! the entry point for most use cases.
//!
//! All internal data types are exposed through the library, so manual construction is possible,
//! though care must be taken to ensure expected properties hold. Structs can encapsulate their
//! fields such that construction may only happen under constrolled circumstances, but enumerations
//! cannot constrain such things. As such, especially manual construction of enumerated values must
//! be done with care.
//!
//! Deserialization and serialization can be performed as follows.
//!
//! ```rust
//! use drone_id::messages::{Message, MessageType};
//! use drone_id::self_id::{SelfID, DescriptionType};
//! use drone_id::try_serialize::TrySerialize;
//!
//! let message = Message::new(
//!     MessageType::SelfID(
//!         SelfID::new(
//!             DescriptionType::Text,
//!             [
//!                 97, 98, 111, 108, 105, 115, 104, 32, 105, 99, 101,
//!                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
//!             ],
//!         )
//!     )
//! );
//!
//! let mut buffer = [0u8; 25];
//! message.try_serialize(&mut buffer).unwrap();
//!
//! let deserialized_message = Message::try_from(buffer.as_ref()).unwrap();
//! ```
//!
//! Deserialization and serialization through this means should NEVER panic, any internal panic
//! would be a bug, instead it will enumerate all errors through [`crate::error::Error`].
mod message_type;

pub use message_type::MessageType;

use crate::error::Error;
use crate::try_serialize::TrySerialize;

/// Core Message
///
/// Contains a protocol version and an enumerated form of the message.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Message {
    protocol_version: u8,
    message_type: MessageType,
}

impl Message {
    /// Protocol version.
    pub const PROTOCOL_VERSION: u8 = 0x02;

    /// Constructs a new Message.
    ///
    /// `protocol_version` is defaulted.
    pub fn new(message_type: MessageType) -> Self {
        Self {
            protocol_version: Self::PROTOCOL_VERSION,
            message_type,
        }
    }

    /// Returns the protocol version.
    ///
    /// This should always be [`Message::PROTOCOL_VERSION`], but we add this redundancy for
    /// convention consistency.
    pub fn protocol_version(&self) -> u8 {
        self.protocol_version
    }

    /// Returns the enumerated message type.
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }

    /// Returns the byte length
    ///
    /// Always `25` unless [`MessageType::Pack`] is used.
    pub fn encoding_byte_length(&self) -> usize {
        match self.message_type {
            MessageType::Pack(pack) => pack.number_of_messages() as usize * 25,
            _ => 25,
        }
    }

    /// Returns true if [`MessageType::Pack`] is used.
    pub fn is_pack(&self) -> bool {
        match self.message_type {
            MessageType::Pack(_) => true,
            _ => false,
        }
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // we dont check the length here because it is checked in the `message_type` parsing, which
        // determines first if the internal message is a `pack`, implying a different value length.
        //
        // we could also check this here but we're separating concerns & reducing redunancy by not
        // doing this.
        //
        // length should be `25` if anything but a pack. if the message is a pack, the length should
        // be `2 + (msg_count * 25)`.
        let protocol_version = value[0] & 0b0000_1111;
        let message_type = value.as_ref().try_into()?;

        if protocol_version != Self::PROTOCOL_VERSION {
            return Err(Error::InvalidProtocolVersion);
        }

        Ok(Self {
            protocol_version,
            message_type,
        })
    }
}

impl TrySerialize for Message {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        if buffer.len() != 25 {
            return Err(Error::InvalidDataLength);
        }

        buffer[0] = self.protocol_version;

        self.message_type.try_serialize(buffer)
    }
}

macro_rules! impl_from_for_message {
    ($message_type:ident, $type_path:ty) => {
        impl From<$type_path> for Message {
            fn from(value: $type_path) -> Self {
                Self {
                    protocol_version: Message::PROTOCOL_VERSION,
                    message_type: MessageType::$message_type(value)
                }
            }
        }
    };
}

impl_from_for_message!(BasicID, crate::basic_id::BasicID);
impl_from_for_message!(Location, crate::location::Location);
impl_from_for_message!(Authentication, crate::authentication::Authentication);
impl_from_for_message!(SelfID, crate::self_id::SelfID);
impl_from_for_message!(System, crate::system::System);
impl_from_for_message!(OperatorID, crate::operator_id::OperatorID);
impl_from_for_message!(Pack, crate::pack::Pack);

#[cfg(test)]
mod tests {
    use crate::{basic_id::{BasicID, UASID, UAType, UTMAssignedUUID}, messages::{Message, MessageType}, operator_id::{OperatorID, OperatorIDType}, try_serialize::TrySerialize};

    #[test]
    fn test_getters() {
        let basic_id = BasicID::new(UAType::Aeroplane, UASID::None);
        let message_type = MessageType::BasicID(basic_id);
        let message = Message::new(message_type);

        assert_eq!(message.protocol_version(), Message::PROTOCOL_VERSION);
        assert_eq!(message.message_type(), &message_type);
        assert_eq!(message.encoding_byte_length(), 25);
        assert_eq!(message.is_pack(), false);
    }

    #[test]
    fn test_encode() {
        let ua_type = UAType::Aeroplane;
        let uas_id = UASID::UTMAssignedUUID(UTMAssignedUUID::new([2u8; 20]));
        let basic_id = BasicID::new(ua_type, uas_id);
        let message_type = MessageType::BasicID(basic_id);
        let message = Message::new(message_type);

        let mut encoded_uas_id = [0u8; 21];
        uas_id.try_serialize(&mut encoded_uas_id).unwrap();

        let mut encoded = [0u8; 25];
        message.try_serialize(&mut encoded).unwrap();

        assert_eq!(encoded[0], Message::PROTOCOL_VERSION | 0 << 4);
        assert_eq!(encoded[1] & 0b0000_1111, u8::from(ua_type));
        assert_eq!(encoded[1] >> 4, 3);
        assert_eq!(encoded[2..22], encoded_uas_id[1..]);
        assert_eq!(encoded[23], 0);
        assert_eq!(encoded[24], 0);
    }

    #[test]
    fn test_encode_operator_id() {
        let operator_id = OperatorID::new(OperatorIDType::OperatorID, [2u8; 20]);
        let message_type = MessageType::OperatorID(operator_id);
        let message = Message::new(message_type);

        let mut encoded_operator_id = [0u8; 24];
        operator_id.try_serialize(&mut encoded_operator_id).unwrap();

        let mut encoded = [0u8; 25];
        message.try_serialize(&mut encoded).unwrap();

        assert_eq!(encoded[0], Message::PROTOCOL_VERSION | 5 << 4);
        assert_eq!(encoded[1..25], encoded_operator_id);
    }

    #[test]
    fn test_encode_fails_invalid_data_length() {
        let mut too_short = [0u8; 24];
        let mut too_long = [0u8; 26];

        let ua_type = UAType::Aeroplane;
        let uas_id = UASID::UTMAssignedUUID(UTMAssignedUUID::new([2u8; 20]));
        let basic_id = BasicID::new(ua_type, uas_id);
        let message_type = MessageType::BasicID(basic_id);
        let message = Message::new(message_type);

        assert!(message.try_serialize(&mut too_short).is_err());
        assert!(message.try_serialize(&mut too_long).is_err());
    }

    #[test]
    fn test_decode() {
        let ua_type = UAType::Aeroplane;
        let uas_id = UASID::UTMAssignedUUID(UTMAssignedUUID::new([2u8; 20]));
        let basic_id = BasicID::new(ua_type, uas_id);
        let message_type = MessageType::BasicID(basic_id);
        let expected = Message::new(message_type);

        let mut encoded_uas_id = [0u8; 21];
        uas_id.try_serialize(&mut encoded_uas_id).unwrap();

        let mut encoded = [0u8; 25];
        encoded[0] = Message::PROTOCOL_VERSION;
        encoded[1..22].clone_from_slice(&encoded_uas_id);
        encoded[1] |= u8::from(ua_type);

        let decoded = Message::try_from(encoded.as_ref()).unwrap();

        assert_eq!(decoded, expected);
    }

    #[test]
    fn test_decode_fails_invalid_data_length() {
        let too_short = [0u8; 24];
        let too_long = [0u8; 26];

        assert!(Message::try_from(too_short.as_ref()).is_err());
        assert!(Message::try_from(too_long.as_ref()).is_err());
    }

    #[test]
    fn test_decode_fails_invalid_protocol_version() {
        let invalid = [0u8; 25];

        assert!(Message::try_from(invalid.as_ref()).is_err());
    }
}
