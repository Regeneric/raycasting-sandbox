use vulkanalia::prelude::v1_0::*;

use crate :: app :: device :: queue_family_indices :: QueueFamilyIndices;

use super::Instance;
use super::Device;
use super::AppData;
use super::{Result, Ok};

pub struct AppCommands {}
impl AppCommands {
    pub unsafe fn create_command_pool(instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
        // Command buffers are executed by submitting them on one of the device queues
        // Each command pool can only allocate command buffers that are submitted on a single type of queue
    
        // There are three possible flags for command pools:
        // vk::CommandPoolCreateFlags::TRANSIENT            – Hint that command buffers are rerecorded with new commands very often (may change memory allocation behavior)
        // vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER – Allow command buffers to be rerecorded individually, without this flag they all have to be reset together
        // vk::CommandPoolCreateFlags::PROTECTED            – Creates "protected" command buffers which are stored in "protected" memory where Vulkan prevents unauthorized operations from accessing the memory
    
    
        let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;
        let info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::empty())
            .queue_family_index(indices.graphics);
    
        data.command_pool = device.create_command_pool(&info, None)?;
        Ok(())
    }
    
    pub unsafe fn create_command_buffer(device: &Device, data: &mut AppData) -> Result<()> {
        // The level parameter specifies if the allocated command buffers are primary or secondary command buffers.
        // vk::CommandBufferLevel::PRIMARY   – Can be submitted to a queue for execution, but cannot be called from other command buffers.
        // vk::CommandBufferLevel::SECONDARY – Cannot be submitted directly, but can be called from primary command buffers.
    
        let allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(data.command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(data.framebuffers.len() as u32);
    
        data.command_buffers = device.allocate_command_buffers(&allocate_info)?;
    
        for (i, command_buffer) in data.command_buffers.iter().enumerate() {
            // let inheritance = vk::CommandBufferInheritanceInfo::builder();
            // let info = vk::CommandBufferBeginInfo::builder()
            //     .flags(vk::CommandBufferUsageFlags::empty())
            //     .inheritance_info(&inheritance);
    
            let info = vk::CommandBufferBeginInfo::builder();
            device.begin_command_buffer(*command_buffer, &info)?;
    
            let render_area = vk::Rect2D::builder()
                .offset(vk::Offset2D::default())
                .extent(data.swapchain_extent);
            let color_clear_value = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            };
            let clear_values = &[color_clear_value];
            let info = vk::RenderPassBeginInfo::builder()
                .render_pass(data.render_pass)
                .framebuffer(data.framebuffers[i])
                .render_area(render_area)
                .clear_values(clear_values);
    
            device.cmd_begin_render_pass(*command_buffer, &info, vk::SubpassContents::INLINE);
            device.cmd_bind_pipeline(*command_buffer, vk::PipelineBindPoint::GRAPHICS, data.pipeline);
            device.cmd_draw(*command_buffer, 3, 1, 0, 0);
            device.cmd_end_render_pass(*command_buffer);
    
            device.end_command_buffer(*command_buffer)?;
        }
    
        Ok(())
    }
}