//! Capability Definitions - Non-weapon envelope and capability lattice
//!
//! This module defines the non-weaponization constraints for
//! Nanoswarm operations.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Non-weapon envelope for Nanoswarm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonWeaponEnvelope {
    pub envelope_id: String,
    pub permitted_missions: Vec<String>,
    pub forbidden_missions: Vec<String>,
    pub effect_type: String,
    pub mission_class: String,
    pub requires_multi_sig: bool,
}

impl NonWeaponEnvelope {
    /// Create a new non-weapon envelope
    pub fn new(mission_class: &str) -> Self {
        Self {
            envelope_id: Uuid::new_v4().to_string(),
            permitted_missions: Self::get_permitted_missions(),
            forbidden_missions: Self::get_forbidden_missions(),
            effect_type: "eco".to_string(),
            mission_class: mission_class.to_string(),
            requires_multi_sig: true,
        }
    }

    /// Get list of permitted mission classes
    pub fn get_permitted_missions() -> Vec<String> {
        vec![
            "ecological_restoration".to_string(),
            "clinical_assistive_care".to_string(),
            "diagnostics".to_string(),
            "neuromorphic_swarm_maintenance".to_string(),
            "canal_construction".to_string(),
            "eco_corridor_setup".to_string(),
        ]
    }

    /// Get list of forbidden mission classes
    pub fn get_forbidden_missions() -> Vec<String> {
        vec![
            "kinetic_damage".to_string(),
            "crowd_control".to_string(),
            "surveillance_without_consent".to_string(),
            "weapon_deployment".to_string(),
            "offensive_operations".to_string(),
        ]
    }

    /// Validate mission class against envelope
    pub fn validate_mission(&self, mission: &str) -> bool {
        self.permitted_missions.contains(&mission.to_string())
            && !self.forbidden_missions.contains(&mission.to_string())
    }
}

/// Nanoswarm capability enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NanoswarmCapability {
    SwarmDeploy,
    SwarmRecall,
    SwarmReconfigure,
    SwarmDiagnostics,
    SwarmMaintenance,
    EcoRestoration,
    ClinicalCare,
}

/// Forbidden capability combinations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForbiddenCapabilityCombo {
    pub combo_id: String,
    pub capabilities: Vec<String>,
    pub reason: String,
    pub severity: String,
}

impl ForbiddenCapabilityCombo {
    /// Get default forbidden combinations
    pub fn defaults() -> Vec<Self> {
        vec![
            ForbiddenCapabilityCombo {
                combo_id: "swarm_ctrl_network_server".to_string(),
                capabilities: vec!["NANOSWARM_CTRL".to_string(), "NETSERVER".to_string()],
                reason: "Prevents remote weaponization of Nanoswarm".to_string(),
                severity: "critical".to_string(),
            },
            ForbiddenCapabilityCombo {
                combo_id: "swarm_ctrl_hardware".to_string(),
                capabilities: vec!["NANOSWARM_CTRL".to_string(), "USB_HID".to_string()],
                reason: "Prevents hardware exploitation via swarm".to_string(),
                severity: "critical".to_string(),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_creation() {
        let envelope = NonWeaponEnvelope::new("ecological_restoration");
        assert!(!envelope.envelope_id.is_empty());
        assert!(!envelope.permitted_missions.is_empty());
        assert!(!envelope.forbidden_missions.is_empty());
    }

    #[test]
    fn test_mission_validation() {
        let envelope = NonWeaponEnvelope::new("ecological_restoration");
        
        assert!(envelope.validate_mission("ecological_restoration"));
        assert!(!envelope.validate_mission("kinetic_damage"));
    }

    #[test]
    fn test_forbidden_combos() {
        let combos = ForbiddenCapabilityCombo::defaults();
        assert!(!combos.is_empty());
    }
}
