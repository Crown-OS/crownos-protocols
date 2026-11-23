# crownos-protocols

Bindings for the Wayland protocol extensions CrownOS defines for itself.

The XML under `protocols/` is the protocol; everything under `src/` is generated
from it at compile time by `wayland-scanner`, the same way `wayland-protocols`
does it — so the description and the code cannot drift.

```toml
crownos-protocols = { version = "0.1", features = ["server"] }  # compositors
crownos-protocols = { version = "0.1", features = ["client"] }  # apps
```

## Protocols

| Protocol | Interfaces | What it does |
| --- | --- | --- |
| `crownos_background_effects` | `crownos_shape_manager_v1`, `crownos_shape_v1`, `crownos_background_effect_manager_v1`, `crownos_background_effect_surface_v1` | Server-side material behind and around a surface: a blurred, tinted and saturated backdrop under a parametric shape, a refractive rim along its edge, a corner radius, and a drop shadow. |
| `crownos_screencast_v1` | `crownos_screencast_manager_v1`, `crownos_screencast_session_v1` | Zero-copy capture of an output into a ring of client-owned dmabufs (RGB or GPU-converted NV12), rendered only on damage or `force_frame` and paced by the client releasing slots; optional explicit sync through two DRM syncobj timelines, one per direction; cursor embedded, hidden or sent as metadata. Privileged. |
| `crownos_virtual_output_v1` | `crownos_virtual_output_manager_v1`, `crownos_virtual_output_v1` | Outputs with no display behind them — a tablet as a second monitor, a headless remote desktop — that are ordinary heads to the rest of the session and whose frame clock is driven by the client's `request_frame`. Privileged. |
| `crownos_input_v1` | `crownos_input_manager_v1`, `crownos_input_injector_v1`, `crownos_input_capture_v1` | Injecting pointer, keyboard and touch events into a seat through the normal input path, and capturing a seat's input once the pointer is pushed through an armed edge of the layout (shared keyboard and mouse between machines). Privileged. |
| `crownos_app_icons_v1` | `crownos_app_icon_manager_v1`, `crownos_app_icon_v1` | Where a launcher such as the dock shows each application's icon, so the compositor grows the application's windows out of it and shrinks them back into it; activating an icon restores and focuses the application's windows, or asks the launcher to start it with `launch`. Privileged. |
| `crownos_surface_animation_v1` | `crownos_surface_animation_manager_v1`, `crownos_animated_surface_v1` | The compositor runs closed-form spring animations of a subsurface's translation, scale, rotation and opacity from a client-supplied CLOCK_MONOTONIC start time, so the app does not render every frame; `settled` and `interrupted` report where motion stopped or was retargeted. |
| `crownos_surface_visibility_v1` | `crownos_surface_visibility_manager_v1`, `crownos_surface_visibility_v1` | Tells a client whether a surface is visible, occluded or hidden, and the scale it is drawn at while shown as a thumbnail (an overview), so it can throttle or lower detail. Changes are grouped by `done` and coalesced per scene update; animated values are reported where they settle. |
| `crownos_agent_access_v1` | `crownos_agent_access_manager_v1`, `crownos_agent_access_v1` | An app asks the compositor for permission to expose its UI to AI agents (over MCP); the answer is a grant with an opaque token, a denial, or a later revocation. Privileged. |
| `crownos_workspace_mode_v1` | `crownos_workspace_mode_manager_v1`, `crownos_workspace_mode_output_v1` | Reports whether the focused workspace tiles its windows or lets them float, on bind and on every change (including focus moving to a workspace in the other mode), and switches it with `set_mode`. From version 2, `get_output` does the same for the active workspace of one output. Privileged. |
| `crownos_window_control_v1` | `crownos_window_control_manager_v1`, `crownos_window_v1` | The window list for the agent daemon: identifier shared with `ext_foreign_toplevel_handle_v1`, app id, title, owning pid, placement and state, the focus, and requests to focus, close, move between workspaces, send to or fetch from the agent workspace and place floating windows; from version 2, move a window to another output. Agent daemon only. |
| `crownos_guest_seat_v1` | `crownos_guest_seat_manager_v1`, `crownos_guest_seat_v1` | A seat of a remote desktop guest's own, confined to the output their session shows, taking injector-style frames, so a guest works without taking the user's pointer or keyboard focus. crownlink only. |
| `crownos_agent_cursor_v1` | `crownos_agent_cursor_manager_v1`, `crownos_agent_cursor_v1` | A seat of an agent's own: a coloured, named cursor and a keyboard focus confined to the agent workspace, taking injector-style frames, so agents work beside the user and beside each other. Agent daemon only. |
| `crownos_context_v1` | `crownos_context_manager_v1`, `crownos_context_window_v1` | What the user is doing, without content, for the visual context engine: windows with process, sandbox, placement, state, content type and sensitivity, the focus, session state, input activity as counts, selection metadata, and a pause. Version 2: many clients, each with a scope from the user's grants, which can be revoked. |
| `crownos_output_info_v1` | `crownos_output_info_manager_v1`, `crownos_output_info_v1` | How the compositor treats each output, so shell apps never match output names against the settings: the name the settings use for it, the shell surfaces it shows (bar, dock, notifications, defaults applied), whether it is primary or focused, whether its display claims HDR or is virtual, and the bits per channel it is scanned out at. Grouped by `done`. Privileged. |
| `crownos_output_brightness_v1` | `crownos_output_brightness_manager_v1`, `crownos_output_brightness_v1` | The brightness of each output on one perceptual scale from 0 to 10000, whether the compositor drives a backlight, DDC/CI or a software dim, with `none` for an output whose level cannot change; `set_level` is clamped and coalesced, `refresh` reads the hardware again. Privileged. |
| `crownos_capture_privacy_v1` | `crownos_capture_privacy_manager_v1`, `crownos_surface_privacy_v1` | An application keeps a surface of its own out of captures. Any client. |
| `crownos_secret_input_v1` | `crownos_secret_input_manager_v1` | The compositor types a stored secret into a checked field after its own confirmation. Only crownos-secretsd. |

## C bindings

The XML is plain `wayland-scanner` input, so a C client or compositor needs no
Rust at all:

```sh
wayland-scanner --strict client-header \
    protocols/crownos-background-effects-v1.xml \
    crownos-background-effects-v1-client-protocol.h
wayland-scanner --strict server-header \
    protocols/crownos-background-effects-v1.xml \
    crownos-background-effects-v1-server-protocol.h
wayland-scanner --strict private-code \
    protocols/crownos-background-effects-v1.xml \
    crownos-background-effects-v1-protocol.c
```
