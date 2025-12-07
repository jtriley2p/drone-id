//! ## Pack Message
//!
//! The [`Pack`] message internally contains a batch of one to nine internal messages. This is
//! generally used with transmission/broadcast messages with larger data frames capable of
//! containing the full message pack.
//!
//! Since [`Message`] fields inflate to larger values and incur the additional memory cost of Rust's
//! memory layout, we keep the messages serialized until retrieval via [`Pack::try_get_message`].
//! The upper bound of bytes required for the internal messages in the pack is `225` bytes
//! (`9 * 25`).
//!
//! This may be constructed from deserializing bytes directly or from a reference to a message
//! array.
use crate::error::Error;
use crate::messages::Message;
use crate::try_serialize::TrySerialize;

/// Pack Message
///
/// Contains a dynamic number of messages (up to nine) and an indicator of how many messages there
/// are.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pack {
    number_of_messages: u8,
    messages: [u8; 225],
}

impl Pack {
    /// Message length is always the same for non-pack messages.
    pub const MESSAGES_LENGTH: usize = 25;

    /// Message code for pack messages is `0x0F`.
    pub const PACK_MESSAGE_CODE: u8 = 0x0f;

    /// Tries to get a message.
    ///
    /// Returns [`Option::None`] if the index exceeds the number of messages.
    ///
    /// Returns [`Result::Err`] if the message fails to be deserialized.
    pub fn try_get_message(&self, index: u8) -> Option<Result<Message, Error>> {
        if index >= self.number_of_messages {
            return None;
        }

        let offset = (index * 25) as usize;

        let raw_message = &self.messages[offset..offset + 25];

        if raw_message[0] == Self::PACK_MESSAGE_CODE {
            // no recursive packing
            return Some(Err(Error::CannotRecursivelyPack));
        }

        match raw_message.try_into() {
            Ok(message) => Some(Ok(message)),
            Err(e) => Some(Err(e)),
        }
    }

    /// Returns the number of messages in the pack.
    pub fn number_of_messages(&self) -> u8 {
        self.number_of_messages
    }

    /// Returns the raw message data.
    ///
    /// For finding a specific message, use [`Pack::try_get_message`].
    pub fn messages(&self) -> &[u8] {
        &self.messages
    }
}

impl TryFrom<&[u8]> for Pack {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let number_of_messages = *value.get(1).ok_or(Error::InvalidDataLength)?;

        if number_of_messages > 9 {
            return Err(Error::InvalidInteger);
        }

        if value.len() != 2 + number_of_messages as usize * Self::MESSAGES_LENGTH {
            return Err(Error::InvalidDataLength);
        }

        if value[0] != Self::MESSAGES_LENGTH as u8 {
            return Err(Error::InvalidInteger);
        }

        let mut messages = [0u8; 225];

        for i in 2..value.len() {
            messages[i - 2] = value[i];
        }

        Ok(Self {
            number_of_messages,
            messages,
        })
    }
}

impl TrySerialize for Pack {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let messages_length = self.number_of_messages as usize * Self::MESSAGES_LENGTH;

        if buffer.len() != 2 + messages_length {
            return Err(Error::InvalidDataLength);
        }

        buffer[0] = Self::MESSAGES_LENGTH as u8;
        buffer[1] = self.number_of_messages;
        buffer[2..].clone_from_slice(&self.messages[..messages_length]);

        Ok(())
    }
}

impl<const N: usize> TryFrom<[Message; N]> for Pack {
    type Error = Error;

