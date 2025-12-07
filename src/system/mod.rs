//! ## System Message
//!
//! [`System`] messages indicate operating area and operator information.
//!
//! ### Operator Information
//!
//! The operator's latitude, longitude, and altitude are encapsulated, as well as what "type" of
//! location information it is. The types of location information may be the "take-off" location, a
//! fixed operator location, or a dynamic operator location.
//!
//! ### Operating Area Information
//!
//! The operating area information includes the number of UAS's operating, the radius, ceiling
//! altitude, and floor altitude.
//!
//! ### Classification Information
//!
//! The classification information specification is a bit messier. The
//! [`System::classification_type`] is of type [`ClassificationType`] and can be "undeclared" or
//! "European Union". The [`System::ua_classification`] is of type [`UAClassification`] and can be
//! undefined, specific, certified, or "open". If the UA classification is "open", it encapsulates
//! [`OpenClassification`], which may be undefined or of class zero to six.
//!
//! ### On System Altitudes
//!
//! The altitudes of the operator, operating area floor, and operating area ceiling are geodetic,
//! that is, they are the height above the WGS-84 ellipsoid.

mod area_count;
mod classification_type;
mod operating_area_radius;
mod operator_location_source_type;
mod timestamp;
mod ua_classification;

pub use area_count::AreaCount;
pub use classification_type::ClassificationType;
pub use operating_area_radius::OperatingAreaRadius;
pub use operator_location_source_type::OperatorLocationSourceType;
pub use timestamp::Timestamp;
pub use ua_classification::OpenClassification;
pub use ua_classification::UAClassification;

use crate::error::Error;
use crate::location::Altitude;
use crate::location::Latitude;
use crate::location::Longitude;
use crate::try_serialize::TrySerialize;

/// System Message
///
/// Encapsulates operator location information, classification of the aircraft, a timestamp, and
/// area parameters such as the radius, ceiling, floor, and number of aircraft operating in the
/// area.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct System {
    classification_type: ClassificationType,
    operator_location_source_type: OperatorLocationSourceType,
    operator_latitude: Latitude,
    operator_longitude: Longitude,
    area_count: AreaCount,
    area_radius: OperatingAreaRadius,
    area_ceiling: Altitude,
    area_floor: Altitude,
    ua_classification: UAClassification,
    operator_altitude: Altitude,
    timestamp: Timestamp,
}

impl System {
    /// Constructs a new System message.
    pub fn new(
        classification_type: ClassificationType,
        operator_location_source_type: OperatorLocationSourceType,
        operator_latitude: Latitude,
        operator_longitude: Longitude,
        area_count: AreaCount,
        area_radius: OperatingAreaRadius,
        area_ceiling: Altitude,
        area_floor: Altitude,
        ua_classification: UAClassification,
        operator_altitude: Altitude,
        timestamp: Timestamp,
    ) -> Self {
        Self {
            classification_type,
            operator_location_source_type,
            operator_latitude,
            operator_longitude,
            area_count,
            area_radius,
            area_ceiling,
            area_floor,
            ua_classification,
            operator_altitude,
            timestamp,
        }
    }

    /// Returns the classification type.
    ///
    /// See the [classification disambiguation](crate::system) for more.
    pub fn classification_type(&self) -> ClassificationType {
        self.classification_type
    }

    /// Returns the operator location's source type.
    pub fn operator_location_source_type(&self) -> OperatorLocationSourceType {
        self.operator_location_source_type
    }

    /// Returns the operator's latitude.
    pub fn operator_latitude(&self) -> Latitude {
        self.operator_latitude
    }

    /// Returns the operator longitude.
    pub fn operator_longitude(&self) -> Longitude {
        self.operator_longitude
    }

    /// Returns the number of UAS's in the operating area.
    pub fn area_count(&self) -> AreaCount {
        self.area_count
    }

    /// Returns the radius of the operating area.
    pub fn area_radius(&self) -> OperatingAreaRadius {
        self.area_radius
    }

    /// Returns the ceiling altitude of the operating area.
    pub fn area_ceiling(&self) -> Altitude {
        self.area_ceiling
    }

    /// Returns the floor altitude of the operating area.
    pub fn area_floor(&self) -> Altitude {
        self.area_floor
    }

    /// Returns the UA classification.
    ///
    /// See the [classification disambiguation](crate::system) for more.
    pub fn ua_classification(&self) -> UAClassification {
        self.ua_classification
    }

    /// Returns the operator's altitude.
    pub fn operator_altitude(&self) -> Altitude {
        self.operator_altitude
    }

    /// Returns the timestamp.
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}

impl TryFrom<&[u8]> for System {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        let classification_type = (value[0] >> 2 & 0b0000_0011).try_into()?;

        let operator_location_source_type = (value[0] & 0b0000_0011).try_into()?;

        let operator_latitude = i32::from_le_bytes([value[1], value[2], value[3], value[4]]).into();

        let operator_longitude =
            i32::from_le_bytes([value[5], value[6], value[7], value[8]]).into();

        let area_count = u16::from_le_bytes([value[9], value[10]]).try_into()?;

        let area_radius = value[11].into();

        let area_ceiling = u16::from_le_bytes([value[12], value[13]]).into();

        let area_floor = u16::from_le_bytes([value[14], value[15]]).into();

        let ua_classification = value[16].into();

        let operator_altitude = u16::from_le_bytes([value[17], value[18]]).into();

        let timestamp = u32::from_le_bytes([value[19], value[20], value[21], value[22]]).into();

        Ok(Self {
            classification_type,
            operator_location_source_type,
            operator_latitude,
            operator_longitude,
            area_count,
            area_radius,
            area_ceiling,
            area_floor,
            ua_classification,
            operator_altitude,
            timestamp,
        })
    }
}

impl TrySerialize for System {
    type Error = Error;

    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        if buffer.len() != 24 {
            return Err(Error::InvalidDataLength);
        }

        buffer[0] =
            u8::from(self.classification_type) << 2 | u8::from(self.operator_location_source_type);
        buffer[1..5].clone_from_slice(&i32::from(self.operator_latitude).to_le_bytes());
        buffer[5..9].clone_from_slice(&i32::from(self.operator_longitude).to_le_bytes());
        buffer[9..11].clone_from_slice(&u16::from(self.area_count).to_le_bytes());
        buffer[11] = u8::from(self.area_radius);
        buffer[12..14].clone_from_slice(&u16::from(self.area_ceiling).to_le_bytes());
        buffer[14..16].clone_from_slice(&u16::from(self.area_floor).to_le_bytes());
        buffer[16] = u8::from(self.ua_classification);
        buffer[17..19].clone_from_slice(&u16::from(self.operator_altitude).to_le_bytes());
        buffer[19..23].clone_from_slice(&u32::from(self.timestamp).to_le_bytes());

        Ok(())
    }
}
