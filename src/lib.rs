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

pub mod agent_access;
pub mod agent_cursor;
pub mod app_icons;
pub mod background_effects;
pub mod capture_privacy;
pub mod context;
pub mod guest_seat;
pub mod input;
pub mod keybind;
pub mod output_brightness;
pub mod output_info;
pub mod plugin_host;
pub mod screencast;
pub mod secret_input;
pub mod session;
pub mod surface_animation;
pub mod surface_visibility;
pub mod virtual_output;
pub mod window_control;
pub mod workspace_mode;

#[cfg(all(test, feature = "server"))]
mod tests {
    use super::background_effects::v1::server::*;

    /// The secret input global and its enums, as crownos-secretsd and crownpositor name them.
    #[test]
    fn the_secret_input_names_are_stable() {
        use super::secret_input::v1::server::crownos_secret_consent_v1::{
            Action, CrownosSecretConsentV1,
        };
        use super::secret_input::v1::server::crownos_secret_input_manager_v1::CrownosSecretInputManagerV1;
        use super::secret_input::v1::server::crownos_secret_input_request_v1::{
            CrownosSecretInputRequestV1, Kind, Outcome,
        };
        use wayland_server::Resource as _;

        assert_eq!(
            CrownosSecretConsentV1::interface().name,
            "crownos_secret_consent_v1"
        );
        assert_eq!(Action::Proxy as u32, 1);
        assert_eq!(
            CrownosSecretInputManagerV1::interface().name,
            "crownos_secret_input_manager_v1"
        );
        assert_eq!(
            CrownosSecretInputRequestV1::interface().name,
            "crownos_secret_input_request_v1"
        );
        assert_eq!(Kind::Password as u32, 1);
        assert_eq!(Outcome::NotPasswordField as u32, 2);
        assert_eq!(Outcome::SessionLocked as u32, 8);
    }

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
            Capability::Blur
                | Capability::Shadow
                | Capability::CornerRadius
                | Capability::Border
                | Capability::CompositedBlur
        );
    }

    /// A child object takes its manager's version, so both effect interfaces
    /// move together: a version 2 surface object is how the compositor tells
    /// a client that knows about `composited_blur` from one that does not.
    #[test]
    fn the_effect_interfaces_share_a_version() {
        use wayland_server::Resource as _;

        assert_eq!(
            crownos_background_effect_manager_v1::CrownosBackgroundEffectManagerV1::interface()
                .version,
            2
        );
        assert_eq!(
            crownos_background_effect_surface_v1::CrownosBackgroundEffectSurfaceV1::interface()
                .version,
            2
        );
        assert_eq!(
            crownos_background_effect_manager_v1::Capability::CompositedBlur.bits(),
            16
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
                | crownos_screencast_manager_v1::Capability::WorkspaceSource
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

    #[test]
    fn the_surface_visibility_interfaces_are_named_as_expected() {
        use crate::surface_visibility::v1::server::{
            crownos_surface_visibility_manager_v1::CrownosSurfaceVisibilityManagerV1,
            crownos_surface_visibility_v1::{CrownosSurfaceVisibilityV1, State},
        };
        use wayland_server::Resource as _;

        assert_eq!(
            CrownosSurfaceVisibilityManagerV1::interface().name,
            "crownos_surface_visibility_manager_v1"
        );
        assert_eq!(
            CrownosSurfaceVisibilityV1::interface().name,
            "crownos_surface_visibility_v1"
        );
        assert_eq!(State::Visible as u32, 0);
        assert_eq!(State::Occluded as u32, 1);
        assert_eq!(State::Hidden as u32, 2);
    }

    #[test]
    fn the_agent_access_interfaces_are_named_as_expected() {
        use crate::agent_access::v1::server::{
            crownos_agent_access_manager_v1::CrownosAgentAccessManagerV1,
            crownos_agent_access_v1::CrownosAgentAccessV1,
        };
        use wayland_server::Resource as _;

        assert_eq!(
            CrownosAgentAccessManagerV1::interface().name,
            "crownos_agent_access_manager_v1"
        );
        assert_eq!(
            CrownosAgentAccessV1::interface().name,
            "crownos_agent_access_v1"
        );
    }

    #[test]
    fn the_surface_animation_interfaces_are_named_as_expected() {
        use crate::surface_animation::v1::server::{
            crownos_animated_surface_v1::{CrownosAnimatedSurfaceV1, Property},
            crownos_surface_animation_manager_v1::CrownosSurfaceAnimationManagerV1,
        };
        use wayland_server::Resource as _;

        assert_eq!(
            CrownosSurfaceAnimationManagerV1::interface().name,
            "crownos_surface_animation_manager_v1"
        );
        assert_eq!(
            CrownosAnimatedSurfaceV1::interface().name,
            "crownos_animated_surface_v1"
        );
        assert_eq!(Property::TranslateX as u32, 0);
        assert_eq!(Property::Opacity as u32, 4);
    }
    #[test]
    fn the_workspace_mode_interface_is_named_as_expected() {
        use crate::workspace_mode::v1::server::crownos_workspace_mode_manager_v1::{
            CrownosWorkspaceModeManagerV1, Error, Mode,
        };
        use wayland_server::Resource as _;

        assert_eq!(
            CrownosWorkspaceModeManagerV1::interface().name,
            "crownos_workspace_mode_manager_v1"
        );
        assert_eq!(Mode::Tiling as u32, 0);
        assert_eq!(Mode::Floating as u32, 1);
        assert_eq!(Error::InvalidMode as u32, 0);
    }
    /// The output object of version 2 takes the manager's version, so a
    /// client that binds version 1 can never send get_output.
    #[test]
    fn the_workspace_mode_output_interface_is_version_2() {
        use crate::workspace_mode::v1::server::{
            crownos_workspace_mode_manager_v1::CrownosWorkspaceModeManagerV1,
            crownos_workspace_mode_output_v1::{CrownosWorkspaceModeOutputV1, Error},
        };
        use wayland_server::Resource as _;

        assert_eq!(CrownosWorkspaceModeManagerV1::interface().version, 2);
        assert_eq!(
            CrownosWorkspaceModeOutputV1::interface().name,
            "crownos_workspace_mode_output_v1"
        );
        assert_eq!(CrownosWorkspaceModeOutputV1::interface().version, 2);
        assert_eq!(Error::InvalidMode as u32, 0);
    }

    /// A window object takes the version of the manager that announced it,
    /// so both window control interfaces move together.
    #[test]
    fn the_window_control_interfaces_are_version_2() {
        use crate::window_control::v1::server::{
            crownos_window_control_manager_v1::CrownosWindowControlManagerV1,
            crownos_window_v1::CrownosWindowV1,
        };
        use wayland_server::Resource as _;

        assert_eq!(CrownosWindowControlManagerV1::interface().version, 2);
        assert_eq!(CrownosWindowV1::interface().version, 2);
    }

    #[test]
    fn the_output_info_interfaces_are_named_as_expected() {
        use crate::output_info::v1::server::{
            crownos_output_info_manager_v1::CrownosOutputInfoManagerV1,
            crownos_output_info_v1::{Capability, CrownosOutputInfoV1, Shell, State},
        };
        use wayland_server::Resource as _;

        assert_eq!(
            CrownosOutputInfoManagerV1::interface().name,
            "crownos_output_info_manager_v1"
        );
        assert_eq!(
            CrownosOutputInfoV1::interface().name,
            "crownos_output_info_v1"
        );
        assert_eq!(CrownosOutputInfoManagerV1::interface().version, 1);
        assert_eq!(CrownosOutputInfoV1::interface().version, 1);
        assert_eq!(
            Shell::all(),
            Shell::Bar | Shell::Dock | Shell::Notifications
        );
        assert_eq!(Shell::Notifications.bits(), 4);
        assert_eq!(State::all(), State::Primary | State::Focused);
        assert_eq!(Capability::all(), Capability::Hdr | Capability::Virtual);
        assert_eq!(Capability::Virtual.bits(), 2);
    }

    #[test]
    fn the_output_brightness_interfaces_are_named_as_expected() {
        use crate::output_brightness::v1::server::{
            crownos_output_brightness_manager_v1::CrownosOutputBrightnessManagerV1,
            crownos_output_brightness_v1::{Control, CrownosOutputBrightnessV1, Error},
        };
        use wayland_server::Resource as _;

        assert_eq!(
            CrownosOutputBrightnessManagerV1::interface().name,
            "crownos_output_brightness_manager_v1"
        );
        assert_eq!(
            CrownosOutputBrightnessV1::interface().name,
            "crownos_output_brightness_v1"
        );
        assert_eq!(CrownosOutputBrightnessManagerV1::interface().version, 1);
        assert_eq!(CrownosOutputBrightnessV1::interface().version, 1);
        assert_eq!(Control::None as u32, 0);
        assert_eq!(Control::Backlight as u32, 1);
        assert_eq!(Control::Ddc as u32, 2);
        assert_eq!(Control::Software as u32, 3);
        assert_eq!(Error::InvalidLevel as u32, 0);
    }

    #[test]
    fn the_keybind_interfaces_are_named_as_expected() {
        use crate::keybind::v1::server::{
            crownos_keybind_manager_v1::{CrownosKeybindManagerV1, Error},
            crownos_keybind_v1::CrownosKeybindV1,
        };
        use wayland_server::Resource as _;

        assert_eq!(
            CrownosKeybindManagerV1::interface().name,
            "crownos_keybind_manager_v1"
        );
        assert_eq!(CrownosKeybindV1::interface().name, "crownos_keybind_v1");
        assert_eq!(CrownosKeybindManagerV1::interface().version, 1);
        assert_eq!(CrownosKeybindV1::interface().version, 1);
        assert_eq!(Error::InvalidIdentity as u32, 0);
    }

    #[test]
    fn the_agent_platform_interfaces_are_named_as_expected() {
        use crate::agent_cursor::v1::server::{
            crownos_agent_cursor_manager_v1::CrownosAgentCursorManagerV1,
            crownos_agent_cursor_v1::{CrownosAgentCursorV1, RevokeReason},
        };
        use crate::capture_privacy::v1::server::crownos_surface_privacy_v1::CrownosSurfacePrivacyV1;
        use crate::context::v1::server::{
            crownos_context_manager_v1::CrownosContextManagerV1,
            crownos_context_window_v1::{CrownosContextWindowV1, Sensitivity},
        };
        use crate::window_control::v1::server::{
            crownos_window_control_manager_v1::CrownosWindowControlManagerV1,
            crownos_window_v1::{CrownosWindowV1, State},
        };
        use wayland_server::Resource as _;

        assert_eq!(
            CrownosWindowControlManagerV1::interface().name,
            "crownos_window_control_manager_v1"
        );
        assert_eq!(CrownosWindowV1::interface().name, "crownos_window_v1");
        assert_eq!(State::AgentWorkspace.bits(), 32);
        assert_eq!(
            CrownosAgentCursorManagerV1::interface().name,
            "crownos_agent_cursor_manager_v1"
        );
        assert_eq!(
            CrownosAgentCursorV1::interface().name,
            "crownos_agent_cursor_v1"
        );
        assert_eq!(RevokeReason::Locked as u32, 2);
        assert_eq!(
            crate::guest_seat::v1::server::crownos_guest_seat_manager_v1::CrownosGuestSeatManagerV1::interface().name,
            "crownos_guest_seat_manager_v1"
        );
        assert_eq!(
            CrownosContextManagerV1::interface().name,
            "crownos_context_manager_v1"
        );
        assert_eq!(
            CrownosContextWindowV1::interface().name,
            "crownos_context_window_v1"
        );
        assert_eq!(Sensitivity::EngineFlagged.bits(), 16);
        assert_eq!(
            CrownosSurfacePrivacyV1::interface().name,
            "crownos_surface_privacy_v1"
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

        assert_eq!(
            crate::surface_visibility::v1::client::crownos_surface_visibility_v1::CrownosSurfaceVisibilityV1::interface().name,
            "crownos_surface_visibility_v1"
        );
        assert_eq!(
            crate::agent_access::v1::client::crownos_agent_access_v1::CrownosAgentAccessV1::interface().name,
            "crownos_agent_access_v1"
        );
        assert_eq!(
            crate::workspace_mode::v1::client::crownos_workspace_mode_manager_v1::CrownosWorkspaceModeManagerV1::interface().name,
            "crownos_workspace_mode_manager_v1"
        );
        assert_eq!(
            crate::workspace_mode::v1::client::crownos_workspace_mode_output_v1::CrownosWorkspaceModeOutputV1::interface().name,
            "crownos_workspace_mode_output_v1"
        );
        assert_eq!(
            crate::output_info::v1::client::crownos_output_info_v1::CrownosOutputInfoV1::interface(
            )
            .name,
            "crownos_output_info_v1"
        );
        assert_eq!(
            crate::output_brightness::v1::client::crownos_output_brightness_v1::CrownosOutputBrightnessV1::interface().name,
            "crownos_output_brightness_v1"
        );
        assert_eq!(
            crate::keybind::v1::client::crownos_keybind_v1::CrownosKeybindV1::interface().name,
            "crownos_keybind_v1"
        );
    }

    /// Version 2 requests come after every version 1 request, so the
    /// opcodes a version 1 client sends keep their meaning.
    #[test]
    fn the_version_2_requests_take_new_opcodes() {
        use wayland_client::Proxy as _;

        let opcode_of = |requests: &[wayland_backend::protocol::MessageDesc], name: &str| {
            requests.iter().position(|request| request.name == name)
        };
        let window_requests =
            crate::window_control::v1::client::crownos_window_v1::CrownosWindowV1::interface()
                .requests;
        assert_eq!(opcode_of(window_requests, "set_rectangle"), Some(6));
        assert_eq!(opcode_of(window_requests, "move_to_output"), Some(7));

        let manager_requests = crate::workspace_mode::v1::client::crownos_workspace_mode_manager_v1::CrownosWorkspaceModeManagerV1::interface()
            .requests;
        assert_eq!(opcode_of(manager_requests, "set_mode"), Some(1));
        assert_eq!(opcode_of(manager_requests, "get_output"), Some(2));
    }

    /// The context events of version 2 come after every version 1 event,
    /// so a version 1 client decodes the same opcodes as before.
    #[test]
    fn the_context_version_2_events_take_new_opcodes() {
        use crate::context::v1::client::{
            crownos_context_manager_v1::{self, CrownosContextManagerV1, RevokeReason, Scope},
            crownos_context_window_v1::CrownosContextWindowV1,
        };
        use wayland_client::Proxy as _;

        let manager = CrownosContextManagerV1::interface();
        assert_eq!(manager.version, 2);
        assert_eq!(CrownosContextWindowV1::interface().version, 2);
        let event_names: Vec<_> = manager.events.iter().map(|event| event.name).collect();
        assert_eq!(
            event_names,
            [
                "window",
                "focus",
                "session",
                "input_activity",
                "selection",
                "paused",
                "done",
                "scope",
                "revoked"
            ]
        );
        assert_eq!(crownos_context_manager_v1::EVT_SCOPE_SINCE, 2);
        assert_eq!(crownos_context_manager_v1::EVT_REVOKED_SINCE, 2);
        assert_eq!(Scope::Metadata.bits(), 1);
        assert_eq!(Scope::Text.bits(), 2);
        assert_eq!(Scope::Raw.bits(), 4);
        assert_eq!(Scope::Engine.bits(), 8);
        assert_eq!(Scope::Pause.bits(), 16);
        assert_eq!(crownos_context_manager_v1::PauseOrigin::Client as u32, 3);
        assert_eq!(RevokeReason::Expired as u32, 1);
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
