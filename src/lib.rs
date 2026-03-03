//! Nanoswarm Secure Control - Hardened swarm control with NDM integration
//!
//! This crate provides the only authorized control interface for Nanoswarm
//! operations, with capability gates, NDM monitoring, and non-weaponization
//! enforcement.
//!
//! # Architecture
//!
//! ```text
//! Operator → CommandController → NDM Monitor → DID Whitelist → Nanoswarm Nodes
//!                              ↓
//!                      Audit Logger → ROW/RPM + Cyberspectre
//! ```
//!
//! # Example
//!
//! ```rust
//! use nanoswarm_secure_ctrl::{SwarmController, SwarmCommand, MissionClass};
//!
//! let mut controller = SwarmController::new(config)?;
//!
//! let command = SwarmCommand::new(
//!     MissionClass::EcologicalRestoration,
//!     "canal_sector_7".to_string(),
//! );
//!
//! let result = controller.issue_command(command, &operator_dids)?;
//!
//! // Result includes ROW ID and Cyberspectre trace ID
//! assert!(result.row_id.is_some());
//! assert!(result.trace_id.is_some());
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

pub mod controller;
pub mod monitor;
pub mod freeze;
pub mod audit;
pub mod whitelist;
pub mod capabilities;
pub mod error;
pub mod types;
pub mod hex_stamp;

/// Crate version
pub const VERSION: &str = "1.0.0";

/// Hex-stamp attestation for this release
pub const HEX_STAMP: &str = "0xbe7f3e6d5c2b8a0f9e4d3c2b1a0f9e8d7c6b5a49f8e7d6c5b4a3928170f6e5d4";

/// Ledger reference for this release
pub const LEDGER_REF: &str = "row:nanoswarm-secure-ctrl:v1.0.0:2026-03-04";

/// Re-export commonly used types
pub use controller::SwarmController;
pub use types::{SwarmCommand, MissionClass, CommandResult};
pub use error::SwarmCtrlError;
pub use capabilities::NonWeaponEnvelope;

/// Issue a swarm command with full security checks
///
/// # Arguments
///
/// * `command` - Swarm command to issue
/// * `operator_dids` - List of operator DIDs for multi-sig
/// * `session_id` - Current session identifier
///
/// # Returns
///
/// * `CommandResult` - Result with ROW ID and Cyberspectre trace ID
pub fn issue_swarm_command(
    command: SwarmCommand,
    operator_dids: &[&str],
    session_id: &str,
) -> Result<CommandResult, SwarmCtrlError> {
    let config = controller::ControllerConfig::default();
    let mut controller = SwarmController::new(config)?;
    
    controller.issue_command(command, operator_dids, session_id)
}

/// Emergency freeze all swarm operations
///
/// # Arguments
///
/// * `reason` - Reason for freeze (logged to ROW/RPM)
/// * `initiator_did` - DID of freeze initiator
///
/// # Returns
///
/// * `FreezeResult` - Confirmation with ROW ID
pub fn emergency_freeze(
    reason: &str,
    initiator_did: &str,
) -> Result<freeze::FreezeResult, SwarmCtrlError> {
    freeze::FreezeProtocol::execute(reason, initiator_did)
}

/// Get current NDM status for swarm session
///
/// # Arguments
///
/// * `session_id` - Session identifier
///
/// # Returns
///
/// * `NDMStatus` - Current NDM score and state
pub fn get_ndm_status(session_id: &str) -> Result<monitor::NDMStatus, SwarmCtrlError> {
    monitor::NDMMonitor::get_status(session_id)
}

/// Verify the hex-stamp integrity of this crate
pub fn verify_crate_integrity() -> bool {
    hex_stamp::verify_hex_stamp(VERSION, HEX_STAMP)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_version() {
        assert_eq!(VERSION, "1.0.0");
    }

    #[test]
    fn test_hex_stamp_format() {
        assert!(HEX_STAMP.starts_with("0x"));
        assert_eq!(HEX_STAMP.len(), 66);
    }

    #[test]
    fn test_emergency_freeze() {
        let result = emergency_freeze("test_freeze", "bostrom1test");
        assert!(result.is_ok());
    }
}
