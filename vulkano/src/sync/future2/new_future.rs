use std::sync::Arc;
use crate::device::{DeviceOwned};
use crate::sync::{FlushError};

pub unsafe trait GpuFuture2: DeviceOwned {
    fn cleanup_finished(&mut self);

    fn flush(&mut self) -> Result<(), FlushError>;
}

fn join(first: Arc<dyn GpuFuture2>, second: Arc<dyn GpuFuture2>) -> Arc<dyn GpuFuture2> {
    first
}

fn fork(future: Arc<dyn GpuFuture2>, resources_for_fork: &[u32]) -> (Arc<dyn GpuFuture2>, Arc<dyn GpuFuture2>) {
    (future.clone(), future)
}

