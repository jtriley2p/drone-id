use crate::authentication::authentication_type::AuthenticationType;
use crate::error::Error;
use crate::try_serialize::TrySerialize;

/// Subsequent Authentication Message(s)
///
/// The subsequent authentication message(s) contain an authentication type, page number, and
/// respective authentication data.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Subsequent {
    authentication_type: AuthenticationType,
    page_number: usize,
    data: [u8; 23],
}

impl Subsequent {
    /// Constructs a new Subsequent message
    ///
    /// Returns an error if:
    ///
    /// - `page_number` is greater than 15.
    pub fn try_new(
        authentication_type: AuthenticationType,
        page_number: usize,
        data: [u8; 23],
    ) -> Result<Self, Error> {
        if page_number > 15 {
            return Err(Error::InvalidInteger);
        }

        Ok(Self {
            authentication_type,
            page_number,
            data,
        })
    }

    /// Returns the authentication type.
    pub fn authentication_type(&self) -> AuthenticationType {
        self.authentication_type
    }

    /// Returns the page number.
    ///
    /// Values range from `1` to `15`.
    pub fn page_number(&self) -> usize {
        self.page_number
    }

    /// Returns the unstructured authentication data.
    pub fn data(&self) -> &[u8; 23] {
        &self.data
    }
}

impl TryFrom<&[u8]> for Subsequent {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        let page_number = (value[0] & 0b0000_1111) as usize;

        let authentication_type = (value[0] >> 4).try_into()?;

        let data = value[1..]
            .try_into()
            .map_err(|_| Error::Unreachable)
            .unwrap();

        Ok(Self {
            authentication_type,
            page_number,
            data,
        })
    }
}

impl TrySerialize for Subsequent {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        if buffer.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        buffer[0] = u8::from(self.authentication_type) << 4 | self.page_number as u8;
        buffer[1..].clone_from_slice(&self.data);

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::authentication::AuthenticationType;
    use crate::authentication::Subsequent;
    use crate::try_serialize::TrySerialize;

    #[test]
    fn test_try_new() {
        let authentication_type = AuthenticationType::UASIDSignature;
        let page_number = 1;
        let data = [2u8; 23];

        let subsequent = Subsequent::try_new(authentication_type, page_number, data).unwrap();

        assert_eq!(subsequent.authentication_type(), authentication_type);
        assert_eq!(subsequent.page_number(), page_number);
        assert_eq!(subsequent.data(), &data);
    }

    #[test]
    fn test_try_new_fails_invalid_page_number() {
        let authentication_type = AuthenticationType::UASIDSignature;
        let too_bigpage_number = 16;
        let data = [2u8; 23];

        assert!(Subsequent::try_new(authentication_type, too_bigpage_number, data).is_err());
    }

    #[test]
    fn test_encode() {
        let authentication_type = AuthenticationType::UASIDSignature;
        let page_number = 1u8;
        let data = [2u8; 23];

        let subsequent =
            Subsequent::try_new(authentication_type, page_number as usize, data).unwrap();

        let mut encoded = [0u8; 24];
        subsequent.try_serialize(&mut encoded).unwrap();

        assert_eq!(
            encoded[0] >> 4,
            u8::from(AuthenticationType::UASIDSignature)
        );
        assert_eq!(encoded[0] & 0b0000_1111, page_number);
        assert_eq!(encoded[1..], data);
    }

    #[test]
    fn test_decode() {
        let authentication_type = AuthenticationType::UASIDSignature;
        let page_number = 1u8;
        let data = [2u8; 23];

        let subsequent =
            Subsequent::try_new(authentication_type, page_number as usize, data).unwrap();

        let mut encoded = [0u8; 24];
        subsequent.try_serialize(&mut encoded).unwrap();

        let decoded = Subsequent::try_from(encoded.as_ref()).unwrap();

        assert_eq!(subsequent, decoded);
    }

    #[test]
    fn test_encode_fails_value_length() {
        let too_short = [0u8; 23];
        let too_long = [0u8; 25];

        assert!(Subsequent::try_from(too_short.as_ref()).is_err());
        assert!(Subsequent::try_from(too_long.as_ref()).is_err());
    }

    #[test]
    fn test_decode_fails_value_length() {
        let mut too_short = [0u8; 23];
        let mut too_long = [0u8; 25];

        let subsequent =
            Subsequent::try_new(AuthenticationType::UASIDSignature, 0, [2u8; 23]).unwrap();

        assert!(subsequent.try_serialize(&mut too_short).is_err());
        assert!(subsequent.try_serialize(&mut too_long).is_err());
    }
}
