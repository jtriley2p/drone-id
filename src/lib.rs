//! # Drone ID
//!
//! The Drone ID Rust library implemets the core data types as well as the encoding and decoding
//! schemes of the ASTM F3411-22a specification, as required by the Federal Aviation Administration
//! in the United States.
//!
//! | Note: Drone, Unmanned Aerial System (UAS), and Unmanned Aircraft (UA) may be used
//! | interchangably in the documentation.
//!
//! ## Message Types
//!
//! The core [`Message`](messages) data type encapsulates all other messages, it is the entry point
//! for serialization or deserialization.
//!
//! There are seven distinct message types which may be broadcast by a UAS.
//!
//! 1. [`Authentication`](authentication) messages provide authentication information.
//! 2. [`BasicID`](basic_id) messages identify the UAS with an aircraft type and unique identifier.
//! 3. [`Location`](location) messages indicate location, speed, and direction of the UAS.
//! 4. [`OperatorID`](operator_id) messages identify the pilot of the UAS with a unique Identifier.
//! 5. [`Pack`](pack) messages group together up to nine other messages.
//! 6. [`SelfID`](self_id) messages are free-text indicating to observers the purpose of the flight.
//! 7. [`System`](system) messages indicate operating area and operator information.
//!
//! ## Decoding
//!
//! Decoding should generally be performed through [`Message::try_from`](messages::Message), which
//! takes a slice reference which must be 25 bytes unless the [`Pack`](pack::Pack) message is used,
//! in which case the slice must be of length `25 * message_count + 2`.
//!
//! Partial decoding is possible, for example if a library consumer is certain the message will
//! always be the [`BasicID`](basic_id::BasicID) type, they may use
//! [`BasicID::try_from`](basic_id::BasicID) directly, but the proper length must be used; in this
//! case it is 24 bytes.
//!
//! ## Encoding
//!
//! Encoding should generally be performed through the [`try_serialize::TrySerialize`] trait
//! implemented on [`Message`](messages::Message), which takes a mutable slice reference which must
//! be 25 bytes unless the [`Pack`](pack::Pack) message is used, in which case the slice must be of
//! length `25 * message_count + 2`. The proper length can be found with
//! [`Message::encoding_byte_length`](messages::Message::encoding_byte_length).
//!
//! As above, partial encoding is possible, for example if a library consumer only intends to
//! encode the [`BasicID`](basic_id::BasicID) type, they may use [`try_serialize::TrySerialize`]
//! directly, but the proper length must be used; in this case it is 24 bytes.
#![no_std]
#![warn(missing_docs)]

pub mod authentication;
pub mod basic_id;
pub mod error;
pub mod location;
pub mod messages;
pub mod operator_id;
pub mod pack;
pub mod self_id;
pub mod system;
pub mod try_serialize;
