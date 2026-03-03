# Nanoswarm Secure Control

**Hardened Nanoswarm control interface with NDM integration and non-weaponization enforcement**

[![License: ASL-1.0](https://img.shields.io/badge/License-ASL--1.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/nanoswarm-secure-ctrl.svg)](https://crates.io/crates/nanoswarm-secure-ctrl)
[![Docs](https://docs.rs/nanoswarm-secure-ctrl/badge.svg)](https://docs.rs/nanoswarm-secure-ctrl)
[![Hex-Stamp](https://img.shields.io/badge/hex--stamp-0xbe7f3e6d5c2b8a0f9e4d3c2b1a0f9e8d7c6b5a49-green.svg)](docs/security/hex-stamp-attestation.md)
[![Audit Status](https://img.shields.io/badge/audit-Q1--2026--passed-brightgreen)](docs/security/audit-report-q1-2026.md)

## Purpose

`nanoswarm-secure-ctrl` is the **only authorized control interface** for Nanoswarm operations in the ALN Sovereign Stack. All swarm commands must pass through this crate's capability gates, NDM monitoring, and multi-DID authorization system.

This guarantees:
- **Non-Weaponization** - `NANOSWARM_CTRL` capability requires valid non-weapon envelope
- **NDM Integration** - Automatic freeze on suspicious patterns (NDM ≥ 0.7)
- **Multi-DID Authorization** - No single point of compromise for swarm control
- **Complete Audit Trail** - Every command logged to ROW/RPM with Cyberspectre trace
- **Offline-First** - Swarm control works without network connectivity

## Architecture

┌─────────────────────────────────────────────────────────────────┐
│ OPERATOR INTERFACES │
│ (CLI / Dashboard / SDK / AI-Chat Gateway) │
└────────────────────────────┬────────────────────────────────────┘
│ Swarm Commands
▼
┌─────────────────────────────────────────────────────────────────┐
│ nanoswarm-secure-ctrl │
│ ┌───────────────────────────────────────────────────────────┐ │
│ │ CommandController (capability-gated issuance) │ │
│ └───────────────────────────────────────────────────────────┘ │
│ │ │ │ │
│ ▼ ▼ ▼ │
│ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ │
│ │NDMMonitor │ │FreezeProtocol│ │AuditLogger │ │
│ └──────────────┘ └──────────────┘ └──────────────┘ │
│ │ │ │ │
│ └──────────────────┼──────────────────┘ │
│ ▼ │
│ ┌───────────────────────────────────────────────────────────┐ │
│ │ DIDWhitelist (operator authorization) │ │
│ └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────────────────────┐
│ NANOSWARM NODES │
│ (Ecological Restoration / Healthcare / Diagnostics) │
└─────────────────────────────────────────────────────────────────┘


## Key Components

| Component | Description |
|-----------|-------------|
| `CommandController` | Capability-gated swarm command issuance |
| `NDMMonitor` | Real-time NDM scoring for swarm sessions |
| `FreezeProtocol` | Emergency freeze for compromised sessions |
| `AuditLogger` | Command history logging to ROW/RPM |
| `DIDWhitelist` | DID-based operator authorization system |
| `NonWeaponEnvelope` | Mission class validation for NANOSWARM_CTRL |

## Permitted Mission Classes

| Mission Class | Description | Status |
|---------------|-------------|--------|
| `ecological_restoration` | Canal construction, eco-corridor setup | ✅ Allowed |
| `clinical_assistive_care` | Healthcare support for augmented citizens | ✅ Allowed |
| `diagnostics` | Environmental and health diagnostics | ✅ Allowed |
| `neuromorphic_swarm_maintenance` | Swarm node maintenance and repair | ✅ Allowed |
| `kinetic_damage` | Any weaponization or harm | ❌ Forbidden |
| `crowd_control` | Surveillance or control of populations | ❌ Forbidden |
| `offensive_operations` | Any offensive capability | ❌ Forbidden |

## Quick Start

```bash
# Clone the repository
git clone https://github.com/aln-sovereign/nanoswarm-secure-ctrl.git
cd nanoswarm-secure-ctrl

# Build with all features
cargo build --release --features full-ndm-integration

# Initialize control interface
cargo run --bin nanoswarm-ctrl -- init --config config/ndm_thresholds.aln

# Issue swarm command (requires multi-DID auth)
cargo run --bin nanoswarm-ctrl -- command --mission ecological_restoration --target canal_sector_7

# Check NDM status
cargo run --bin nanoswarm-ctrl -- ndm-status --session <session_id>

# Emergency freeze all swarm operations
cargo run --bin nanoswarm-ctrl -- emergency-freeze --reason "suspicious_pattern_detected"

NDM Threshold Configuration
[table-7ac8f8d0-d83d-4b2a-b1cb-d2450cb2b1f5.csv](https://github.com/user-attachments/files/25728787/table-7ac8f8d0-d83d-4b2a-b1cb-d2450cb2b1f5.csv)
Threshold,Value,Action
normal_ceiling,0.3,Full swarm capabilities
monitoring_ceiling,0.6,"Enhanced monitoring, no new missions"
degrade_ceiling,0.8,"ObserveOnly mode, freeze new commands"
auto_freeze_threshold,0.7,Automatic freeze trigger
multisig_threshold,0.5,Multi-sig required above this
quarantine_ceiling,1.0,"Full isolation, audit required"

Security Properties
Capability-Gated - All commands require NANOSWARM_CTRL capability Sourze
Non-Weaponized - Mission class validation prevents weaponization
NDM-Monitored - Real-time scoring with automatic freeze on suspicion
Multi-DID Signed - No single operator can compromise swarm
Audit-Logged - Every command traced to ROW/RPM + Cyberspectre
Governance
All swarm operations require:
Valid Non-Weapon Envelope - Mission class must be permitted
DID Authorization - Operator must be on whitelist with valid NDM score
ROW/RPM Anchoring - Every command logged to immutable ledger
Cyberspectre Trace - Full introspection of control paths
Hex-Stamp Attestation: 0xbe7f3e6d5c2b8a0f9e4d3c2b1a0f9e8d7c6b5a49f8e7d6c5b4a3928170f6e5d4
Ledger Reference: row:nanoswarm-secure-ctrl:v1.0.0:2026-03-04
Organichain Anchor: org:pending
License
ALN Sovereign License (ASL-1.0) - See LICENSE for details.
⚠️ Non-Weaponization Notice: This crate enforces nanoswarm.nonweapon.envelope.v1.aln constraints. Any attempt to issue weapon-class commands will trigger immediate NDM quarantine and ROW/RPM incident logging.
