//! Transaction-related types/traits

pub use libra_types::{
    transaction::{SignedTransaction, Transaction, TransactionOutput},
    vm_error::VMStatus as Status,
};
pub use vm_runtime::VMVerifier;
pub use vm_validator::vm_validator::TransactionValidation;

use std::sync::Arc;
use storage_client::StorageRead;

/// Initialize a `TransactionValidation`-capable transaction validator
pub trait NewVerifier: Send + Sync {
    /// Transaction validator type this trait produces
    type Verifier: TransactionValidation + 'static;

    /// Initialize a transaction validator from the given storage reader
    fn new_verifier(&self, storage_read_client: Arc<dyn StorageRead>) -> Self::Verifier;
}
