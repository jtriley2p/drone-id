use crate::error::Error;

/// Operating Area Radius
///
/// Contains the area, in meters, of the radius of the operating area.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct OperatingAreaRadius(u16);

impl OperatingAreaRadius {
    /// Special value representing the maximum operating area radius.
    pub const MAX: u16 = 2_550;

    /// Multiplier to adjust radius.
    pub const MULTIPLIER: u16 = 10;

    /// Constructs a new operating area radius
    ///
    /// Returns an error if:
    ///
    /// - value is greater than `2550` (`u8::MAX * 10`).
    pub fn try_new(radius: u16) -> Result<Self, Error> {
        if radius > Self::MAX {
            return Err(Error::InvalidInteger);
        }

        Ok(Self(radius))
    }

    /// Returns the inner radius.
    pub fn radius(&self) -> u16 {
        self.0
    }
}

impl From<u8> for OperatingAreaRadius {
    fn from(value: u8) -> Self {
        Self(value as u16 * Self::MULTIPLIER)
    }
}

impl From<OperatingAreaRadius> for u8 {
    fn from(value: OperatingAreaRadius) -> Self {
        (value.0 / OperatingAreaRadius::MULTIPLIER) as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::system::OperatingAreaRadius;

    #[test]
    fn test_try_new_fails_invalid_integer() {
        assert!(OperatingAreaRadius::try_new(OperatingAreaRadius::MAX + 1).is_err());
    }

    #[test]
    fn test_radius() {
        let radius = OperatingAreaRadius::try_new(10).unwrap();

        assert_eq!(radius.radius(), 10);
    }

    #[test]
    fn test_encode() {
        let radius = OperatingAreaRadius::try_new(10).unwrap();

        assert_eq!(u8::from(radius), 1);
    }

    #[test]
    fn test_decode() {
        let decoded = OperatingAreaRadius::from(1);

        assert_eq!(decoded, OperatingAreaRadius::try_new(10).unwrap());
    }
}
