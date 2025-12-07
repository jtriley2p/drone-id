//! ## Try Serialize Trait
//!
//! For uniformity, we define a [`TrySerialize`] trait which each variant of
//! [`Message`](crate::messages::Message) must implement, as well as any relevant internal data
//! types which do not convert to primitive integer types.
//!
//! Data types which implement `From<T> for u8` or any other small interger type may omit this
//! trait, as they will be serialized higher up the type hierarchy.

/// Try Serialize
pub trait TrySerialize {
    /// Internal `Error` data type to allow for other error definitions.
    type Error;

    /// Tries to seriaize a value into a mutable reference to a byte buffer.
    fn try_serialize(&self, buffer: &mut [u8]) -> Result<(), Self::Error>;
}
