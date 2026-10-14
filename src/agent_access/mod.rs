//! `crownos_agent_access_v1`: asking the compositor for permission to expose
//! an application's UI to AI agents.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-agent-access-v1.xml");
}
