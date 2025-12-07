//! ## Location Message
//!
//! [`Location`] messages indicate location, speed, and direction of the UAS, as well as timestamp
//! information and accuracy indicators of each metric.
//!
//! ### On Altitude and Height
//!
//! The specification calls for "pressure altitude", "geodetic altitude", and "height" parameters.
//! Each of these use the same core data type, [`Altitude`], though their means of measurement and
//! context are different; however all of their resolutions must be within one meter.
//!
//! Pressure altitude, or barometric altitude, is given by an atmospheric pressure measurement,
//! converted to altitude above the "standard datum plane", which is the theoretical altitude at
//! which the barometric pressure is 29.921 iHg (inches of mercury) or 1,013.2 mbar (millibars).
//!
//! Geodetic altitude is the distance above (or below) the Earth measured by a line from the UAS to
//! the ellipsoid given by the World Geodetic System of 1984 (WGS-84). This is often measured by
//! readings given from GPS satellites.
//!
//! Height is the measurement of the UAS's position above (or below) either the takeoff altitude or
//! ground level. Height above ground level (AGL) may be measured with a radar altimeter.
//!
//! ### Information Provided
//!
//! - `operational_status` is the UAS status and may be used to signal an emergency.
//! - `height_type` indicates whether the `height` reading is above ground level or takeoff.
//! - `track_direction` is the UAS's angle relative to True North.
//! - `speed` is the UAS's ground speed.
//! - `vertical_speed` is the UAS's speed on the vertical axis.
//! - `latitude` is the UAS's latitude angle.
//! - `longitude` is the UAS's longitude angle
//! - `pressure_altitude` is the UAS's altitude given by barometric pressure.
//! - `geodetic_altitude` is the UAS's altitude above the WGS-84 ellipsoid.
//! - `height` is the UAS's altitude either AGL or above takeoff altitude.
//! - `vertical_accuracy` is the accuracy of the `geodetic_altitude`.
//! - `horizontal_accuracy` is the accuracy of `longitude` and `latitude`.
//! - `altitude_accuracy` is the accuracy of the `pressure_altitude`.
//! - `speed_accuracy` is the accuracy of `speed`.
//! - `timestamp` is the number of tenths of a second since the most recent hour.
//! - `timestamp_accuracy` is the accuracy of the `timestamp`.
mod altitude;
mod ground_speed;
mod height_type;
mod horizontal_accuracy;
mod latitude;
mod longitude;
mod operational_status;
mod speed_accuracy;
mod timestamp;
mod timestamp_accuracy;
mod track_direction;
mod vertical_accuracy;
mod vertical_speed;

pub use altitude::Altitude;
pub use ground_speed::GroundSpeed;
pub use height_type::HeightType;
pub use horizontal_accuracy::HorizontalAccuracy;
pub use latitude::Latitude;
pub use longitude::Longitude;
pub use operational_status::OperationalStatus;
pub use speed_accuracy::SpeedAccuracy;
pub use timestamp::Timestamp;
pub use timestamp_accuracy::TimestampAccuracy;
pub use track_direction::TrackDirection;
pub use vertical_accuracy::VerticalAccuracy;
pub use vertical_speed::VerticalSpeed;

use crate::error::Error;
use crate::try_serialize::TrySerialize;

/// Location Message
///
/// Contains information on the aircraft's location, speed, direction, and accuracy of each
/// measurement.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Location {
    operational_status: OperationalStatus,
    height_type: HeightType,
    track_direction: TrackDirection,
    speed: GroundSpeed,
    vertical_speed: VerticalSpeed,
    latitude: Latitude,
    longitude: Longitude,
    pressure_altitude: Altitude,
    geodetic_altitude: Altitude,
    height: Altitude,
    vertical_accuracy: VerticalAccuracy,
    horizontal_accuracy: HorizontalAccuracy,
    altitude_accuracy: VerticalAccuracy,
    speed_accuracy: SpeedAccuracy,
    timestamp: Timestamp,
    timestamp_accuracy: TimestampAccuracy,
}

