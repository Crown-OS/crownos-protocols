//! The scanner invocation, factored out so every protocol module is one line.
//!
//! Taken from `wayland_protocols::protocol_macro`: the `__interfaces` module
//! has to exist alongside the generated code and be in scope for it, which is
//! not something a plain function can express.

macro_rules! crownos_protocol {
    ($path:expr) => {
        #[cfg(feature = "client")]
        pub use self::generated::client;

        #[cfg(feature = "server")]
        pub use self::generated::server;

        mod generated {
            #![allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables)]
            #![allow(non_upper_case_globals, non_snake_case, unused_imports)]
            #![allow(missing_docs, clippy::all)]

            #[cfg(feature = "client")]
            pub mod client {
                //! Client-side API of this protocol.
                use wayland_client;
                use wayland_client::protocol::*;

                pub mod __interfaces {
                    use wayland_client::protocol::__interfaces::*;
                    wayland_scanner::generate_interfaces!($path);
                }
                use self::__interfaces::*;

                wayland_scanner::generate_client_code!($path);
            }

            #[cfg(feature = "server")]
            pub mod server {
                //! Server-side API of this protocol.
                use wayland_server;
                use wayland_server::protocol::*;

                pub mod __interfaces {
                    use wayland_server::protocol::__interfaces::*;
                    wayland_scanner::generate_interfaces!($path);
                }
                use self::__interfaces::*;

                wayland_scanner::generate_server_code!($path);
            }
        }
    };
}
