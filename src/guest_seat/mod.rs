//! `crownos_guest_seat_v1`: a seat of a remote desktop guest's own, confined to the output their session shows.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-guest-seat-v1.xml");
}