impl Location {
    /// Constructs a new Location.
    pub fn new(
        operational_status: OperationalStatus,
        height_type: HeightType,
        track_direction: TrackDirection,
        speed: GroundSpeed,
        vertical_speed: VerticalSpeed,
        latitude: Latitude,
        longitude: Longitude,
        pressure_altitude: Altitude,
        geodetic_altitude: Altitude,
        height: Altitude,
        vertical_accuracy: VerticalAccuracy,
        horizontal_accuracy: HorizontalAccuracy,
        altitude_accuracy: VerticalAccuracy,
        speed_accuracy: SpeedAccuracy,
        timestamp: Timestamp,
        timestamp_accuracy: TimestampAccuracy,
    ) -> Self {
        Self {
            operational_status,
            height_type,
            track_direction,
            speed,
            vertical_speed,
            latitude,
            longitude,
            pressure_altitude,
            geodetic_altitude,
            height,
            vertical_accuracy,
            horizontal_accuracy,
            altitude_accuracy,
            speed_accuracy,
            timestamp,
            timestamp_accuracy,
        }
    }

    /// Returns the operational status.
    ///
    /// May be used to signal an emergency.
    pub fn operational_status(&self) -> OperationalStatus {
        self.operational_status
    }

    /// Returns whether the height is above ground level or relative to takeoff altitude.
    pub fn height_type(&self) -> HeightType {
        self.height_type
    }

    /// Returns the angle relatiev to True North.
    pub fn track_direction(&self) -> TrackDirection {
        self.track_direction
    }

    /// Returns the ground speed.
    pub fn speed(&self) -> GroundSpeed {
        self.speed
    }

    /// Returns the vertical speed.
    pub fn vertical_speed(&self) -> VerticalSpeed {
        self.vertical_speed
    }

    /// Returns the latitude angle.
    pub fn latitude(&self) -> Latitude {
        self.latitude
    }

    /// Returns the longitude angle.
    pub fn longitude(&self) -> Longitude {
        self.longitude
    }

    /// Returns the barometric pressure altitude.
    pub fn pressure_altitude(&self) -> Altitude {
        self.pressure_altitude
    }

    /// Returns the distance above the WGS-84 ellipsoid.
    pub fn geodetic_altitude(&self) -> Altitude {
        self.geodetic_altitude
    }

    /// Returns the height either above ground level or relative to takeoff altitude.
    ///
    /// Determine which it is through [`Location::height_type`].
    pub fn height(&self) -> Altitude {
        self.height
    }

    /// Returns the accuracy of the geodetic altitude.
    pub fn vertical_accuracy(&self) -> VerticalAccuracy {
        self.vertical_accuracy
    }

    /// Returns the accuracy of the latitude and longitude.
    pub fn horizontal_accuracy(&self) -> HorizontalAccuracy {
        self.horizontal_accuracy
    }

    /// Returns the accuracy of the barometric pressure altitude.
    pub fn altitude_accuracy(&self) -> VerticalAccuracy {
        self.altitude_accuracy
    }

    /// Returns the accuracy of the ground speed.
    pub fn speed_accuracy(&self) -> SpeedAccuracy {
        self.speed_accuracy
    }

    /// Returns the number of tenths of a second since the most recent hour.
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Returns the accuracy of the timestamp.
    pub fn timestamp_accuracy(&self) -> TimestampAccuracy {
        self.timestamp_accuracy
    }
}

impl TryFrom<&[u8]> for Location {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        let operational_status = (value[0] >> 4 & 0b0000_1111).try_into()?;

        let height_type: HeightType = (value[0] >> 2 & 1)
            .try_into()
            .map_err(|_| Error::Unreachable)
            .unwrap();

        let east_west_bit = value[0] >> 1 & 1 != 0;
        let ground_speed_use_low_precision = value[0] & 1 != 0;

        let track_direction = (east_west_bit, value[1]).into();

        let speed = (ground_speed_use_low_precision, value[2]).into();

        let vertical_speed = value[3].into();

