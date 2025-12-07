//! ## Enumerated Errors
//!
//! The errors are relatively limited in scope to serialization and deserialization of data types.
//!
//! Invalid data length indicates the slice reference, be it the mutable one for serializing or the
//! immutable one for deserializing, is of the wrong length. During each we provide each data type
//! with only the subslice required for their portion of (de)serialization; this also allows for
//! partial (de)serialization for users only interacting with a subset of the messages or data
//! types.
//!
//! Invalid integer generally refers to the construction of a value being incorrect. Some enumerated
//! values contain an `Invalid` variant, in which case construction through deserialization would
//! simply return `Ok(DataType::Invalid)`, though cases where these are not covered, we return an
//! error `Err(Error::InvalidInteger)`.
//!
//! Invalid Registration ID refers to a malformed
//! [`RegistrationID`](crate::basic_id::RegistrationID).
//!
//! Invalid Serial Number refers to a malformed [`SerialNumber`](crate::basic_id::SerialNumber).
//!
//! Cannot Recursively Pack refers to a [`Pack`](crate::pack::Pack) message which contains in itself
//! another Pack message.
//!
//! Invalid Protocol Version refers to a bytes array deserializing to
//! [`Message`](crate::messages::Message) but which contains a protocol version other than `2`.
//!
//! Unreachable is a special error value. Per the convention of this library, we only allow `unwrap`
//! operations on [`Error::Unreachable`] to make explicit it is not reachable. If you are a library
//! consumer and have seen this error in a panic message (or otherwise), please open a bug report.

/// Error Enumeration
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Error {
    /// Invalid byte length.
    InvalidDataLength,
    /// Invalid integer value.
    ///
    /// Invalid integers do not always trigger this error, in particular data types with a member
    /// such as `DataType::Invalid` can encapsulate an otherwise invalid value. This error is
    /// triggered when the target value has no means of encapsulating the invalid integer, per the
    /// specification.
    InvalidInteger,
    /// Invalid [`RegistrationID`](crate::basic_id::RegistrationID).
    InvalidRegistrationID,
    /// Invalid [`SerialNumber`](crate::basic_id::SerialNumber).
    InvalidSerialNumber,
    /// [`Pack`](crate::pack::Pack) cannot recursively contain pack messages.
    CannotRecursivelyPack,
    /// Protocol version is not
    /// [`Message::PROTOCOL_VERSION`](crate::messages::Message::PROTOCOL_VERSION).
    InvalidProtocolVersion,
    /// Unreachable.
    ///
    /// If you see this error in a panic trace, this is a bug, please open a bug report.
    Unreachable,
}
