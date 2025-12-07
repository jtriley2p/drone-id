use crate::error::Error;
use crate::try_serialize::TrySerialize;

/// Serial Number compliant with ANSI/CTA-2063-A.
///
/// Contains a manufacturer's code and a manufacturer's serial number.
///
/// Per the specification, the manufacturer's code MUST be exactly 4 characters of uppercase
/// ASCII (except "O" or "I") or digits. The manufacturer's serial MUST be 1-15 characters of
/// uppercase ASCII (except "O" or "I") or digits.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SerialNumber([u8; 20]);

impl SerialNumber {
    /// Tries to cosntruct a new serial number.
    /// 
    /// Returns an error if:
    /// 
    /// - `mfr_code` length is not 4.
    /// - `mfr_serial` length is not between 1 and 15 (inclusive).
    /// - `mfr_code` is not ASCII, decimal digits, or nulls.
    /// - `mfr_serial` is not ASCII, decimal digits, or nulls.
    pub fn try_new(mfr_code: &str, mfr_serial: &str) -> Result<Self, Error> {
        if mfr_code.len() != 4 || mfr_serial.len() < 1 || mfr_serial.len() > 15 {
            return Err(Error::InvalidDataLength);
        }

        let mfr_code_valid = mfr_code.chars()
            .all(Self::is_valid_character);
        let mfr_serial_valid = mfr_serial.chars()
            .all(Self::is_valid_character);

        if !mfr_code_valid || !mfr_serial_valid {
            return Err(Error::InvalidSerialNumber);
        }

        // we send the length values to uppercase ASCII hex values as follows:
        //
        // - 1-9 to ASCII decimal values 49-57 ("1" to "9")
        // - 10-15 to ASCII decimal values 65-70 ("A" to "F")
        let hex_length_ascii = match mfr_serial.len() <= 10 {
            true => mfr_serial.len() + 48,
            false => mfr_serial.len() + 55,
        } as u8;

        let mut serial_number = [0u8; 20];
        serial_number[..4].clone_from_slice(&mfr_code.as_bytes());
        serial_number[4] = hex_length_ascii;
        serial_number[5..5 + mfr_serial.len()].clone_from_slice(&mfr_serial.as_bytes());

        Ok(Self(serial_number))
    }

    /// Returns the manufacturer's code.
    ///
    /// Code is issued by International Civil Aviation Organization to UAS manufacturers globally.
    pub fn mfr_code(&self) -> &str {
        // INVARIANT: bytes are valid utf8 chars
        &str::from_utf8(&self.0[..4]).unwrap()
    }

    /// Returns the manufacturer's serial number.
    pub fn mfr_serial(&self) -> &str {
        let mfr_serial = self.0[5..]
            .split(|&byte| byte == 0)
            .next()
            .ok_or(Error::Unreachable)
            .unwrap();

        &str::from_utf8(mfr_serial).map_err(|_| Error::Unreachable).unwrap()
    }

    fn is_valid_character(c: char) -> bool {
        let is_ascii_digit_or_null = c.is_ascii_uppercase() || c.is_digit(10) || c == '\x00';

        is_ascii_digit_or_null && c != 'O' && c != 'I'
    }
}

impl TryFrom<&[u8]> for SerialNumber {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 20 {
            return Err(Error::InvalidDataLength);
        }

        let value: [u8; 20] = value.try_into().map_err(|_| Error::Unreachable).unwrap();

        if value
            .iter()
            .any(|&byte| !Self::is_valid_character(byte as char))
        {
            return Err(Error::InvalidSerialNumber);
        }

        let serial_length = value[4] as char;

        if !serial_length.is_ascii_hexdigit() {
            return Err(Error::InvalidSerialNumber);
        }

        Ok(Self(value))
    }
}

impl TrySerialize for SerialNumber {
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
    use crate::{basic_id::SerialNumber, try_serialize::TrySerialize};

    fn str_to_fixed_bytes(s: &str) -> [u8; 20] {
        assert!(s.len() < 20);

        let mut fixed_bytes = [0u8; 20];

        fixed_bytes[..s.len()].clone_from_slice(s.as_bytes());

        fixed_bytes
    }

    #[test]
    fn test_try_new() {
        let mfr_code = "ASDF";
        let mfr_serial = "1234";

        let serial_number = SerialNumber::try_new(mfr_code, mfr_serial).unwrap();

        assert_eq!(serial_number.mfr_code(), mfr_code);
        assert_eq!(serial_number.mfr_serial(), mfr_serial);
    }

    #[test]
    fn test_try_new_fails_invalid_length() {
        let mfr_code = "ASDF";
        let mfr_serial = "1234";

        let too_short_mfr_code = "AAA";
        let too_long_mfr_code = "AAAAA";

        let too_short_mfr_serial = "";
        let too_long_mfr_serial = "AAAAAAAAAAAAAAAA";

        assert!(SerialNumber::try_new(too_short_mfr_code, mfr_serial).is_err());
        assert!(SerialNumber::try_new(too_long_mfr_code, mfr_serial).is_err());
        assert!(SerialNumber::try_new(mfr_code, too_short_mfr_serial).is_err());
        assert!(SerialNumber::try_new(mfr_code, too_long_mfr_serial).is_err());
    }

    #[test]
    fn test_try_new_fails_invalid_chars() {
        let mfr_code = "ASDF";
        let mfr_serial = "1234";

        let invalid_mfr_code = "λλλλ";
        let invalid_mfr_serial = "λλλλ";

        assert!(SerialNumber::try_new(mfr_code, invalid_mfr_serial).is_err());
        assert!(SerialNumber::try_new(invalid_mfr_code, mfr_serial).is_err());
    }

    #[test]
    fn test_encode() {
        let mfr_code = "ASDF";
        let mfr_serial = "1234";
        let total = str_to_fixed_bytes("ASDF41234");

        let serial_number = SerialNumber::try_new(mfr_code, mfr_serial).unwrap();

        let mut encoded = [0u8; 20];
        serial_number.try_serialize(&mut encoded).unwrap();

        assert_eq!(encoded, total);
    }

    #[test]
    fn test_encode_fails_invalid_length() {
        let mut too_short = [0u8; 19];
        let mut too_long = [0u8; 21];

        let serial_number = SerialNumber::try_new("ASDF", "1324").unwrap();

        assert!(serial_number.try_serialize(&mut too_short).is_err());
        assert!(serial_number.try_serialize(&mut too_long).is_err());
    }

    #[test]
    fn test_decode() {
        let mfr_code = "ASDF";
        let mfr_serial = "1234";
        let total = str_to_fixed_bytes("ASDF41234");

        let serial_number = SerialNumber::try_new(mfr_code, mfr_serial).unwrap();

        let decoded = SerialNumber::try_from(total.as_ref()).unwrap();

        assert_eq!(serial_number, decoded);
    }

    #[test]
    fn test_decode_fails_invalid_length() {
        let too_short = [0u8; 19];
        let too_long = [0u8; 21];

        assert!(SerialNumber::try_from(too_short.as_ref()).is_err());
        assert!(SerialNumber::try_from(too_long.as_ref()).is_err());
    }

    #[test]
    fn test_decode_fails_invalid_character() {
        let invalid = str_to_fixed_bytes("λλλλ4λλλλ");

        assert!(SerialNumber::try_from(invalid.as_ref()).is_err());
    }

    #[test]
    fn test_decode_fails_invalid_len_character() {
        let invalid = str_to_fixed_bytes("AAAAλAAAA");

        assert!(SerialNumber::try_from(invalid.as_ref()).is_err());
    }
}
