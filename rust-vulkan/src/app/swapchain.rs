use super::App;
use super :: render_pass :: AppRenderPass;
use super :: pipeline    :: AppPipeline;
use super :: framebuffer :: AppFramebuffer;
use super :: commands    :: AppCommands;

use vulkanalia :: prelude :: v1_0 :: *;
use vulkanalia :: vk      :: KhrSwapchainExtension;

use super::Window;
use super::AppData;
use super::Result;

use crate :: app :: device :: queue_family_indices :: QueueFamilyIndices;
use crate :: app :: device :: swapchain_support    :: SwapchainSupport;

pub struct AppSwapchain {}
impl AppSwapchain {
    pub fn get_swapchain_surface_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
        // For the color space we'll use sRGB if it is available
        formats
            .iter()
            .cloned()
            .find(|f| {
                f.format == vk::Format::B8G8R8A8_SRGB
                    && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            })
            .unwrap_or_else(|| formats[0])  // If that also fails then we could rank the available formats
    }

    pub fn get_swapchain_present_mode(present_modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
        // vk::PresentModeKHR::IMMEDIATE     – Images submitted by application are transferred to the screen right away
        // vk::PresentModeKHR::FIFO          – The swapchain is a FIFO queue (aka "vertical blank")
        // vk::PresentModeKHR::FIFO_RELAXED  - If queue is empty, send picture to screen right away
        // vk::PresentModeKHR::MAILBOX       - If queue is full, replace images with newer ones (aka "tripple buffering")
    
        // Only the vk::PresentModeKHR::FIFO mode is guaranteed to be available
        present_modes
            .iter()
            .cloned()
            .find(|m| *m == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO)    // If MAILBOX is not available, use FIFO
    }

    pub fn get_swapchain_extent(window: &Window, capabilities: vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
        // Some window managers do allow us to differ here 
        // and this is indicated by setting the width and height 
        // in `current_extent` to a special value: the maximum value of u32
        if capabilities.current_extent.width != u32::max_value() {capabilities.current_extent}
        else {
            let size = window.inner_size();
            let clamp = |min: u32, max: u32, v: u32| min.max(max.min(v));
            vk::Extent2D::builder()
                .width(clamp(
                    capabilities.min_image_extent.width,
                    capabilities.min_image_extent.width,
                    size.width,
                ))
                .height(clamp(
                    capabilities.min_image_extent.height,
                    capabilities.max_image_extent.height,
                    size.height
                ))
                .build()
        }
    
        // We define the clamp function to restrict the actual size 
        // of the window within the range supported by the Vulkan device.
    }

    pub unsafe fn create_swapchain(window: &Window, instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
        let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;
        let support = SwapchainSupport::get(instance, data, data.physical_device)?;
    
        let surface_format = Self::get_swapchain_surface_format(&support.formats);
        let present_mode = Self::get_swapchain_present_mode(&support.present_modes);
        let extent = Self::get_swapchain_extent(window, support.capabilities);
    
        let mut image_count = support.capabilities.min_image_count + 1;     // How many images we would like to have in the swapchain
        if support.capabilities.max_image_count != 0
            && image_count > support.capabilities.max_image_count {
                image_count = support.capabilities.max_image_count;
            }
    
        // There are two ways to handle images that are accessed from multiple queues:
        // vk::SharingMode::EXCLUSIVE  – An image is owned by one queue family at a time 
        // vk::SharingMode::CONCURRENT – Images can be used across multiple queue families 
        
        // If the graphics queue family and presentation queue family are THE SAME  - EXCLUSIVE
        // If the graphics queue family and presentation queue family are DIFFERENT - CONCURRENT
        let mut queue_family_indices = vec![];
        let image_sharing_mode = if indices.graphics != indices.present {
            queue_family_indices.push(indices.graphics);
            queue_family_indices.push(indices.present);
            vk::SharingMode::CONCURRENT
        } else {vk::SharingMode::EXCLUSIVE};
    
    
        let info = vk::SwapchainCreateInfoKHR::builder()
            .surface(data.surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(image_sharing_mode)
            .queue_family_indices(&queue_family_indices)
            .pre_transform(support.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(vk::SwapchainKHR::null());
    
        data.swapchain = device.create_swapchain_khr(&info, None)?;
        data.swapchain_images = device.get_swapchain_images_khr(data.swapchain)?;
        data.swapchain_format = surface_format.format;
        data.swapchain_extent = extent;
    
        Ok(())
    }


    pub unsafe fn create_swapchain_image_views(device: &Device, data: &mut AppData) -> Result<()> {
        data.swapchain_image_views = data
            .swapchain_images
            .iter()
            .map(|i| {
                // For each image view we are creating we'll first need to define the color component mapping for the image view.
                let components = vk::ComponentMapping::builder()
                    .r(vk::ComponentSwizzle::IDENTITY)
                    .g(vk::ComponentSwizzle::IDENTITY)
                    .b(vk::ComponentSwizzle::IDENTITY)
                    .a(vk::ComponentSwizzle::IDENTITY);
    
                // Next we will define the subresource range for the image view which describes the image's purpose and which part of the image should be accessed.
                let subresource_range = vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1);
    
                // Parameters for image view creation.
                let info = vk::ImageViewCreateInfo::builder()
                    .image(*i)
                    .view_type(vk::ImageViewType::_2D)
                    .format(data.swapchain_format)
                    .components(components)
                    .subresource_range(subresource_range);
    
                // The view_type and format fields specify how the image data should be interpreted. 
                // The view_type field allows you to treat images as 1D textures, 2D textures, 3D textures, and cube maps.
    
                device.create_image_view(&info, None)
            })
            .collect::<Result<Vec<_>, _>>()?;
    
        Ok(())
    }

    pub unsafe fn recreate_swapchain(instance: &Instance, device: &Device, data: &mut AppData, window: &Window) -> Result<()> {
        // If we want to resize the window, we have to recreate whole swapchain
        // and all other steps that come with it
        
        device.device_wait_idle()?;
        Self::destroy_swapchain(&device, data);

        Self::create_swapchain(window, &instance, &device, data)?;
        Self::create_swapchain_image_views(&device, data)?;
        AppRenderPass::create_render_pass(&instance, &device, data)?;
        AppPipeline::create_pipeline(&device, data)?;
        AppFramebuffer::create_framebuffer(&device, data)?;
        AppCommands::create_command_buffer(&device, data)?;
        data
            .images_in_flight
            .resize(data.swapchain_images.len(), vk::Fence::null());

        Ok(())
    }

    pub unsafe fn destroy_swapchain(device: &Device, data: &mut AppData) {
        // We don't need to destroy everything on `recreate_window`
        // so we just destroy swapchain related stuff

        data.framebuffers
            .iter()
            .for_each(|f| device.destroy_framebuffer(*f, None));

        device.free_command_buffers(data.command_pool, &data.command_buffers);

        device.destroy_pipeline(data.pipeline, None);
        device.destroy_pipeline_layout(data.pipeline_layout, None);
        device.destroy_render_pass(data.render_pass, None);

        data.swapchain_image_views
            .iter()
            .for_each(|iv| device.destroy_image_view(*iv, None));

        device.destroy_swapchain_khr(data.swapchain, None);
    }
}