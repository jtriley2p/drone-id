use crate::error::Error;

/// Number of aircraft in an area, group, or formation.
///
/// Possible values go up to 65,000 despite the maximum value of a 16 bit unsigned integer being
/// slightly larger than this.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct AreaCount(u16);

impl AreaCount {
    /// Maximum value of the [`AreaCount`].
    pub const MAX: u16 = 65_000;
}

impl TryFrom<u16> for AreaCount {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > Self::MAX {
            return Err(Error::InvalidInteger);
        }

        Ok(Self(value))
    }
}

impl From<AreaCount> for u16 {
    fn from(value: AreaCount) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::system::AreaCount;

    #[test]
    fn test_encode_decode() {
        let area_count = AreaCount::try_from(1).unwrap();

        assert_eq!(u16::from(area_count), 1)
    }

    #[test]
    fn test_decode_fails_invalid() {
        assert!(AreaCount::try_from(AreaCount::MAX + 1).is_err())
    }
}