    fn try_from(value: [Message; N]) -> Result<Self, Self::Error> {
        if N > 9 {
            return Err(Error::InvalidInteger);
        }

        let mut buffer = [0u8; 225];

        for i in 0..N {
            let start = i * Self::MESSAGES_LENGTH;

            let end = start + Self::MESSAGES_LENGTH;

            value[i].try_serialize(&mut buffer[start..end])?;
        }

        Ok(Self {
            number_of_messages: N as u8,
            messages: buffer,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        basic_id::{BasicID, UASID, UAType, UTMAssignedUUID},
        messages::{Message, MessageType},
        operator_id::{OperatorID, OperatorIDType},
        pack::Pack,
        try_serialize::TrySerialize,
    };

    const fn total_len(message_count: usize) -> usize {
        2 + message_count * Pack::MESSAGES_LENGTH
    }

    #[test]
    fn test_getters() {
        let ua_type = UAType::Aeroplane;
        let uas_id = UASID::UTMAssignedUUID(UTMAssignedUUID::new([2u8; 20]));
        let basic_id = BasicID::new(ua_type, uas_id);
        let message_type = MessageType::BasicID(basic_id);
        let message = Message::new(message_type);

        let pack = Pack::try_from([message]).unwrap();

        let mut encoded_messages = [0u8; 225];
        message.try_serialize(&mut encoded_messages[..25]).unwrap();

        assert_eq!(pack.try_get_message(0).unwrap().unwrap(), message);
        assert_eq!(pack.number_of_messages(), 1);
        assert_eq!(pack.messages(), encoded_messages);
    }

    #[test]
    fn test_encode() {
        let operator_id = OperatorID::new(OperatorIDType::OperatorID, [2u8; 20]);

        let message = Message::from(operator_id);

        let mut encoded_message = [0u8; 25];
        message.try_serialize(&mut encoded_message).unwrap();

        let pack = Pack::try_from([message]).unwrap();

        let mut encoded = [0u8; total_len(1)];
        pack.try_serialize(&mut encoded).unwrap();

        assert_eq!(encoded[0], Pack::MESSAGES_LENGTH as u8);
        assert_eq!(encoded[1], 1);
        assert_eq!(encoded[2..], encoded_message);
    }

    #[test]
    fn test_encode_two() {
        let operator_id = Message::from(OperatorID::new(OperatorIDType::OperatorID, [2u8; 20]));
        let basic_id = Message::from(BasicID::new(
            UAType::Aeroplane,
            UASID::UTMAssignedUUID(UTMAssignedUUID::new([2u8; 20])),
        ));

        let mut encoded_operator_id_message = [0u8; 25];
        operator_id
            .try_serialize(&mut encoded_operator_id_message)
            .unwrap();

        let mut encoded_basic_id_message = [0u8; 25];
        basic_id
            .try_serialize(&mut encoded_basic_id_message)
            .unwrap();

        let pack = Pack::try_from([operator_id, basic_id]).unwrap();

        let mut encoded = [0u8; total_len(2)];
        pack.try_serialize(&mut encoded).unwrap();

        assert_eq!(encoded[0], Pack::MESSAGES_LENGTH as u8);
        assert_eq!(encoded[1], 2);
        assert_eq!(encoded[2..27], encoded_operator_id_message);
        assert_eq!(encoded[27..52], encoded_basic_id_message);
    }

    #[test]
    fn test_encode_fails_invalid_length() {
        let mut too_short = [0u8; 26];
        let mut too_long = [0u8; 28];

        let operator_id = Message::from(OperatorID::new(OperatorIDType::OperatorID, [2u8; 20]));
        let pack = Pack::try_from([operator_id]).unwrap();

        assert!(pack.try_serialize(&mut too_short).is_err());
        assert!(pack.try_serialize(&mut too_long).is_err());
    }

    #[test]
    fn test_decode() {
        let operator_id = Message::from(OperatorID::new(OperatorIDType::OperatorID, [2u8; 20]));

        let mut encoded_operator_id_message = [0u8; 25];
        operator_id
            .try_serialize(&mut encoded_operator_id_message)
            .unwrap();

        let mut encoded = [0u8; total_len(1)];
        encoded[0] = Pack::MESSAGES_LENGTH as u8;
        encoded[1] = 1;
        encoded[2..27].clone_from_slice(&encoded_operator_id_message);

        let pack = Pack::try_from([operator_id]).unwrap();

        assert_eq!(Pack::try_from(encoded.as_ref()).unwrap(), pack);
    }

    #[test]
    fn test_decode_two() {
        let operator_id = Message::from(OperatorID::new(OperatorIDType::OperatorID, [2u8; 20]));
        let basic_id = Message::from(BasicID::new(
            UAType::Aeroplane,
            UASID::UTMAssignedUUID(UTMAssignedUUID::new([2u8; 20])),
        ));

        let mut encoded_operator_id_message = [0u8; 25];
        operator_id
            .try_serialize(&mut encoded_operator_id_message)
            .unwrap();

        let mut encoded_basic_id_message = [0u8; 25];
        basic_id
            .try_serialize(&mut encoded_basic_id_message)
            .unwrap();

        let mut encoded = [0u8; total_len(2)];
        encoded[0] = Pack::MESSAGES_LENGTH as u8;
        encoded[1] = 2;
        encoded[2..27].clone_from_slice(&encoded_operator_id_message);
        encoded[27..52].clone_from_slice(&encoded_basic_id_message);

        let pack = Pack::try_from([operator_id, basic_id]).unwrap();

        assert_eq!(Pack::try_from(encoded.as_ref()).unwrap(), pack);
    }

    #[test]
    fn test_decode_fails_invalid_length() {
        let too_short = [0u8; 26];
        let too_long = [0u8; 28];

        assert!(Pack::try_from(too_short.as_ref()).is_err());
        assert!(Pack::try_from(too_long.as_ref()).is_err());
    }

    #[test]
    fn test_try_from_messages_fails_invalid_length() {
        let operator_id = Message::from(OperatorID::new(OperatorIDType::OperatorID, [2u8; 20]));

        let mut encoded_operator_id_message = [0u8; 25];
        operator_id
            .try_serialize(&mut encoded_operator_id_message)
            .unwrap();

        let result = Pack::try_from([
            operator_id,
            operator_id,
            operator_id,
            operator_id,
            operator_id,
            operator_id,
            operator_id,
            operator_id,
            operator_id,
            operator_id,
        ]);

        assert!(result.is_err());
    }
}
