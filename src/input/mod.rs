//! `crownos_input_v1`: injecting pointer, keyboard and touch input into a
//! seat, and capturing a seat's input when the pointer leaves the layout
//! through an armed edge.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-input-v1.xml");
}
