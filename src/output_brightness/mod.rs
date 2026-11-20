//! `crownos_output_brightness_v1`: the brightness of each output, through its
//! backlight, DDC/CI or a dim in the compositor.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-output-brightness-v1.xml");
}
