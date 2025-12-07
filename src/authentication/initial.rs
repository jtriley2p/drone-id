use crate::authentication::authentication_type::AuthenticationType;
use crate::error::Error;
use crate::system::Timestamp;
use crate::try_serialize::TrySerialize;

/// Initial Authentication Message
///
/// The initial authentication message contains an authentication type, the index of the last page,
/// a total length of the underlying authentication data, a 32-bit unix timestamp, and 17 bytes of
/// authentication data.
///
/// The maximum [`Initial::last_page_index`] value is 15 and the maximum [`Initial::total_length`]
/// value is `255`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Initial {
    authentication_type: AuthenticationType,
    // max: 15
    last_page_index: usize,
    // max 255
    total_length: usize,
    timestamp: Timestamp,
    data: [u8; 17],
}

impl Initial {
    /// Constructs a new Initial message.
    ///
    /// Returns an error if:
    ///
    /// - `last_page_index` is greater than 15.
    /// - `total_length` is greater than 255.
    pub fn try_new(
        authentication_type: AuthenticationType,
        last_page_index: usize,
        total_length: usize,
        timestamp: Timestamp,
        data: [u8; 17],
    ) -> Result<Self, Error> {
        if last_page_index > 15 || total_length > 255 {
            return Err(Error::InvalidInteger);
        }

        Ok(Self {
            authentication_type,
            last_page_index,
            total_length,
            timestamp,
            data,
        })
    }

    /// Returns the authentication type.
    pub fn authentication_type(&self) -> AuthenticationType {
        self.authentication_type
    }

    /// Returns the page number (always zero for [`Initial`]).
    pub fn page_number(&self) -> usize {
        0
    }

    /// Returns the last page index.
    ///
    /// Maximum is `15`.
    pub fn last_page_index(&self) -> usize {
        self.last_page_index
    }

    /// Returns the total byte length of the authentication data.
    ///
    /// Maximum is `255`.
    pub fn total_length(&self) -> usize {
        self.total_length
    }

    /// Returns the system [`Timestamp`](crate::system::Timestamp).
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Returns the unstructured authentication data.
    pub fn data(&self) -> &[u8; 17] {
        &self.data
    }
}

impl TryFrom<&[u8]> for Initial {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        Ok(Self {
            authentication_type: (value[0] >> 4).try_into()?,
            last_page_index: (value[1] & 0b0000_1111) as usize,
            total_length: value[2] as usize,
            timestamp: u32::from_le_bytes([value[3], value[4], value[5], value[6]]).into(),
            data: value[7..]
                .try_into()
                .map_err(|_| Error::Unreachable)
                .unwrap(),
        })
    }
}

impl TrySerialize for Initial {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        if buffer.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        buffer[0] = u8::from(self.authentication_type) << 4;
        buffer[1] = self.last_page_index as u8;
        buffer[2] = self.total_length as u8;
        buffer[3..7].clone_from_slice(&u32::from(self.timestamp).to_le_bytes());
        buffer[7..].clone_from_slice(&self.data);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::authentication::AuthenticationType;
    use crate::authentication::Initial;
    use crate::system::Timestamp;
    use crate::try_serialize::TrySerialize;

    #[test]
    fn test_try_new() {
        let authentication_type = AuthenticationType::UASIDSignature;
        let last_page_index = 0;
        let total_length = 25;
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

        assert_eq!(initial.authentication_type(), authentication_type);
        assert_eq!(initial.last_page_index(), last_page_index);
        assert_eq!(initial.total_length(), total_length);
        assert_eq!(initial.timestamp(), timestamp);
        assert_eq!(initial.data(), &data);
    }

    #[test]
    fn test_try_new_fails_invalid_index() {
        let too_big_last_page_index = 16;

        assert!(
            Initial::try_new(
                AuthenticationType::UASIDSignature,
                too_big_last_page_index,
                25,
                Timestamp::new(1),
                [2u8; 17],
            )
            .is_err()
        );
    }

    #[test]
    fn test_try_new_fails_invalid_total_length() {
        let too_big_total_length = 256;

        assert!(
            Initial::try_new(
                AuthenticationType::UASIDSignature,
                1,
                too_big_total_length,
                Timestamp::new(1),
                [2u8; 17],
            )
            .is_err()
        );
    }

    #[test]
    fn test_encode() {
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

        let mut encoded = [0u8; 24];
        initial.try_serialize(&mut encoded).unwrap();

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

        let mut encoded = [0u8; 24];
        initial.try_serialize(&mut encoded).unwrap();

        let decoded = Initial::try_from(encoded.as_ref()).unwrap();

        assert_eq!(initial, decoded);
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

        assert!(initial.try_serialize(&mut too_short).is_err());
        assert!(initial.try_serialize(&mut too_long).is_err());
    }
}