        let latitude = i32::from_le_bytes([value[4], value[5], value[6], value[7]]).into();

        let longitude = i32::from_le_bytes([value[8], value[9], value[10], value[11]]).into();

        let pressure_altitude = u16::from_le_bytes([value[12], value[13]]).into();

        let geodetic_altitude = u16::from_le_bytes([value[14], value[15]]).into();

        let height = u16::from_le_bytes([value[16], value[17]]).into();

        let vertical_accuracy = (value[18] >> 4).into();

        let horizontal_accuracy = (value[18] & 0b0000_1111).into();

        let altitude_accuracy = (value[19] >> 4).into();

        let speed_accuracy = (value[19] & 0b0000_1111).into();

        let timestamp = u16::from_le_bytes([value[20], value[21]]).into();

        let timestamp_accuracy = (value[22] & 0b0000_1111).into();

        Ok(Self {
            operational_status,
            height_type,
            track_direction,
            speed,
            vertical_speed,
            latitude,
            longitude,
            pressure_altitude,
            geodetic_altitude,
            height,
            vertical_accuracy,
            horizontal_accuracy,
            altitude_accuracy,
            speed_accuracy,
            timestamp,
            timestamp_accuracy,
        })
    }
}

impl TrySerialize for Location {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        if buffer.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        let (east_west_bit, angle) = self.track_direction.into();
        let (ground_speed_use_low_precision, speed) = self.speed.into();

        let flags: u8 = u8::from(self.height_type) << 2
            | (east_west_bit as u8) << 1
            | ground_speed_use_low_precision as u8;

        buffer[0] = u8::from(self.operational_status) << 4 | flags;

        buffer[1] = angle;

        buffer[2] = speed;

        buffer[3] = u8::from(self.vertical_speed);

        buffer[4..8].clone_from_slice(&i32::from(self.latitude).to_le_bytes());

        buffer[8..12].clone_from_slice(&i32::from(self.longitude).to_le_bytes());

        buffer[12..14].clone_from_slice(&u16::from(self.pressure_altitude).to_le_bytes());

        buffer[14..16].clone_from_slice(&u16::from(self.geodetic_altitude).to_le_bytes());

        buffer[16..18].clone_from_slice(&u16::from(self.height).to_le_bytes());

        buffer[18] = u8::from(self.vertical_accuracy) << 4 | u8::from(self.horizontal_accuracy);

        buffer[19] = u8::from(self.altitude_accuracy) << 4 | u8::from(self.speed_accuracy);

        buffer[20..22].clone_from_slice(&u16::from(self.timestamp).to_le_bytes());

