//! Spoof and trust verdicts for received Remote ID frames.
//!
//! This crate decodes the ASTM F3411-22a message set but returns no judgement
//! about whether a broadcast is real. Remote ID is unauthenticated in almost
//! every aircraft flying today, and public tools broadcast fake drones from a
//! laptop or a five dollar microcontroller, so a receiver that only decodes
//! fields trusts a spoofer exactly as much as a real aircraft.
//!
//! Behind the `trust` feature, this module feeds the same raw message bytes
//! this crate decodes into the [`remoteid`] trust engine (maintained by
//! Matthew Busel) and returns a graded [`Verdict`] with the reasons behind it.
//! Nothing is dropped; the verdict is advice for a detector.

pub use remoteid::trust::{Assessment, Reason, TrustConfig, TrustState, Verdict};
pub use remoteid::{Observation, Transport};

/// A new, empty per-transmitter trust accumulator.
pub fn trust_state() -> TrustState {
    TrustState::new()
}

/// Grades one received frame against the running evidence for a transmitter.
///
/// `message` is the raw Remote ID message, header byte first, exactly the
/// slice [`crate::messages::Message::try_from`] accepts: 25 bytes, or
/// `2 + 25 * message_count` for a [`crate::pack::Pack`]. `transport` is the
/// link it arrived on, `counter` the frame's message counter when the
/// transport carries one, and `now_ms` the receiver's monotonic clock in
/// milliseconds.
pub fn assess(
    state: &mut TrustState,
    config: &TrustConfig,
    message: &[u8],
    transport: Transport,
    counter: Option<u8>,
    now_ms: u64,
) -> Assessment {
    let observation = Observation::new(transport, counter, message, now_ms);
    state.observe(config, &observation)
}

#[cfg(test)]
mod tests {
    use super::{Reason, Transport, TrustConfig, Verdict, assess, trust_state};
    use crate::basic_id::{BasicID, UASID, UAType, UTMAssignedUUID};
    use crate::messages::Message;
    use crate::try_serialize::TrySerialize;

    fn encode_basic_id(id: [u8; 20]) -> [u8; 25] {
        let basic_id = BasicID::new(
            UAType::Aeroplane,
            UASID::UTMAssignedUUID(UTMAssignedUUID::new(id)),
        );
        let mut buffer = [0u8; 25];
        Message::new(crate::messages::MessageType::BasicID(basic_id))
            .try_serialize(&mut buffer)
            .unwrap();
        buffer
    }

    // Two different aircraft identities broadcast under one message counter is a
    // spoofer sharing one address. drone-id decodes both frames without
    // complaint; the trust layer flags the conflict and returns Spoofed.
    #[test]
    fn counter_reuse_with_new_content_is_spoofed() {
        let real = encode_basic_id([2u8; 20]);
        let impostor = encode_basic_id([3u8; 20]);

        // Both frames decode fine through drone-id alone: no verdict either way.
        assert!(Message::try_from(real.as_ref()).is_ok());
        assert!(Message::try_from(impostor.as_ref()).is_ok());

        let cfg = TrustConfig::default();
        let mut state = trust_state();
        let mut assessment = assess(&mut state, &cfg, &real, Transport::Bluetooth4, Some(5), 0);

        for i in 0..10u64 {
            for (k, frame) in [(0u64, &real), (1, &impostor)] {
                assessment = assess(
                    &mut state,
                    &cfg,
                    frame,
                    Transport::Bluetooth4,
                    Some(5),
                    i * 10 + k,
                );
            }
        }

        assert!(
            assessment.reasons.contains(Reason::CounterConflict),
            "{:?}",
            assessment.reasons
        );
        assert_eq!(assessment.verdict, Verdict::Spoofed);
    }
}
