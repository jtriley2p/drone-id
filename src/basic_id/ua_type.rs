use crate::error::Error;

/// Unmanned Aircraft Type
///
/// This may be used to infer the flight characteristics of the aircraft.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UAType {
    /// Undeclared.
    NotDeclared,
    /// Fixed wing aircraft.
    Aeroplane,
    /// Rotor wing, either helicopter or multi-copter.
    Helicopter,
    /// Unpowered rotor wing using autorotation.
    Gyroplane,
    /// Vertical Take-off and Landing (VTOL) UAS with an alternative flight characteristic, such as
    /// a fixed wing.
    HybridLift,
    /// UAS that uses flapping wings to generate lift.
    Ornithopter,
    /// Fixed wing UAS, often unpowered.
    Glider,
    /// Tethered UAS using moving-air to generate lift.
    Kite,
    /// Unguided, untethered balloon using gas buoyancy to generate lift.
    FreeBalloon,
    /// Tethered or piloted balloon using gas buoyancy to generate lift.
    CaptiveBalloon,
    /// A type of missile UAS, presumably unpowered.
    FreeFall,
    /// Rocket propelled UAS
    Rocket,
    /// UAS which communicates via a wired connection.
    TetheredPoweredAircraft,
    /// Ground obstacle, specification is unclear as to what this is.
    GroundObstacle,
    /// Unlisted but not undeclared.
    Other,
}

impl TryFrom<u8> for UAType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UAType::NotDeclared),
            1 => Ok(UAType::Aeroplane),
            2 => Ok(UAType::Helicopter),
            3 => Ok(UAType::Gyroplane),
            4 => Ok(UAType::HybridLift),
            5 => Ok(UAType::Ornithopter),
            6 => Ok(UAType::Glider),
            7 => Ok(UAType::Kite),
            8 => Ok(UAType::FreeBalloon),
            9 => Ok(UAType::CaptiveBalloon),
            10 => Ok(UAType::FreeFall),
            11 => Ok(UAType::Rocket),
            12 => Ok(UAType::TetheredPoweredAircraft),
            13 => Ok(UAType::GroundObstacle),
            14 => Ok(UAType::Other),
            _ => Err(Error::InvalidInteger),
        }
    }
}

impl From<UAType> for u8 {
    fn from(value: UAType) -> Self {
        value as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::basic_id::UAType;

    #[test]
    fn test_encode() {
        let ornithopter = UAType::Ornithopter;

        let encoded = ornithopter.try_into().unwrap();

        assert_eq!(5u8, encoded);
    }

    #[test]
    fn test_decode() {
        let ornithopter_number = 5u8;

        let ornithopter = UAType::Ornithopter;

        assert_eq!(ornithopter, ornithopter_number.try_into().unwrap());
    }

    #[test]
    fn test_decode_fails_invalid_value() {
        let invalid = 15;

        assert!(UAType::try_from(invalid).is_err());
    }
}
