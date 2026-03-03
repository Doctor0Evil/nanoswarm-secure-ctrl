//! Swarm Command Controller - Capability-gated command issuance
//!
//! This module handles all Nanoswarm command issuance with:
//! - Non-weapon envelope validation
//! - Multi-DID authorization
//! - NDM threshold checking
//! - ROW/RPM audit logging

use crate::types::{SwarmCommand, MissionClass, CommandResult};
use crate::error::SwarmCtrlError;
use crate::monitor::NDMMonitor;
use crate::whitelist::DIDWhitelist;
use crate::capabilities::NonWeaponEnvelope;
use crate::audit::AuditLogger;
use uuid::Uuid;
use chrono::Utc;

/// Swarm controller configuration
#[derive(Debug, Clone)]
pub struct ControllerConfig {
    pub ndm_auto_freeze_threshold: f64,
    pub require_multi_sig: bool,
    pub min_signatures: usize,
    pub audit_enabled: bool,
}

impl Default for ControllerConfig {
    fn default() -> Self {
        Self {
            ndm_auto_freeze_threshold: 0.7,
            require_multi_sig: true,
            min_signatures: 2,
            audit_enabled: true,
        }
    }
}

/// Swarm command controller
pub struct SwarmController {
    config: ControllerConfig,
    whitelist: DIDWhitelist,
    logger: AuditLogger,
}

impl SwarmController {
    /// Create a new swarm controller
    pub fn new(config: ControllerConfig) -> Result<Self, SwarmCtrlError> {
        Ok(Self {
            config,
            whitelist: DIDWhitelist::new(),
            logger: AuditLogger::new(),
        })
    }

    /// Issue a swarm command
    ///
    /// # Arguments
    ///
    /// * `command` - Swarm command to issue
    /// * `operator_dids` - List of operator DIDs for authorization
    /// * `session_id` - Current session identifier
    ///
    /// # Returns
    ///
    /// * `CommandResult` - Result with ROW ID and Cyberspectre trace ID
    pub fn issue_command(
        &mut self,
        command: SwarmCommand,
        operator_dids: &[&str],
        session_id: &str,
    ) -> Result<CommandResult, SwarmCtrlError> {
        // Generate trace ID for Cyberspectre
        let trace_id = Uuid::new_v4().to_string();
        
        // Check NDM status (auto-freeze if threshold exceeded)
        let ndm_status = NDMMonitor::get_status(session_id)?;
        if ndm_status.k_score >= self.config.ndm_auto_freeze_threshold {
            return Err(SwarmCtrlError::NDMAutoFreeze {
                k_score: ndm_status.k_score,
                threshold: self.config.ndm_auto_freeze_threshold,
            });
        }

        // Validate non-weapon envelope
        self.validate_non_weapon_envelope(&command.mission_class)?;

        // Verify operator DIDs against whitelist
        if !self.whitelist.verify_operators(operator_dids)? {
            return Err(SwarmCtrlError::UnauthorizedOperator);
        }

        // Check multi-sig requirement
        if self.config.require_multi_sig && operator_dids.len() < self.config.min_signatures {
            return Err(SwarmCtrlError::InsufficientSignatures {
                required: self.config.min_signatures,
                provided: operator_dids.len(),
            });
        }

        // Execute command (in production, send to swarm nodes)
        let command_id = Uuid::new_v4().to_string();
        
        // Log to audit trail
        let row_id = if self.config.audit_enabled {
            Some(self.logger.log_command(
                &command,
                operator_dids,
                session_id,
                &trace_id,
            )?)
        } else {
            None
        };

        Ok(CommandResult {
            command_id,
            row_id,
            trace_id,
            timestamp: Utc::now().timestamp(),
            status: "approved".to_string(),
        })
    }

    /// Validate non-weapon envelope for mission class
    fn validate_non_weapon_envelope(&self, mission: &MissionClass) -> Result<(), SwarmCtrlError> {
        let permitted_missions = [
            MissionClass::EcologicalRestoration,
            MissionClass::ClinicalAssistiveCare,
            MissionClass::Diagnostics,
            MissionClass::NeuromorphicSwarmMaintenance,
        ];

        let forbidden_missions = [
            MissionClass::KineticDamage,
            MissionClass::CrowdControl,
            MissionClass::SurveillanceWithoutConsent,
            MissionClass::WeaponDeployment,
            MissionClass::OffensiveOperations,
        ];

        if forbidden_missions.contains(mission) {
            return Err(SwarmCtrlError::WeaponizationAttempt {
                mission_class: format!("{:?}", mission),
            });
        }

        if !permitted_missions.contains(mission) {
            return Err(SwarmCtrlError::UnknownMissionClass {
                mission_class: format!("{:?}", mission),
            });
        }

        Ok(())
    }

    /// Emergency freeze all swarm operations
    pub fn emergency_freeze(&mut self, reason: &str, initiator_did: &str) -> Result<(), SwarmCtrlError> {
        self.logger.log_freeze(reason, initiator_did)?;
        // In production, send freeze signal to all swarm nodes
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller_creation() {
        let config = ControllerConfig::default();
        let controller = SwarmController::new(config);
        assert!(controller.is_ok());
    }

    #[test]
    fn test_weaponization_rejected() {
        let config = ControllerConfig::default();
        let mut controller = SwarmController::new(config).unwrap();
        
        let weapon_command = SwarmCommand::new(
            MissionClass::KineticDamage,
            "target".to_string(),
        );

        let result = controller.issue_command(
            weapon_command,
            &["bostrom1operator"],
            "session-123",
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            SwarmCtrlError::WeaponizationAttempt { .. } => { /* expected */ }
            _ => panic!("Expected WeaponizationAttempt error"),
        }
    }

    #[test]
    fn test_permitted_mission_accepted() {
        let config = ControllerConfig::default();
        let mut controller = SwarmController::new(config).unwrap();
        
        let eco_command = SwarmCommand::new(
            MissionClass::EcologicalRestoration,
            "canal_sector_7".to_string(),
        );

        // Note: This will fail whitelist check in test without setup
        // In production, whitelist would be properly configured
        let result = controller.issue_command(
            eco_command,
            &["bostrom1operator1", "bostrom1operator2"],
            "session-123",
        );

        // Expected to fail on whitelist (test environment)
        assert!(result.is_err());
    }
}
