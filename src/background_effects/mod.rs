//! `crownos_background_effects`: server-side material effects behind and
//! around a surface — a blurred, tinted and saturated backdrop under a
//! parametric shape, a refractive rim along its edge, a corner radius, and a
//! drop shadow.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-background-effects-v1.xml");
}
