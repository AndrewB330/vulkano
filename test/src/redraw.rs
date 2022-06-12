use std::sync::{Arc, Mutex};
use winit::window::Window;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassContents};
use vulkano::device::Device;
use vulkano::swapchain::{acquire_next_image, AcquireError, Swapchain};
use vulkano::sync::GpuFuture2;

pub fn redraw(device: Arc<Device>, swapchain: Arc<Swapchain<Window>>, prev_future: &mut Option<Arc<Mutex<dyn GpuFuture2>>>) {
    prev_future.as_mut().unwrap().lock().unwrap().cleanup_finished();

    let (image_num, suboptimal, acquire_future) =
        match acquire_next_image(swapchain.clone(), None) {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {
                return;
            }
            Err(e) => panic!("Failed to acquire next image: {:?}", e),
        };

    /*let mut builder = AutoCommandBufferBuilder::primary(
        device.clone(),
        queue.family(),
        CommandBufferUsage::OneTimeSubmit,
    ).unwrap();

    builder
        .begin_render_pass(
            RenderPassBeginInfo {
                // A list of values to clear the attachments with. This list contains
                // one item for each attachment in the render pass. In this case,
                // there is only one attachment, and we clear it with a blue color.
                //
                // Only attachments that have `LoadOp::Clear` are provided with clear
                // values, any others should use `ClearValue::None` as the clear value.
                clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                ..RenderPassBeginInfo::framebuffer(framebuffers[image_num].clone())
            },
            SubpassContents::Inline,
        )
        .unwrap()
        .set_viewport(0, [viewport.clone()])
        .bind_pipeline_graphics(pipeline.clone())
        .bind_vertex_buffers(0, vertex_buffer.clone())
        .draw(vertex_buffer.len() as u32, 1, 0, 0)
        .unwrap()
        // We leave the render pass. Note that if we had multiple
        // subpasses we could have called `next_subpass` to jump to the next subpass.
        .end_render_pass()
        .unwrap();

    // Finish building the command buffer by calling `build`.
    let command_buffer = builder.build().unwrap();*/

    /*let future = previous_frame_end
        .take()
        .unwrap()
        .join(acquire_future)
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        // The color output is now expected to contain our triangle. But in order to show it on
        // the screen, we have to *present* the image by calling `present`.
        //
        // This function does not actually present the image immediately. Instead it submits a
        // present command at the end of the queue. This means that it will only be presented once
        // the GPU has finished executing the command buffer that draws the triangle.
        .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
        .then_signal_fence_and_flush();*/

    let future = previous_frame_end.take().unwrap()
        .join(acquire_future)
        .then_swapchain_present(queue.clone(), swapchain.clone(), image_num);
}