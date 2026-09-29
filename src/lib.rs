//! Bindings for the Wayland protocol extensions CrownOS defines for itself.
//!
//! Generated from the XML under `protocols/` at compile time, so the protocol
//! description is the single source of truth and no hand-written binding can
//! fall behind it. The layout mirrors [`wayland_protocols`]: one module per
//! protocol, each with a `client` and a `server` submodule behind the cargo
//! feature of the same name.

#![forbid(improper_ctypes, unsafe_op_in_unsafe_fn)]

#[macro_use]
mod generate;

pub mod background_effects;
pub mod input;
pub mod screencast;
pub mod virtual_output;

#[cfg(all(test, feature = "server"))]
mod tests {
    use super::background_effects::v1::server::*;

    /// The generated names the compositor binds against, checked here so a
    /// rename in the XML fails at the protocol crate rather than three crates
    /// downstream.
    #[test]
    fn the_generated_interfaces_are_named_as_expected() {
        use crownos_background_effect_manager_v1::Capability;
        use wayland_server::Resource as _;

        assert_eq!(
            crownos_shape_manager_v1::CrownosShapeManagerV1::interface().name,
            "crownos_shape_manager_v1"
        );
        assert_eq!(
            crownos_shape_v1::CrownosShapeV1::interface().name,
            "crownos_shape_v1"
        );
        assert_eq!(
            crownos_background_effect_surface_v1::CrownosBackgroundEffectSurfaceV1::interface()
                .name,
            "crownos_background_effect_surface_v1"
        );
        assert_eq!(
            Capability::all(),
            Capability::Blur | Capability::Shadow | Capability::CornerRadius | Capability::Border
        );
    }

    #[test]
    fn the_screencast_interfaces_are_named_as_expected() {
        use crate::screencast::v1::server::{
            crownos_screencast_manager_v1::{self, CrownosScreencastManagerV1},
            crownos_screencast_session_v1::{self, CrownosScreencastSessionV1},
        };
        use wayland_server::Resource as _;

        assert_eq!(
            CrownosScreencastManagerV1::interface().name,
            "crownos_screencast_manager_v1"
        );
        assert_eq!(
            CrownosScreencastSessionV1::interface().name,
            "crownos_screencast_session_v1"
        );
        assert_eq!(
            crownos_screencast_manager_v1::Capability::all(),
            crownos_screencast_manager_v1::Capability::ExplicitSync
                | crownos_screencast_manager_v1::Capability::ToplevelSource
        );
        assert_eq!(crownos_screencast_session_v1::Ring::MaxSlots as u32, 4);
        assert_eq!(
            crownos_screencast_session_v1::Error::LateTimelines as u32,
            8
        );
        assert_eq!(
            crownos_screencast_session_v1::Error::InvalidReleasePoint as u32,
            9
        );
    }

    #[test]
    fn the_virtual_output_interfaces_are_named_as_expected() {
        use crate::virtual_output::v1::server::{
            crownos_virtual_output_manager_v1::CrownosVirtualOutputManagerV1,
            crownos_virtual_output_v1::CrownosVirtualOutputV1,
        };
        use wayland_server::Resource as _;

        assert_eq!(
            CrownosVirtualOutputManagerV1::interface().name,
            "crownos_virtual_output_manager_v1"
        );
        assert_eq!(
            CrownosVirtualOutputV1::interface().name,
            "crownos_virtual_output_v1"
        );
    }

    #[test]
    fn the_input_interfaces_are_named_as_expected() {
        use crate::input::v1::server::{
            crownos_input_capture_v1::{CrownosInputCaptureV1, Edge},
            crownos_input_injector_v1::CrownosInputInjectorV1,
            crownos_input_manager_v1::CrownosInputManagerV1,
        };
        use wayland_server::Resource as _;

        assert_eq!(
            CrownosInputManagerV1::interface().name,
            "crownos_input_manager_v1"
        );
        assert_eq!(
            CrownosInputInjectorV1::interface().name,
            "crownos_input_injector_v1"
        );
        assert_eq!(
            CrownosInputCaptureV1::interface().name,
            "crownos_input_capture_v1"
        );
        assert_eq!(
            Edge::all(),
            Edge::Top | Edge::Bottom | Edge::Left | Edge::Right
        );
    }
}

#[cfg(all(test, feature = "client"))]
mod client_tests {
    /// The client half generates from the same XML, so an app built against
    /// it speaks exactly the names the compositor binds.
    #[test]
    fn the_client_interfaces_share_the_server_names() {
        use wayland_client::Proxy as _;

        assert_eq!(
            crate::screencast::v1::client::crownos_screencast_session_v1::CrownosScreencastSessionV1::interface().name,
            "crownos_screencast_session_v1"
        );
        assert_eq!(
            crate::virtual_output::v1::client::crownos_virtual_output_v1::CrownosVirtualOutputV1::interface().name,
            "crownos_virtual_output_v1"
        );
        assert_eq!(
            crate::input::v1::client::crownos_input_capture_v1::CrownosInputCaptureV1::interface()
                .name,
            "crownos_input_capture_v1"
        );
    }

    /// Frames and releases each get their own timeline, so neither party
    /// ever signals a timeline the other one signals.
    #[test]
    fn explicit_sync_takes_one_timeline_per_direction() {
        use wayland_client::Proxy as _;

        let set_timelines = crate::screencast::v1::client::crownos_screencast_session_v1::CrownosScreencastSessionV1::interface()
            .requests
            .iter()
            .find(|request| request.name == "set_timelines")
            .map(|request| request.signature.len());
        assert_eq!(set_timelines, Some(2));
    }
}
