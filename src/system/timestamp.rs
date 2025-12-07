/// Timestamp for System and Authentication Messages
///
/// Differs from [`crate::location::Timestamp`], as this encapsulates a 32-bit unsigned
/// integer representing the number of seconds since the "epoch", not the Unix timestamp epoch, but
/// a custom epoch starting at the start of the year 2019 (00:00:00 01/01/2019).
///
/// Adjusting to the Unix timestamp may be done by adding [`Timestamp::UNIX_TIMESTAMP_OFFSET`].
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Timestamp(u32);

impl Timestamp {
    /// Number of seconds to add to [`Timestamp`].
    pub const UNIX_TIMESTAMP_OFFSET: u64 = 1_546_300_800;

    /// Constructs a new timestamp.
    ///
    /// NOTICE: Double check the [`Timestamp`] epoch, it is NOT the Unix epoch.
    pub fn new(timestamp: u32) -> Self {
        Self(timestamp)
    }

    /// Returns the raw inner time, the number of seconds since the custom "epoch".
    pub fn system_time(&self) -> u32 {
        self.0
    }

    /// Converts the system time to unix time.
    ///
    /// Also returns a 64-bit unsigned integer to avoid the Unix timestamp rollover in 2038.
    pub fn unix_time(&self) -> u64 {
        self.0 as u64 + Self::UNIX_TIMESTAMP_OFFSET
    }

    /// Converts unix time to the system time.
    ///
    /// Also reduces to the internal 32-bit unsigned integer since such a rollover would not occur
    /// until 2087.
    pub fn from_unix_time(unix_time: u64) -> Self {
        let system_time = (unix_time - Self::UNIX_TIMESTAMP_OFFSET) as u32;

        Self(system_time)
    }
}

impl From<u32> for Timestamp {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Timestamp> for u32 {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::system::Timestamp;

    #[test]
    fn test_getters() {
        let timestamp = Timestamp::new(1);

        assert_eq!(timestamp.system_time(), 1);
        assert_eq!(timestamp.unix_time(), Timestamp::UNIX_TIMESTAMP_OFFSET + 1);
    }

    #[test]
    fn test_from_unix_time() {
        let timestamp = Timestamp::from_unix_time(Timestamp::UNIX_TIMESTAMP_OFFSET + 1);

        assert_eq!(timestamp, Timestamp::new(1));
    }

    #[test]
    fn test_encode() {
        let timestamp = Timestamp::new(1);

        assert_eq!(u32::from(timestamp), 1);
    }

    #[test]
    fn test_decode() {
        let decoded = Timestamp::from(1);

        assert_eq!(decoded, Timestamp::new(1));
    }
}
