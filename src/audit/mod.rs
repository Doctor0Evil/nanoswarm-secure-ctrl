//! Audit Logger - Command history logging to ROW/RPM
//!
//! This module logs all Nanoswarm commands and freeze events
//! to the ROW/RPM ledger with Cyberspectre trace integration.

use serde::{Deserialize, Serialize};
use crate::types::SwarmCommand;
use crate::error::SwarmCtrlError;
use uuid::Uuid;
use chrono::Utc;

/// Audit logger for swarm operations
pub struct AuditLogger;

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub entry_id: String,
    pub entry_type: String,
    pub timestamp: i64,
    pub session_id: String,
    pub operator_dids: Vec<String>,
    pub command: Option<SwarmCommand>,
    pub freeze_reason: Option<String>,
    pub trace_id: String,
    pub row_id: Option<String>,
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new() -> Self {
        Self
    }

    /// Log a swarm command
    ///
    /// # Arguments
    ///
    /// * `command` - Swarm command executed
    /// * `operator_dids` - DIDs of operators who authorized
    /// * `session_id` - Session identifier
    /// * `trace_id` - Cyberspectre trace ID
    ///
    /// # Returns
    ///
    /// * `String` - ROW ID for the log entry
    pub fn log_command(
        &self,
        command: &SwarmCommand,
        operator_dids: &[&str],
        session_id: &str,
        trace_id: &str,
    ) -> Result<String, SwarmCtrlError> {
        let entry = AuditEntry {
            entry_id: Uuid::new_v4().to_string(),
            entry_type: "swarm_command".to_string(),
            timestamp: Utc::now().timestamp(),
            session_id: session_id.to_string(),
            operator_dids: operator_dids.iter().map(|s| s.to_string()).collect(),
            command: Some(command.clone()),
            freeze_reason: None,
            trace_id: trace_id.to_string(),
            row_id: Some(Uuid::new_v4().to_string()),
        };

        // In production, write to ROW/RPM ledger
        log::info!("Audit log entry: {}", entry.entry_id);
        
        Ok(entry.row_id.unwrap())
    }

    /// Log a freeze event
    ///
    /// # Arguments
    ///
    /// * `reason` - Reason for freeze
    /// * `initiator_did` - DID of freeze initiator
    ///
    /// # Returns
    ///
    /// * `String` - ROW ID for the log entry
    pub fn log_freeze(&self, reason: &str, initiator_did: &str) -> Result<String, SwarmCtrlError> {
        let entry = AuditEntry {
            entry_id: Uuid::new_v4().to_string(),
            entry_type: "swarm_freeze".to_string(),
            timestamp: Utc::now().timestamp(),
            session_id: "all_sessions".to_string(),
            operator_dids: vec![initiator_did.to_string()],
            command: None,
            freeze_reason: Some(reason.to_string()),
            trace_id: Uuid::new_v4().to_string(),
            row_id: Some(Uuid::new_v4().to_string()),
        };

        // In production, write to ROW/RPM ledger
        log::info!("Freeze audit log entry: {}", entry.entry_id);
        
        Ok(entry.row_id.unwrap())
    }

    /// Query audit log entries
    pub fn query_entries(session_id: Option<&str>) -> Result<Vec<AuditEntry>, SwarmCtrlError> {
        // In production, query from ROW/RPM ledger
        Ok(vec![])
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MissionClass;

    #[test]
    fn test_command_logging() {
        let logger = AuditLogger::new();
        let command = SwarmCommand::new(
            MissionClass::EcologicalRestoration,
            "canal_sector_7".to_string(),
        );

        let row_id = logger.log_command(
            &command,
            &["bostrom1op1", "bostrom1op2"],
            "session-123",
            "trace-456",
        );

        assert!(row_id.is_ok());
        assert!(!row_id.unwrap().is_empty());
    }

    #[test]
    fn test_freeze_logging() {
        let logger = AuditLogger::new();
        let row_id = logger.log_freeze("test_reason", "bostrom1initiator");
        assert!(row_id.is_ok());
    }
}
