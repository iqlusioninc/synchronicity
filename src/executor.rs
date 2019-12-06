//! Synchronicity state machine executor

use synchro::{
    config::VMConfig,
    error::Error,
    state_view::StateView,
    transaction::{SignedTransaction, Status, Transaction, TransactionOutput},
    vm_runtime::{VMExecutor, VMVerifier},
};

/// State machine executor used by Synchronicity
pub struct SynchronicityExecutor {}

impl SynchronicityExecutor {
    /// Create a new SynchronicityExecutor (stub!)
    #[allow(clippy::new_without_default)] // sate clippy, for now
    pub fn new() -> Self {
        Self {}
    }
}

impl VMExecutor for SynchronicityExecutor {
    fn execute_block(
        _transactions: Vec<Transaction>,
        _config: &VMConfig,
        _state_view: &dyn StateView,
    ) -> Result<Vec<TransactionOutput>, Error> {
        // TODO(tarcieri): implement this!
        unimplemented!();
    }
}

impl VMVerifier for SynchronicityExecutor {
    fn validate_transaction(
        &self,
        _transaction: SignedTransaction,
        _state_view: &dyn StateView,
    ) -> Option<Status> {
        // TODO(tarcieri): implement this!
        unimplemented!();
    }
}
