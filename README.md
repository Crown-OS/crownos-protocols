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
