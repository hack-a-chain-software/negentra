pub use near_sdk::MockedBlockchain;
pub use near_sdk::{testing_env, Balance, VMContext};
pub use serial_test::{serial};

pub use schema::{CurveType, Schema};
pub use investment::{Investment};

pub use super::*;

pub use mocking::*;

pub mod mocking;

pub const CONTRACT_ACCOUNT: &str = "contract.testnet";
pub const TOKEN_ACCOUNT: &str = "token.testnet";
pub const SIGNER_ACCOUNT: &str = "signer.testnet";
pub const OWNER_ACCOUNT: &str = "owner.testnet";