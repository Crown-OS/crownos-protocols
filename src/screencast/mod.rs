//! `crownos_screencast_v1`: zero-copy, encoder-paced capture of an output into
//! a ring of client-owned dmabufs, with damage, optional explicit sync and
//! cursor metadata.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-screencast-v1.xml");
}
