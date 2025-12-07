use crate::error::Error;
use crate::try_serialize::TrySerialize;

/// Registration ID for a given Civil Aviation Authority
///
/// Contains both a nationality mark issued by the International Civil Aviation Organization and a
/// unique identifier issued by the respective Civil Aviation Authority.
///
/// The string given must be ASCII upper case, digits, or a dot character ".".
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RegistrationID([u8; 20]);

impl RegistrationID {
    /// Tries to construct a new Registration ID
    ///
    /// Returns error if:
    ///
    /// - the total length of `nationality_mark` and `caa_id` is greater than 19.
    /// - `nationality_mark` is not ascii + decimal digits (or null).
    /// - `caa_id` is not ascii + decimal digits (or null).
    pub fn try_new(nationality_mark: &str, caa_id: &str) -> Result<Self, Error> {
        if nationality_mark.len() + caa_id.len() > 19 {
            return Err(Error::InvalidDataLength);
        }

        let valid_nationality_mark = nationality_mark
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_digit(10) || c == '\x00');

        let valid_caa_id = caa_id
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_digit(10) || c == '\x00');

        if !valid_nationality_mark || !valid_caa_id {
            return Err(Error::InvalidRegistrationID);
        }

        let dot_index = nationality_mark.len();
        let caa_start = dot_index + 1;
        let caa_stop = caa_start + caa_id.len();

        let mut id = [0u8; 20];
        id[..dot_index].clone_from_slice(&nationality_mark.as_bytes());
        id[dot_index] = b'.';
        id[caa_start..caa_stop].clone_from_slice(&caa_id.as_bytes());

        Ok(Self(id))
    }

    /// Returns the nationality mark.
    pub fn nationality_mark(&self) -> &str {
        let mark = self
            .0
            .split(|&c| c == b'.')
            .next()
            .ok_or(Error::Unreachable)
            .unwrap();

        str::from_utf8(mark)
            .map_err(|_| Error::Unreachable)
            .unwrap()
    }

    /// Returns the Civil Aviation Authority's registration ID.
    pub fn caa_id(&self) -> &str {
        // bit messy since we have no dynamic allocations, but here's the breakdown:
        //
        // - split string at the dot
        //     - "USA.1234\x00\x00\x00.." -> ("USA", "1234\x00\x00\x00..")
        // - take the second item in the split
        //     - ("USA", "1234\x00\x00\x00..") -> "1234\x00\x00\x00.."
        // - split the second item at the first null byte
        //     - "1234\x00\x00\x00.." -> ("1234", "\x00\x00..")
        // - take the first item of the second split
        //     - ("1234", "\x00\x00..") -> "1234"
        let caa_id = self
            .0
            .split(|&c| c == b'.')
            .nth(1)
            .ok_or(Error::Unreachable)
            .unwrap()
            .split(|&byte| byte == 0)
            .next()
            .ok_or(Error::Unreachable)
            .unwrap();

        str::from_utf8(caa_id)
            .map_err(|_| Error::Unreachable)
            .unwrap()
    }
}

impl TryFrom<&[u8]> for RegistrationID {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 20 {
            return Err(Error::InvalidDataLength);
        }

        let value: [u8; 20] = value.try_into().map_err(|_| Error::Unreachable).unwrap();

        let mut dot_character_count = 0;

        for byte in value {
            let c = byte as char;

            if c == '.' {
                dot_character_count += 1;
                continue;
            }

            if !(c.is_ascii_uppercase() || c.is_digit(10) || c == '\x00') {
                return Err(Error::InvalidRegistrationID);
            }
        }

        if dot_character_count != 1 {
            return Err(Error::InvalidRegistrationID);
        }

        Ok(Self(value))
    }
}

impl TrySerialize for RegistrationID {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        if buffer.len() != 20 {
            return Err(Error::InvalidDataLength);
        }

        buffer.clone_from_slice(&self.0);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::basic_id::RegistrationID;
    use crate::try_serialize::TrySerialize;

    fn str_to_fixed_bytes(s: &str) -> [u8; 20] {
        assert!(s.len() < 20);

        let mut fixed_bytes = [0u8; 20];

        fixed_bytes[..s.len()].clone_from_slice(s.as_bytes());

        fixed_bytes
    }

    #[test]
    fn test_try_new() {
        let nationality_mark = "US";
        let caa_id = "1234";

        let registration_id = RegistrationID::try_new(nationality_mark, caa_id).unwrap();

        assert_eq!(registration_id.nationality_mark(), nationality_mark);
        assert_eq!(registration_id.caa_id(), caa_id);
    }

    #[test]
    fn test_try_new_invalid_length() {
        let nationality_mark = "US";
        let caa_id = "123456789012345678";

        assert!(RegistrationID::try_new(nationality_mark, caa_id).is_err());
    }

    #[test]
    fn test_try_new_invalid_char() {
        assert!(RegistrationID::try_new("A", "λ").is_err());
        assert!(RegistrationID::try_new("λ", "A").is_err());
    }

    #[test]
    fn test_encode() {
        let str_id = "US.1234";
        let id = str_to_fixed_bytes(str_id);

        let decoded = RegistrationID::try_from(id.as_ref()).unwrap();

        let mut encoded = [0u8; 20];
        decoded.try_serialize(&mut encoded).unwrap();

        assert_eq!(id, encoded);
        assert_eq!(encoded[0], b'U');
        assert_eq!(encoded[1], b'S');
        assert_eq!(encoded[2], b'.');
        assert_eq!(encoded[3], b'1');
        assert_eq!(encoded[4], b'2');
        assert_eq!(encoded[5], b'3');
        assert_eq!(encoded[6], b'4');
        assert_eq!(encoded[7], 0);
    }

    #[test]
    fn test_decode() {
        let id = str_to_fixed_bytes("US.1234");

        let decoded = RegistrationID::try_from(id.as_ref()).unwrap();

        assert_eq!(decoded.nationality_mark(), "US");
        assert_eq!(decoded.caa_id(), "1234");
    }

    #[test]
    fn test_encode_fails_invalid_length() {
        let too_short = [0u8; 19];
        let too_long = [0u8; 21];

        assert!(RegistrationID::try_from(too_short.as_ref()).is_err());
        assert!(RegistrationID::try_from(too_long.as_ref()).is_err());
    }

    #[test]
    fn test_decode_fails_invalid_length() {
        let id = str_to_fixed_bytes("US.1234");
        let decoded = RegistrationID::try_from(id.as_ref()).unwrap();

        let mut too_short = [0u8; 19];
        let mut too_long = [0u8; 21];

        assert!(decoded.try_serialize(&mut too_short).is_err());
        assert!(decoded.try_serialize(&mut too_long).is_err());
    }

    #[test]
    fn test_encode_fails_invalid_char() {
        let str_id = "US.1234λ";
        let id = str_to_fixed_bytes(str_id);

        assert!(RegistrationID::try_from(id.as_ref()).is_err());
    }

    #[test]
    fn test_encode_fails_too_many_dots() {
        let str_id = "US.1234.";
        let id = str_to_fixed_bytes(str_id);

        assert!(RegistrationID::try_from(id.as_ref()).is_err());
    }

    #[test]
    fn test_encode_fails_too_few_dots() {
        let str_id = "US1234";
        let id = str_to_fixed_bytes(str_id);

        assert!(RegistrationID::try_from(id.as_ref()).is_err());
    }
}
