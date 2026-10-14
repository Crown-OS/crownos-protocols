//! `crownos_surface_visibility_v1`: whether a surface is drawn, covered or
//! hidden, and the scale it is drawn at while shown as a thumbnail.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-surface-visibility-v1.xml");
}
