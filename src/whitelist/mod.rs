//! DID Whitelist - Operator authorization system
//!
//! This module manages DID-based operator authorization for
//! Nanoswarm control operations.

use serde::{Deserialize, Serialize};
use crate::error::SwarmCtrlError;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// DID whitelist manager
pub struct DIDWhitelist {
    authorized_operators: HashMap<String, OperatorProfile>,
}

/// Operator profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorProfile {
    pub did: String,
    pub role: OperatorRole,
    pub ndm_ceiling: f64,
    pub authorized_missions: Vec<String>,
    pub added_timestamp: i64,
    pub last_active: Option<i64>,
    pub is_active: bool,
}

/// Operator role taxonomy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperatorRole {
    Owner,
    Maintainer,
    Auditor,
    Viewer,
}

impl DIDWhitelist {
    /// Create a new whitelist
    pub fn new() -> Self {
        Self {
            authorized_operators: HashMap::new(),
        }
    }

    /// Verify operators against whitelist
    ///
    /// # Arguments
    ///
    /// * `operator_dids` - List of operator DIDs to verify
    ///
    /// # Returns
    ///
    /// * `bool` - True if all operators are authorized
    pub fn verify_operators(&self, operator_dids: &[&str]) -> Result<bool, SwarmCtrlError> {
        if operator_dids.is_empty() {
            return Err(SwarmCtrlError::NoOperatorsProvided);
        }

        for did in operator_dids {
            let profile = self.authorized_operators.get(*did);
            
            match profile {
                Some(p) => {
                    if !p.is_active {
                        return Err(SwarmCtrlError::OperatorInactive {
                            did: did.to_string(),
                        });
                    }
                }
                None => {
                    return Err(SwarmCtrlError::OperatorNotWhitelisted {
                        did: did.to_string(),
                    });
                }
            }
        }

        Ok(true)
    }

    /// Add operator to whitelist
    pub fn add_operator(&mut self, profile: OperatorProfile) -> Result<(), SwarmCtrlError> {
        self.authorized_operators.insert(profile.did.clone(), profile);
        Ok(())
    }

    /// Remove operator from whitelist
    pub fn remove_operator(&mut self, did: &str) -> Result<(), SwarmCtrlError> {
        self.authorized_operators.remove(did);
        Ok(())
    }

    /// Get operator profile
    pub fn get_operator(&self, did: &str) -> Option<&OperatorProfile> {
        self.authorized_operators.get(did)
    }

    /// Get all active operators
    pub fn get_active_operators(&self) -> Vec<&OperatorProfile> {
        self.authorized_operators
            .values()
            .filter(|p| p.is_active)
            .collect()
    }
}

impl Default for DIDWhitelist {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitelist_creation() {
        let whitelist = DIDWhitelist::new();
        assert!(whitelist.authorized_operators.is_empty());
    }

    #[test]
    fn test_operator_verification() {
        let mut whitelist = DIDWhitelist::new();
        
        let profile = OperatorProfile {
            did: "bostrom1test".to_string(),
            role: OperatorRole::Maintainer,
            ndm_ceiling: 0.5,
            authorized_missions: vec!["ecological_restoration".to_string()],
            added_timestamp: Utc::now().timestamp(),
            last_active: None,
            is_active: true,
        };

        whitelist.add_operator(profile).unwrap();
        
        let result = whitelist.verify_operators(&["bostrom1test"]);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_unauthorized_operator_rejected() {
        let whitelist = DIDWhitelist::new();
        let result = whitelist.verify_operators(&["bostrom1unknown"]);
        assert!(result.is_err());
    }
}
