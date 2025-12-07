//! ## Authentication Message
//!
//! [`Authentication`] messages facilitate authenticated communication between the UAS and an
//! application which receives the messages. Using no authentication is permitted in the scope of
//! the specification, but the authentication message must still be transmitted, just with
//! [`AuthenticationType::None`].
//!
//! [`Authentication`] data may be paginated with up to 16 total messages. These may be contained in
//! [`Pack`](crate::pack::Pack) messages or simply consecutive messages. The format of the
//! authentication depends on if it is the [`Initial`] message or one of the [`Subsequent`]
//! messages.
//!
//! Each authentication message comes with a page number, which determines the format.
mod authentication_type;
mod initial;
mod subsequent;

pub use authentication_type::AuthenticationType;
pub use initial::Initial;
pub use subsequent::Subsequent;

use crate::error::Error;
use crate::try_serialize::TrySerialize;

/// Authentication Message
///
/// Authentication data itself is large and unstructured (relative to the specification). Since
/// signatures and other large values may extend beyond the byte limit on any given message, this
/// specification facilitates up to 16 total pages of authentication data.
///
/// The initial authentication message contains 17 bytes and the up-to 15 subsequent authentication
/// messages contains 23 bytes, allowing theoretically for `362` (`17 + 15 * 23`) total bytes of
/// authentication data. In practice, the maximum total length specified in the initial message is
/// limited to 255 (due to a bit-size constraint).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Authentication {
    /// The initial authentication message.
    Initial(Initial),
    /// Any of the subsequent messages following the initial.
    Subsequent(Subsequent),
}

impl TryFrom<&[u8]> for Authentication {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        let page_number = value[0] & 0b0000_1111;

        match page_number {
            0 => Ok(Self::Initial(value.try_into()?)),
            _ => Ok(Self::Subsequent(value.try_into()?)),
        }
    }
}

impl TrySerialize for Authentication {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        if buffer.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        match self {
            Self::Initial(initial) => initial.try_serialize(buffer),
            Self::Subsequent(subsequent) => subsequent.try_serialize(buffer),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::authentication::Authentication;
    use crate::authentication::AuthenticationType;
    use crate::authentication::Initial;
    use crate::system::Timestamp;
    use crate::try_serialize::TrySerialize;

    #[test]
    fn test_encode_decode() {
        let authentication_type = AuthenticationType::UASIDSignature;
        let last_page_index = 0u8;
        let total_length = 25u8;
        let timestamp = Timestamp::new(1);
        let data = [2u8; 17];

        let initial = Initial::try_new(
            authentication_type,
            last_page_index as usize,
            total_length as usize,
            timestamp,
            data,
        )
        .unwrap();

        let authentication = Authentication::Initial(initial);

        let mut encoded = [0u8; 24];
        authentication.try_serialize(&mut encoded).unwrap();

        let decoded = Authentication::try_from(encoded.as_ref()).unwrap();

        assert_eq!(authentication, decoded);

        assert_eq!(
            encoded[0] >> 4,
            u8::from(AuthenticationType::UASIDSignature)
        );
        assert_eq!(encoded[1] & 0b0000_1111, last_page_index);
        assert_eq!(encoded[2], total_length);
        assert_eq!(encoded[3..7], u32::from(timestamp).to_le_bytes());
        assert_eq!(encoded[7..], data);
    }

    #[test]
    fn test_decode() {
        let authentication_type = AuthenticationType::UASIDSignature;
        let last_page_index = 0u8;
        let total_length = 25u8;
        let timestamp = Timestamp::new(1);
        let data = [2u8; 17];

        let initial = Initial::try_new(
            authentication_type,
            last_page_index as usize,
            total_length as usize,
            timestamp,
            data,
        )
        .unwrap();

        let authentication = Authentication::Initial(initial);

        let mut encoded = [0u8; 24];
        authentication.try_serialize(&mut encoded).unwrap();

        let decoded = Authentication::try_from(encoded.as_ref()).unwrap();

        assert_eq!(authentication, decoded);
    }

    #[test]
    fn test_encode_fails_value_length() {
        let too_short = [0u8; 23];
        let too_long = [0u8; 25];

        assert!(Initial::try_from(too_short.as_ref()).is_err());
        assert!(Initial::try_from(too_long.as_ref()).is_err());
    }

    #[test]
    fn test_decode_fails_value_length() {
        let mut too_short = [0u8; 23];
        let mut too_long = [0u8; 25];

        let initial = Initial::try_new(
            AuthenticationType::UASIDSignature,
            0,
            25,
            Timestamp::new(1),
            [2u8; 17],
        )
        .unwrap();

        let authentication = Authentication::Initial(initial);

        assert!(authentication.try_serialize(&mut too_short).is_err());
        assert!(authentication.try_serialize(&mut too_long).is_err());
    }
}
