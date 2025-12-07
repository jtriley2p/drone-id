use crate::error::Error;

/// Type of Authentication
///
/// The authentication type specifies which type of authentication data is used in an
/// [`Authentication`](crate::authentication::Authentication) message.
///
///
/// Enumeration is used for authentication messages. Values of `6` to `9` are reserved, though
/// values from `0x0A` to `0x0F` are available for private use.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AuthenticationType {
    /// No authentication.
    None,
    /// UAS ID signature.
    UASIDSignature,
    /// Operator ID signature.
    OperatorIDSignature,
    /// Message Set signature.
    ///
    /// Used when all messages can be sent together.
    MessageSetSignature,
    /// Network Remote ID authentication data.
    ///
    /// This type should leave all other bytes in the authentication data as null.
    NetworkRemoteIDAuthentication,
    /// Uses the Internet Assigned Number Authority (IANA) "Specification Required" system specified
    /// by the Internet Engineering Task Force's (IETF) RFC-8126.
    SpecificAuthenticationMessage,
    /// Reserved.
    ReservedForSpec,
    /// Available for private use, particularly for local custom authentication types.
    AvailableForPrivateUse,
}

impl AuthenticationType {
    /// Special value representing the smallest "reserved" value.
    pub const RESERVED_THRESHOLD: u8 = 6;

    /// Special value representing the smallest "private use" value.
    pub const PRIVATE_USE_THRESHOLD: u8 = 0x0a;

    /// Maximum valid authentication value.
    pub const MAX: u8 = 0x10;
}

impl TryFrom<u8> for AuthenticationType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > Self::MAX {
            return Err(Error::InvalidInteger);
        }

        if value >= Self::PRIVATE_USE_THRESHOLD {
            return Ok(Self::AvailableForPrivateUse);
        }

        if value >= Self::RESERVED_THRESHOLD {
            return Ok(Self::ReservedForSpec);
        }

        // todo: de-nest
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::UASIDSignature),
            2 => Ok(Self::OperatorIDSignature),
            3 => Ok(Self::MessageSetSignature),
            4 => Ok(Self::NetworkRemoteIDAuthentication),
            5 => Ok(Self::SpecificAuthenticationMessage),
            _ => Err(Error::Unreachable),
        }
    }
}

impl From<AuthenticationType> for u8 {
    fn from(value: AuthenticationType) -> Self {
        match value {
            AuthenticationType::None => 0,
            AuthenticationType::UASIDSignature => 1,
            AuthenticationType::OperatorIDSignature => 2,
            AuthenticationType::MessageSetSignature => 3,
            AuthenticationType::NetworkRemoteIDAuthentication => 4,
            AuthenticationType::SpecificAuthenticationMessage => 5,
            AuthenticationType::ReservedForSpec => AuthenticationType::RESERVED_THRESHOLD,
            AuthenticationType::AvailableForPrivateUse => AuthenticationType::PRIVATE_USE_THRESHOLD,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::authentication::AuthenticationType;

    #[test]
    fn test_encode_decode_valid() {
        let uas_id_signature = AuthenticationType::UASIDSignature;

        let encoded = u8::from(uas_id_signature);

        let decoded = AuthenticationType::try_from(encoded).unwrap();

        assert_eq!(encoded, 1);
        assert_eq!(uas_id_signature, decoded);
    }

    #[test]
    fn test_encode_decode_reserved() {
        let reserved = AuthenticationType::ReservedForSpec;

        let encoded = u8::from(reserved);

        let decoded = AuthenticationType::try_from(encoded).unwrap();

        assert_eq!(encoded, AuthenticationType::RESERVED_THRESHOLD);
        assert_eq!(reserved, decoded);
    }

    #[test]
    fn test_encode_decode_private_use() {
        let private_use = AuthenticationType::AvailableForPrivateUse;

        let encoded = u8::from(private_use);

        let decoded = AuthenticationType::try_from(encoded).unwrap();

        assert_eq!(encoded, AuthenticationType::PRIVATE_USE_THRESHOLD);
        assert_eq!(private_use, decoded);
    }

    #[test]
    fn test_decode_fails_value_too_large() {
        let invalid_value = AuthenticationType::MAX + 1;

        let failed = AuthenticationType::try_from(invalid_value).is_err();

        assert!(failed);
    }
}
