//! `crownos_agent_cursor_v1`: a seat, cursor and keyboard focus of an agent's own, confined to the agent workspace.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-agent-cursor-v1.xml");
}