        buffer[22] = u8::from(self.timestamp_accuracy);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        location::{
            Altitude, GroundSpeed, HeightType, HorizontalAccuracy, Latitude, Location, Longitude,
            OperationalStatus, SpeedAccuracy, Timestamp, TimestampAccuracy, TrackDirection,
            VerticalAccuracy, VerticalSpeed,
        },
        try_serialize::TrySerialize,
    };

    #[test]
    fn test_getters() {
        let operational_status = OperationalStatus::Undeclared;
        let height_type = HeightType::AGL;
        let track_direction = TrackDirection::Unknown;
        let speed = GroundSpeed::Unknown;
        let vertical_speed = VerticalSpeed::Unknown;
        let latitude = Latitude::Unknown;
        let longitude = Longitude::Unknown;
        let pressure_altitude = Altitude::Unknown;
        let geodetic_altitude = Altitude::Unknown;
        let height = Altitude::Unknown;
        let vertical_accuracy = VerticalAccuracy::Unknown;
        let horizontal_accuracy = HorizontalAccuracy::Unknown;
        let altitude_accuracy = VerticalAccuracy::Unknown;
        let speed_accuracy = SpeedAccuracy::Unknown;
        let timestamp = Timestamp::Unknown;
        let timestamp_accuracy = TimestampAccuracy::Unknown;

        let location = Location::new(
            operational_status,
            height_type,
            track_direction,
            speed,
            vertical_speed,
            latitude,
            longitude,
            pressure_altitude,
            geodetic_altitude,
            height,
            vertical_accuracy,
            horizontal_accuracy,
            altitude_accuracy,
            speed_accuracy,
            timestamp,
            timestamp_accuracy,
        );

        assert_eq!(location.operational_status(), operational_status);
        assert_eq!(location.height_type(), height_type);
        assert_eq!(location.track_direction(), track_direction);
        assert_eq!(location.speed(), speed);
        assert_eq!(location.vertical_speed(), vertical_speed);
        assert_eq!(location.latitude(), latitude);
        assert_eq!(location.longitude(), longitude);
        assert_eq!(location.pressure_altitude(), pressure_altitude);
        assert_eq!(location.geodetic_altitude(), geodetic_altitude);
        assert_eq!(location.height(), height);
        assert_eq!(location.vertical_accuracy(), vertical_accuracy);
        assert_eq!(location.horizontal_accuracy(), horizontal_accuracy);
        assert_eq!(location.altitude_accuracy(), altitude_accuracy);
        assert_eq!(location.speed_accuracy(), speed_accuracy);
        assert_eq!(location.timestamp(), timestamp);
        assert_eq!(location.timestamp_accuracy(), timestamp_accuracy);
    }

    #[test]
    fn test_encode() {
        let operational_status = OperationalStatus::Undeclared;
        let height_type = HeightType::AGL;
        let track_direction = TrackDirection::Unknown;
        let speed = GroundSpeed::Unknown;
        let vertical_speed = VerticalSpeed::Unknown;
        let latitude = Latitude::Unknown;
        let longitude = Longitude::Unknown;
        let pressure_altitude = Altitude::Unknown;
        let geodetic_altitude = Altitude::Unknown;
        let height = Altitude::Unknown;
        let vertical_accuracy = VerticalAccuracy::Unknown;
        let horizontal_accuracy = HorizontalAccuracy::Unknown;
        let altitude_accuracy = VerticalAccuracy::Unknown;
        let speed_accuracy = SpeedAccuracy::Unknown;
        let timestamp = Timestamp::Unknown;
        let timestamp_accuracy = TimestampAccuracy::Unknown;

        let location = Location::new(
            operational_status,
            height_type,
            track_direction,
            speed,
            vertical_speed,
            latitude,
            longitude,
            pressure_altitude,
            geodetic_altitude,
            height,
            vertical_accuracy,
            horizontal_accuracy,
            altitude_accuracy,
            speed_accuracy,
            timestamp,
            timestamp_accuracy,
        );

        let mut encoded = [0u8; 24];
        location.try_serialize(&mut encoded).unwrap();

        let (east_west_bit, angle) = track_direction.into();
        let (ground_speed_use_low_precision, speed) = speed.into();

        let byte0 = u8::from(operational_status) << 4
            | u8::from(height_type) << 2
            | (east_west_bit as u8) << 1
            | ground_speed_use_low_precision as u8;

        assert_eq!(encoded[0], byte0);
        assert_eq!(encoded[1], angle);
        assert_eq!(encoded[2], speed);
        assert_eq!(encoded[3], u8::from(vertical_speed));
        assert_eq!(encoded[4..8], i32::from(latitude).to_le_bytes());
        assert_eq!(encoded[8..12], i32::from(longitude).to_le_bytes());
        assert_eq!(encoded[12..14], u16::from(pressure_altitude).to_le_bytes());
        assert_eq!(encoded[14..16], u16::from(geodetic_altitude).to_le_bytes());
        assert_eq!(encoded[16..18], u16::from(height).to_le_bytes());
        assert_eq!(
            encoded[18],
            u8::from(vertical_accuracy) << 4 | u8::from(horizontal_accuracy)
        );
        assert_eq!(
            encoded[19],
            u8::from(altitude_accuracy) << 4 | u8::from(speed_accuracy)
        );
        assert_eq!(encoded[20..22], u16::from(timestamp).to_le_bytes());
        assert_eq!(encoded[22], u8::from(timestamp_accuracy));
        assert_eq!(encoded[23], 0);
    }

    #[test]
    fn test_encode_fails_invalid_length() {
        let mut too_short = [0u8; 23];
        let mut too_long = [0u8; 25];

        let location = Location::new(
            OperationalStatus::Undeclared,
            HeightType::AGL,
            TrackDirection::Unknown,
            GroundSpeed::Unknown,
            VerticalSpeed::Unknown,
            Latitude::Unknown,
            Longitude::Unknown,
            Altitude::Unknown,
            Altitude::Unknown,
            Altitude::Unknown,
            VerticalAccuracy::Unknown,
            HorizontalAccuracy::Unknown,
            VerticalAccuracy::Unknown,
            SpeedAccuracy::Unknown,
            Timestamp::Unknown,
            TimestampAccuracy::Unknown,
        );

        assert!(location.try_serialize(&mut too_short).is_err());
        assert!(location.try_serialize(&mut too_long).is_err());
    }

    #[test]
    fn test_decode() {
        let operational_status = OperationalStatus::Undeclared;
        let height_type = HeightType::AGL;
        let track_direction = TrackDirection::Unknown;
        let speed = GroundSpeed::Unknown;
        let vertical_speed = VerticalSpeed::Unknown;
        let latitude = Latitude::Unknown;
        let longitude = Longitude::Unknown;
        let pressure_altitude = Altitude::Unknown;
        let geodetic_altitude = Altitude::Unknown;
        let height = Altitude::Unknown;
        let vertical_accuracy = VerticalAccuracy::Unknown;
        let horizontal_accuracy = HorizontalAccuracy::Unknown;
        let altitude_accuracy = VerticalAccuracy::Unknown;
        let speed_accuracy = SpeedAccuracy::Unknown;
        let timestamp = Timestamp::Unknown;
        let timestamp_accuracy = TimestampAccuracy::Unknown;

        let mut encoded = [0u8; 24];

        let (east_west_bit, angle) = track_direction.into();
        let (ground_speed_use_low_precision, encoded_speed) = speed.into();

        let byte0 = u8::from(operational_status) << 4
            | u8::from(height_type) << 2
            | (east_west_bit as u8) << 1
            | ground_speed_use_low_precision as u8;

        encoded[0] = byte0;
        encoded[1] = angle;
        encoded[2] = encoded_speed;
        encoded[3] = u8::from(vertical_speed);
        encoded[4..8].clone_from_slice(&i32::from(latitude).to_le_bytes());
        encoded[8..12].clone_from_slice(&i32::from(longitude).to_le_bytes());
        encoded[12..14].clone_from_slice(&u16::from(pressure_altitude).to_le_bytes());
        encoded[14..16].clone_from_slice(&u16::from(geodetic_altitude).to_le_bytes());
        encoded[16..18].clone_from_slice(&u16::from(height).to_le_bytes());
        encoded[18] = u8::from(vertical_accuracy) << 4 | u8::from(horizontal_accuracy);
        encoded[19] = u8::from(altitude_accuracy) << 4 | u8::from(speed_accuracy);
        encoded[20..22].clone_from_slice(&u16::from(timestamp).to_le_bytes());
        encoded[22] = u8::from(timestamp_accuracy);

        let expected = Location::new(
            operational_status,
            height_type,
            track_direction,
            speed,
            vertical_speed,
            latitude,
            longitude,
            pressure_altitude,
            geodetic_altitude,
            height,
            vertical_accuracy,
            horizontal_accuracy,
            altitude_accuracy,
            speed_accuracy,
            timestamp,
            timestamp_accuracy,
        );

        assert_eq!(Location::try_from(encoded.as_ref()).unwrap(), expected);
    }

    #[test]
    fn test_decode_fails_invalid_length() {
        let too_short = [0u8; 23];
        let too_long = [0u8; 25];

        assert!(Location::try_from(too_short.as_ref()).is_err());
        assert!(Location::try_from(too_long.as_ref()).is_err());
    }
}
