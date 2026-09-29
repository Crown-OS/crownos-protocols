//! `crownos_virtual_output_v1`: outputs with no display behind them, whose
//! frame clock is driven by the client that captures them.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-virtual-output-v1.xml");
}
