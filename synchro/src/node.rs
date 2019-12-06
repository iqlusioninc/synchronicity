//! Synchronicity node type: owns all state for a running node

use consensus::consensus_provider::ConsensusProvider;
use executor::Executor;
use libra_mempool::MempoolRuntime;
use std::sync::Arc;
use vm_runtime::VMExecutor;

/// Synchronicity full node runtime
pub struct Node<V>
where
    V: VMExecutor + Send + Sync + 'static,
{
    /// Tokio runtime
    pub runtime: tokio::runtime::Runtime,

    /// Consensus provider
    pub consensus: Box<dyn ConsensusProvider>,

    /// Mempool runtime
    pub mempool: MempoolRuntime,

    /// Executor
    pub executor: Arc<Executor<V>>,
}
