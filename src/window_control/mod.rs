//! `crownos_window_control_v1`: the window list with placement and process, and control over it, for the agent daemon.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-window-control-v1.xml");
}
