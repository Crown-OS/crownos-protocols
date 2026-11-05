//! `crownos-android-shell-v1`: Android task details for the xdg_toplevels of
//! the CrownOS Android runtime.

pub mod v1 {
    crownos_protocol!("./protocols/crownos-android-shell-v1.xml", xdg_shell);
}
