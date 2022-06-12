use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crate::device::{Device, DeviceOwned};
use crate::swapchain::Swapchain;
use crate::sync::{Fence, FlushError, GpuFuture2, Semaphore};

pub struct SwapchainAcquireFuture2<W> {
    pub swapchain: Arc<Swapchain<W>>,
    pub image_id: usize,
    // Semaphore that is signalled when the acquire is complete. Empty if the acquire has already
    // happened.
    pub semaphore: Option<Semaphore>,
    // Fence that is signalled when the acquire is complete. Empty if the acquire has already
    // happened.
    pub fence: Option<Fence>,
    pub finished: AtomicBool,
}

unsafe impl<W> GpuFuture2 for SwapchainAcquireFuture2<W> {
    fn cleanup_finished(&mut self) {}

    fn flush(&mut self) -> Result<(), FlushError> { Ok(()) }
}

impl<W> Drop for SwapchainAcquireFuture2<W> {
    fn drop(&mut self) {
        if let Some(ref fence) = self.fence {
            fence.wait(None).unwrap(); // TODO: handle error?
            self.semaphore = None;
        }

        // TODO: if this future is destroyed without being presented, then eventually acquiring
        // a new image will block forever ; difficulty: hard
    }
}

unsafe impl<W> DeviceOwned for SwapchainAcquireFuture2<W> {
    #[inline]
    fn device(&self) -> &Arc<Device> {
        &self.swapchain.device()
    }
}