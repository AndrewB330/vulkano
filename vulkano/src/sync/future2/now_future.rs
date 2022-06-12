use std::sync::{Arc, Mutex};
use crate::device::{Device, DeviceOwned};
use crate::sync::{FlushError, GpuFuture2};

/// Represents already signaled future. Does not lock any resource.
pub struct NowFuture2 {
    device: Arc<Device>,
}

pub fn now2(device: Arc<Device>) -> Arc<Mutex<dyn GpuFuture2>> {
    Arc::new(Mutex::new(NowFuture2 { device }))
}

unsafe impl GpuFuture2 for NowFuture2 {
    fn cleanup_finished(&mut self) {}

    fn flush(&mut self) -> Result<(), FlushError> { Ok(()) }
}

unsafe impl DeviceOwned for NowFuture2 {
    #[inline]
    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}
