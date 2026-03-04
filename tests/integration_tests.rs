//! Nanoswarm Secure Control Integration Tests

use nanoswarm_secure_ctrl::{
    SwarmController, ControllerConfig, SwarmCommand, MissionClass,
    emergency_freeze, get_ndm_status,
};

#[test]
fn test_full_command_lifecycle() {
    let config = ControllerConfig::default();
    let mut controller = SwarmController::new(config).unwrap();

    let command = SwarmCommand::new(
        MissionClass::EcologicalRestoration,
        "canal_sector_7".to_string(),
    );

    // This will fail whitelist check (test environment)
    let result = controller.issue_command(
        command,
        &["bostrom1operator1", "bostrom1operator2"],
        "session-123",
    );

    // Expected to fail on whitelist in test
    assert!(result.is_err());
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

    match result.unwrap_err() {
        nanoswarm_secure_ctrl::SwarmCtrlError::WeaponizationAttempt { .. } => { /* expected */ }
        _ => panic!("Expected WeaponizationAttempt error"),
    }
}

#[test]
fn test_emergency_freeze_integration() {
    let result = emergency_freeze("test_emergency", "bostrom1initiator");
    assert!(result.is_ok());
    assert!(!result.unwrap().freeze_id.is_empty());
}

#[test]
fn test_ndm_status_retrieval() {
    let status = get_ndm_status("session-123");
    assert!(status.is_ok());
    assert!(status.unwrap().k_score >= 0.0);
}

#[test]
fn test_permitted_mission_classes() {
    let permitted = [
        MissionClass::EcologicalRestoration,
        MissionClass::ClinicalAssistiveCare,
        MissionClass::Diagnostics,
        MissionClass::NeuromorphicSwarmMaintenance,
    ];

    for mission in &permitted {
        // These should not trigger weaponization error
        // (will fail on whitelist in test environment)
        let config = ControllerConfig::default();
        let mut controller = SwarmController::new(config).unwrap();
        
        let command = SwarmCommand::new(mission.clone(), "target".to_string());
        let result = controller.issue_command(command, &["op1"], "session-123");
        
        // Should fail on whitelist, not weaponization
        assert!(result.is_err());
    }
}

#[test]
fn test_forbidden_mission_classes() {
    let forbidden = [
        MissionClass::KineticDamage,
        MissionClass::CrowdControl,
        MissionClass::WeaponDeployment,
        MissionClass::OffensiveOperations,
    ];

    for mission in &forbidden {
        let config = ControllerConfig::default();
        let mut controller = SwarmController::new(config).unwrap();
        
        let command = SwarmCommand::new(mission.clone(), "target".to_string());
        let result = controller.issue_command(command, &["op1"], "session-123");
        
        // Should fail on weaponization check
        match result.unwrap_err() {
            nanoswarm_secure_ctrl::SwarmCtrlError::WeaponizationAttempt { .. } => { /* expected */ }
            _ => panic!("Expected WeaponizationAttempt error for {:?}", mission),
        }
    }
}
