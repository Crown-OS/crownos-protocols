//! `crownos_workspace_mode_v1`: whether the focused workspace tiles its
//! windows or lets them float, and switching it between the two.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-workspace-mode-v1.xml");
}
