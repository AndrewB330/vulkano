mod new_future;
mod now_future;
mod swapchain_acquire_future;

pub use new_future::*;
pub use now_future::*;
pub use swapchain_acquire_future::*;

#[cfg(test)]
mod tests {
    use crate::image::ImageUsage;
    use crate::swapchain::{Swapchain, SwapchainCreateInfo};
    use crate::sync::future2::now2;

    #[test]
    fn create() {
        let (device, _) = gfx_dev_and_queue!();

        let future = now2(device.clone());

        let event_loop = EventLoop::new();
        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();

        let (mut swapchain, images) = {
            let surface_capabilities = physical_device
                .surface_capabilities(&surface, Default::default())
                .unwrap();
            let image_format = Some(
                physical_device
                    .surface_formats(&surface, Default::default())
                    .unwrap()[0]
                    .0,
            );
            Swapchain::new(
                device.clone(),
                surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count,
                    image_format,
                    image_extent: surface.window().inner_size().into(),
                    image_usage: ImageUsage::color_attachment(),
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .iter()
                        .next()
                        .unwrap(),
                    ..Default::default()
                },
            )
                .unwrap()
        };
    }
}