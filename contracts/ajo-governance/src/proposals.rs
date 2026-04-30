//! AjoChain Governance — Proposal data types

use soroban_sdk::{contracttype, Address, String, Vec};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Governance configuration (Instance storage).
    Config,
    /// Proposal by ID (Persistent storage).
    Proposal(u64),
}

/// Governance configuration.
#[contracttype]
#[derive(Clone, Debug)]
pub struct GovernanceConfig {
    /// Protocol admin.
    pub admin: Address,
    /// Security council members.
    pub council_members: Vec<Address>,
    /// Timelock delay in seconds.
    pub timelock_delay: u64,
    /// Total proposals created.
    pub proposal_count: u64,
    /// Whether governance is paused.
    pub is_paused: bool,
}

/// The lifecycle status of a proposal.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ProposalStatus {
    /// Awaiting timelock expiry.
    Pending = 0,
    /// Executed successfully.
    Executed = 1,
    /// Vetoed by a council member.
    Vetoed = 2,
    /// Cancelled by the proposer.
    Cancelled = 3,
}

/// Types of governance actions.
#[contracttype]
#[derive(Clone, Debug)]
pub enum ProposalAction {
    /// Update the protocol fee.
    UpdateFee(u32),
    /// Add a token to the allowlist.
    AddToken(Address),
    /// Remove a token from the allowlist.
    RemoveToken(Address),
    /// Upgrade a contract to a new WASM hash.
    UpgradeContract(Address),
    /// Transfer admin role.
    TransferAdmin(Address),
    /// Custom action with encoded data.
    Custom(String),
}

/// A governance proposal.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Proposal {
    /// Unique proposal ID.
    pub id: u64,
    /// The address that created this proposal.
    pub proposer: Address,
    /// Human-readable title.
    pub title: String,
    /// Detailed description.
    pub description: String,
    /// The action to execute.
    pub action: ProposalAction,
    /// Current status.
    pub status: ProposalStatus,
    /// When the proposal was created.
    pub created_at: u64,
    /// When the proposal can be executed (after timelock).
    pub executable_at: u64,
    /// When the proposal was executed (0 if not yet).
    pub executed_at: u64,
    /// Address that vetoed (if applicable).
    pub vetoed_by: Option<Address>,
}
