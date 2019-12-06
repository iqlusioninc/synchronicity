//! Transaction verifier

use crate::executor::SynchronicityExecutor;
use std::sync::Arc;
use synchro::{
    error::Error,
    futures::future::Future,
    storage_client::StorageRead,
    transaction::{NewVerifier, SignedTransaction, Status, TransactionValidation},
};

/// Verification provider
pub struct VerifyProvider {}

impl VerifyProvider {
    /// Create a new verify provider
    #[allow(clippy::new_without_default)] // sate clippy, for now
    pub fn new() -> Self {
        Self {}
    }
}

impl NewVerifier for VerifyProvider {
    type Verifier = Verifier;

    /// Initialize a transaction validator from the given storage reader
    fn new_verifier(&self, storage_read_client: Arc<dyn StorageRead>) -> Verifier {
        Verifier::new(storage_read_client)
    }
}

/// Validator for Synchronicity transactions
#[allow(dead_code)] // TODO(tarcieri): verify stuff
pub struct Verifier {
    storage_read_client: Arc<dyn StorageRead>,
    executor: SynchronicityExecutor,
}

impl Verifier {
    pub fn new(storage_read_client: Arc<dyn StorageRead>) -> Self {
        // TODO(tarcieri): make this real
        let executor = SynchronicityExecutor::new();

        Self {
            storage_read_client,
            executor,
        }
    }
}

impl TransactionValidation for Verifier {
    type ValidationInstance = SynchronicityExecutor;

    /// Validate transaction. See example here for how it's supposed to work:
    ///
    /// <https://github.com/libra/libra/blob/testnet/vm-validator/src/vm_validator.rs#L47>
    fn validate_transaction(
        &self,
        _txn: SignedTransaction,
    ) -> Box<dyn Future<Item = Option<Status>, Error = Error> + Send> {
        unimplemented!();
    }
}
