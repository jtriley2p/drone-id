# Drone ID

## Introduction

The federal aviation administration (faa) of the united states federal government commands all
people flying unmanned aerial systems (uas) greater than 250 grams in weight within its airspace to
broadcast uniquely identifying information about the uas which is registered to the operator in a
federal database. It also necessitates broadcasting information such as the craft's location, speed,
and direction.

But there's a catch.

The specification for such compliance is behind the paywall of a private entity. At the time of
writing implementors must pay one hundred and thirteen united states dollars to ASTM international
to access the specification for compliance with the law.

So the specification has been paid for and now this reference implementation optimizes for rich
typing, memory safety, and readability over raw performance.

> SOAPBOX: The passive aggressive README introduction is a bit of a soapbox as well but to make it
> abundantly clear: Putting federally mandated specifications behind the paywall of a private entity
> already screams "lobbying", be it from the organization which paywalls it or the manufacturers and
> implementors that intend to maximize the barrier to entry into their self-proclaimed "free
> market".
>
> Intellectual property laws which encourage us to hide our knowledge from our fellow humans and
> force duplicate work at best and damn our work to obscurity at worst must be abolished. It hinders
> human progress and encourages extraction on artificial scarcity of trivially replicable
> information.
>
> Liberate the source code, liberate the people.

## High Level Implementation and Purpose

We implement the ASTM F3411-22a specification, which defines core data types, encoding, and decoding
schemes regarding the Drone ID requirements. This library serves as a core library, dealing with
WiFi and Bluetooth packets is beyond the scope of this library; it is akin to the work of the
[Open Drone ID](https://github.com/opendroneid) community.

The Open Drone ID community also has several implementations of receivers and transmitters using
various hardware, which is great, though in some cases they can be difficult to reason about and to
compile. Using Rust allows us to both richly type and thoroughly express the requirements of the
specification and unifies the build system under the `cargo` package manager.

## On Timestamps

There are two kinds of timestamps in the specification, neither of which are the Unix timestamp.

The location message timestamp (in [`location::timestamp`](src/location/timestamp.rs)) represents
the number of tenths of seconds since the top of the most recent hour. So anywhere from zero to
3600, which may be recorded with an accuracy reported somewhere between half a second and one and a
half seconds.

The authentication and system timestamp (in [`system::timestamp`](src/system/timestamp.rs))
represents the number of seconds since the "epoch", which is defined as the start of 2019, rather
than the Unix timestamp's epoch starting in 1970. Adjusting from the system/authentication timestamp
to the Unix timestamp entails adding `1,546,300,800` to it.

## General Development Strategy

### Dependency Minimization

The core library should have no external, direct dependencies, though exceptions may be made for
developer dependencies for testing and benchmarking purposes. We also use `#![no_std]` to make the
core library friendly to embedded systems which may not have an operating system or dynamic memory
allocator.

If the community needs integration with external serialization libraries, this may also make for an
exception to the dependency minimization.

### Serializing and Deserializing

The general strategy for serializing and deserializing is to reasonably maximize for safety. While
the entrypoint to parse a message is using [`Message::try_from`](src/messages/mod.rs), which then
internally checks proper data lengths before parsing out respective internal types, we design the
data types to be used largely independently. As such, there will be many redundant checks internally
when parsing the message through the main entrypoint, but this preserves the internal invariants of
each type, for example if a user were to only parse [`BasicID::try_from`](src/basic_id/mod.rs)
directly.

When deserializing, we pass the full length slice to [`Message::try_from`](src/messages/mod.rs),
which may be between 25 bytes and 257 bytes, but internally each type receives a reference only to
the bytes they need. For example, [`BasicID::try_from`](src/basic_id/mod.rs) only requires 24 bytes,
so it will only receive 24 bytes. When serializing, we use the same strategy, but using a unified
mutable reference to the underlying slice; we do so through a custom
[`TrySerialize`](src/try_serialize.rs) trait.

### Unknown and Invalid Values

Many fields are specified to contain one of the following:

- invalid
- no value
- unknown
- known

The "unknown" value is generally indicated by one specific, unusual number which would not occur in
normal circumstances. The "invalid" value is sometimes clear, for example a latitude greater than 90
degrees, but in other cases this is not so clear; thus in practice there are some enumerations which
contain an "Invalid" field, per the specification, but there is no practical way to construct that
value from a byte parser. The "no value" value can also be complicated, as sometimes this refers to
"zero" when the expected value is non-zero, but may also sometimes imply that the value is not
encoded at all, for example when a local Civil Aviation Authority makes a field optional, but
messages are still fundamentally 25 bytes (except message packs).

### Error Handling and Panics

Additionally, though largely symbolic, we use `Result::unwrap` only on the `Error::Unreachable` type
of error. This is cleaner than pattern matching against the `unreachable!()` macro and also makes
explicit that any invocation of `Result::unwrap` is explicitly declared as unreachable by the
developer. The exceptions to this are indexing and slicing into slices using the `ident[expr]`
syntax, which implicitly panic on an out-of-bounds read, and may only be used _after_ constraining
the length to be at least the upper bound of `expr`. Finally we make the exception for
`<[T]>::clone_from_slice` which panics on a mismatched length between the source and destination
slices, as there is no non-panic alternative; once again this may only be used _after_ constraining
the lengths to be equivalent.

### Styling

Atop the conventions required through the `cargo fmt` command on default configuration, we
additionally style as follows.

- Flatten imports to one line each, avoiding `::{..}` syntax.
- Leave empty lines between expressions (where reasonable) to avoid dense clusters of code.
- Reasonably express variable names: sometimes they're tool long, but we optimize for readability.
- De-nest where possible: the squint test should show reasonably flat code.
- Prefer over-documentation: a variable's purpose or layout should not be ambiguous.
- Reasonably minimize magic values: scope consts to their appropriate types.
- Reasonably minimize indirection: repeat yourself for readability's sake.

> NOTICE: An effort may be made in the future to maximize for performance with more direct methods
> of serialization and deserialization. This depends on the community's need for such things.

### Testing

All functions, including serialization and deserialization must be tested. Each branch should be hit
by a test. Generally, this breaks down into a few consistent groups of tests:

- `test_encode`: construct type, serialize, check the serialized bytes against the test variables
- `test_decode`: construct type, serialize, deserialize, check all fields & getters match
- `test_encode_fails_invalid_length`: check the encoder returns err on an invalid data length
- `test_decode_fails_invalid_length`: check the decoder returns err on an invalid data length
- `test_encode_invalid_value`: check encoder returns err on invalid values
- `test_decode_invalid_value`: check decoder returns err on invalid values
