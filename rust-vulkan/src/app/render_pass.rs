use vulkanalia::prelude::v1_0::*;

use super::AppData;
use super::Result;

pub struct AppRenderPass {}
impl AppRenderPass {
    pub unsafe fn create_render_pass(instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
        // A single render pass can consist of multiple subpasses. 
        // Subpasses are subsequent rendering operations that depend 
        // on the contents of framebuffers in previous passes
    
    
    
        // The `load_op` and `store_op` determine what to do with the data in the attachment before rendering and after rendering. 
        // We have the following choices for `load_op`:
    
        // vk::AttachmentLoadOp::LOAD      – Preserve the existing contents of the attachment
        // vk::AttachmentLoadOp::CLEAR     – Clear the values to a constant at the start
        // vk::AttachmentLoadOp::DONT_CARE – Existing contents are undefined; we don't care about them
    
    
        // In our case we're going to use the clear operation to clear the framebuffer to black before drawing a new frame. 
        // There are only two possibilities for the `store_op`:
    
        // vk::AttachmentStoreOp::STORE     – Rendered contents will be stored in memory and can be read later
        // vk::AttachmentStoreOp::DONT_CARE – Contents of the framebuffer will be undefined after the rendering operation
    
    
        let color_attachment = vk::AttachmentDescription::builder()
            .format(data.swapchain_format)
            .samples(vk::SampleCountFlags::_1)  // The format of the color attachment should match the format of the `swapchain` images
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);
        
        // Textures and framebuffers in Vulkan are represented by vk::Image objects with a certain pixel format
        // Some of the most common layouts are:
    
        // vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL – Images used as color attachment
        // vk::ImageLayout::PRESENT_SRC_KHR          – Images to be presented in the swapchain
        // vk::ImageLayout::TRANSFER_DST_OPTIMAL     – Images to be used as destination for a memory copy operation
    
    
        // The index of the attachment in this array is directly referenced 
        // from the fragment shader with the `layout(location = 0) out vec4 outColor` directive!
        let color_attachment_ref = vk::AttachmentReference::builder()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let color_attachments = &[color_attachment_ref];
        let subpass = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(color_attachments);
    
        // The `dst_subpass` must always be higher than `src_subpass` to prevent cycles in the dependency graph.
        // Unless one of the subpasses is `vk::SUBPASS_EXTERNAL`.
        let dependency = vk::SubpassDependency::builder()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);
    
        let attachments = &[color_attachment];
        let subpasses = &[subpass];
        let dependencies = &[dependency];
        let info = vk::RenderPassCreateInfo::builder()
            .attachments(attachments)
            .subpasses(subpasses)
            .dependencies(dependencies);
    
        data.render_pass = device.create_render_pass(&info, None)?;
        Ok(())
    }
}