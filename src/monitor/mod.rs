//! NDM Monitor - Real-time NDM scoring for swarm sessions
//!
//! This module provides real-time NDM monitoring for all Nanoswarm
//! control sessions, with automatic freeze triggers on suspicion.

use serde::{Deserialize, Serialize};
use crate::error::SwarmCtrlError;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// NDM status for swarm session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NDMStatus {
    pub session_id: String,
    pub k_score: f64,
    pub r_score: f64,
    pub e_score: f64,
    pub current_state: String,
    pub suspicion_triggers: Vec<String>,
    pub last_updated: i64,
}

/// NDM monitor for swarm sessions
pub struct NDMMonitor;

impl NDMMonitor {
    /// Get current NDM status for session
    pub fn get_status(session_id: &str) -> Result<NDMStatus, SwarmCtrlError> {
        // In production, query NDM state from sovereigntycore
        // For now, return simulated status
        Ok(NDMStatus {
            session_id: session_id.to_string(),
            k_score: 0.2,
            r_score: 0.3,
            e_score: 0.1,
            current_state: "Normal".to_string(),
            suspicion_triggers: vec![],
            last_updated: Utc::now().timestamp(),
        })
    }

    /// Increment NDM score for suspicion trigger
    pub fn increment_suspicion(
        session_id: &str,
        trigger: &str,
        increment: f64,
    ) -> Result<NDMStatus, SwarmCtrlError> {
        // In production, update NDM state and check thresholds
        let mut status = Self::get_status(session_id)?;
        status.k_score = (status.k_score + increment).min(1.0);
        status.suspicion_triggers.push(trigger.to_string());
        status.last_updated = Utc::now().timestamp();
        
        Ok(status)
    }

    /// Check if session should be frozen
    pub fn should_freeze(status: &NDMStatus, threshold: f64) -> bool {
        status.k_score >= threshold
    }

    /// Get suspicion trigger definitions
    pub fn get_suspicion_triggers() -> Vec<SuspicionTrigger> {
        vec![
            SuspicionTrigger {
                trigger_id: "unauthorized_did_session".to_string(),
                trigger_type: "auth".to_string(),
                severity: 0.3,
                ndm_increment: 0.15,
                description: "Session initiated from unauthorized DID".to_string(),
            },
            SuspicionTrigger {
                trigger_id: "unusual_swarm_command_sequence".to_string(),
                trigger_type: "swarm".to_string(),
                severity: 0.5,
                ndm_increment: 0.25,
                description: "Anomalous Nanoswarm command pattern detected".to_string(),
            },
            SuspicionTrigger {
                trigger_id: "capability_escalation_attempt".to_string(),
                trigger_type: "capability".to_string(),
                severity: 0.6,
                ndm_increment: 0.3,
                description: "Attempt to escalate capabilities beyond grant".to_string(),
            },
            SuspicionTrigger {
                trigger_id: "weaponization_pattern_detected".to_string(),
                trigger_type: "weaponization".to_string(),
                severity: 0.9,
                ndm_increment: 0.5,
                description: "Pattern matching weaponization signature library".to_string(),
            },
        ]
    }
}

/// Suspicion trigger definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspicionTrigger {
    pub trigger_id: String,
    pub trigger_type: String,
    pub severity: f64,
    pub ndm_increment: f64,
    pub description: String,
}

/// NDM threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NDMThresholds {
    pub normal_ceiling: f64,
    pub monitoring_ceiling: f64,
    pub degrade_ceiling: f64,
    pub auto_freeze_threshold: f64,
    pub multisig_threshold: f64,
    pub quarantine_ceiling: f64,
}

impl Default for NDMThresholds {
    fn default() -> Self {
        Self {
            normal_ceiling: 0.3,
            monitoring_ceiling: 0.6,
            degrade_ceiling: 0.8,
            auto_freeze_threshold: 0.7,
            multisig_threshold: 0.5,
            quarantine_ceiling: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ndm_status_retrieval() {
        let status = NDMMonitor::get_status("session-123");
        assert!(status.is_ok());
        assert!(status.unwrap().k_score >= 0.0);
    }

    #[test]
    fn test_freeze_threshold_check() {
        let status = NDMStatus {
            session_id: "session-123".to_string(),
            k_score: 0.8,
            r_score: 0.3,
            e_score: 0.1,
            current_state: "Freeze".to_string(),
            suspicion_triggers: vec![],
            last_updated: Utc::now().timestamp(),
        };

        assert!(NDMMonitor::should_freeze(&status, 0.7));
        assert!(!NDMMonitor::should_freeze(&status, 0.9));
    }
}
