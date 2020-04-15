use std::sync::Arc;

use rustracing_jaeger::Tracer;

use protocol::traits::{Executor, ExecutorFactory, ServiceMapping, Storage};
use protocol::types::MerkleRoot;
use protocol::ProtocolResult;

use crate::executor::ServiceExecutor;

pub struct ServiceExecutorFactory;

impl<DB: 'static + cita_trie::DB, S: 'static + Storage, Mapping: 'static + ServiceMapping>
    ExecutorFactory<DB, S, Mapping> for ServiceExecutorFactory
{
    fn from_root(
        root: MerkleRoot,
        db: Arc<DB>,
        storage: Arc<S>,
        mapping: Arc<Mapping>,
        tracer: Tracer,
    ) -> ProtocolResult<Box<dyn Executor>> {
        let executor = ServiceExecutor::with_root(root, db, storage, mapping, tracer)?;
        Ok(Box::new(executor))
    }
}
