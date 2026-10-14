//! `crownos_surface_animation_v1`: the compositor springs a subsurface's
//! transform and opacity, so the client does not render every frame of the
//! motion.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-surface-animation-v1.xml");
}
