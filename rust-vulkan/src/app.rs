pub mod app_data;
    mod instance;
    mod device;
    mod swapchain;
    mod render_pass;
    mod pipeline;
    mod framebuffer;
    mod commands;
    mod sync_objects;
    mod vertex;
    mod shared;

use crate :: app :: app_data     :: AppData;
use crate :: app :: instance     :: AppInstance;
use crate :: app :: device       :: AppDevice;
use crate :: app :: swapchain    :: AppSwapchain;
use crate :: app :: render_pass  :: AppRenderPass;
use crate :: app :: pipeline     :: AppPipeline;
use crate :: app :: framebuffer  :: AppFramebuffer;
use crate :: app :: commands     :: AppCommands;
use crate :: app :: sync_objects :: AppSyncObjects;
use crate :: app :: vertex       :: AppVertex;
use crate :: app :: shared       :: Commons;


use thiserror :: Error;
use anyhow    :: {anyhow, Result};

use std   :: collections :: HashSet;
use winit :: window      :: {Window};

use vulkanalia :: prelude :: v1_0 :: *;
use vulkanalia :: loader  :: {LibloadingLoader, LIBRARY};
use vulkanalia :: vk      :: KhrSurfaceExtension;
use vulkanalia :: vk      :: KhrSwapchainExtension;
use vulkanalia :: window  as vk_window;


// Value based on whether the program is being compiled in debug mode or not.
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];
const MAX_FRAMES_IN_FLIGHT: usize = 2;


#[derive(Clone, Debug)]
pub struct App {
pub device: Device,
pub resized: bool,
    entry: Entry,
    instance: Instance,
    data: AppData,
    frame: usize,
}
impl App {
    // Creates Vulkan app
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let mut data = AppData::default();

        let loader = LibloadingLoader::new(LIBRARY)?;                                   // Load initial Vulkan commands from shared library
        let entry = Entry::new(loader).map_err(|e| anyhow!("{}", e))?;                  // Load all commands we need to manage instances
        let instance = AppInstance::new(window, &entry)?;                               // Create instance with provided obejcts
        data.surface = vk_window::create_surface(&instance, &window, &window)?;         // Platform independent window creation
        AppDevice::pick_physical_device(&instance, &mut data)?;                         // Pick physical device (choose features, type etc.)         
        let device = AppDevice::create_logical_device(&entry, &instance, &mut data)?;   // Set up a logical device to interface with physical device                        
        AppSwapchain::create_swapchain(window, &instance, &device, &mut data)?;         // Infrastructure that will own the buffers (there's no `default buffer`)
        AppSwapchain::create_swapchain_image_views(&device, &mut data)?;                // Create a basic image view for every image in the swapchain
        AppRenderPass::create_render_pass(&instance, &device, &mut data)?;              // Tell Vulkan about the framebuffer attachments that will be used while rendering.
        AppPipeline::create_pipeline(&device, &mut data)?;                              // Sequence of operations that take the vertices and textures in the render targets                                           
        AppFramebuffer::create_framebuffer(&device, &mut data)?;                        // Create a framebuffer for all of the images in the swapchain
        AppCommands::create_command_pool(&instance, &device, &mut data)?;               // Command pools manage the memory that is used to store the buffers and command buffers are allocated from them
        AppVertex::create_vertex_buffer(&instance, &device, &mut data)?;                // Vertex buffer to store vertices
        AppCommands::create_command_buffer(&device, &mut data)?;                        // Start allocating command buffers and recording drawing commands in them
        AppSyncObjects::create_sync_objects(&device, &mut data)?;                       // Semaphores signals - image ready for rendering; And another one - rendering has finished                                  
        AppSwapchain::recreate_swapchain(&instance, &device, &mut data, window)?;       // On window resize, minimize etc.

        Ok(Self{entry, instance, data, device, frame: 0, resized: false})
    }

    // Renders a frame
    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        // Acquire an image from the swapchain
        // Execute the command buffer with that image as attachment in the framebuffer
        // Return the image to the swapchain for presentation

        self.device.wait_for_fences(
            &[self.data.in_flight_fences[self.frame]], 
            true,
            u64::max_value())?;

        let result = self.device.acquire_next_image_khr(
            self.data.swapchain, 
            u64::max_value(), 
            self.data.image_available_semaphore[self.frame], 
            vk::Fence::null());

        let image_index = match result {
            Ok((image_index, _)) => image_index as usize,
            Err(vk::ErrorCode::OUT_OF_DATE_KHR) => return AppSwapchain::recreate_swapchain(
                                                                        &self.instance, 
                                                                        &self.device, 
                                                                        &mut self.data, 
                                                                        window),
            Err(e) => return Err(anyhow!(e))
        };

        if !self.data.images_in_flight[image_index as usize].is_null() {
            self.device.wait_for_fences(
                &[self.data.images_in_flight[image_index as usize]], 
                true,
                u64::max_value())?;
        }

        self.data.images_in_flight[image_index as usize] = self.data.in_flight_fences[self.frame];

        let wait_semaphores = &[self.data.image_available_semaphore[self.frame]];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = &[self.data.command_buffers[image_index as usize]];
        let signal_semaphores = &[self.data.image_finished_semaphore[self.frame]];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores);

        self.device.reset_fences(&[self.data.in_flight_fences[self.frame]])?;

        self.device.queue_submit(
            self.data.graphics_queue, 
            &[submit_info], 
            self.data.in_flight_fences[self.frame])?;        

        let swapchains = &[self.data.swapchain];
        let image_indices = &[image_index as u32];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(signal_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        // Recreate the swapchain if it is suboptimal, because we want the best possible result
        let result = self.device.queue_present_khr(self.data.present_queue, &present_info);
        let changed = result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR) 
                   || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR);
        
        if self.resized || changed {
            self.resized = false;
            AppSwapchain::recreate_swapchain(&self.instance, &self.device, &mut self.data, window)?;
        } else if let Err(e) = result {return  Err(anyhow!(e));}

        self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;
        
        Ok(())
    }

    // Destroys Vulkan app
    pub unsafe fn destroy(&mut self) {
        // Manually destroying all things we used
        self.device.device_wait_idle().unwrap();

        AppSwapchain::destroy_swapchain(&self.device, &mut self.data);

        self.data.in_flight_fences
            .iter()
            .for_each(|f| self.device.destroy_fence(*f, None));

        self.data.image_finished_semaphore
            .iter()
            .for_each(|s| self.device.destroy_semaphore(*s, None));
        self.data.image_available_semaphore
            .iter()
            .for_each(|s| self.device.destroy_semaphore(*s, None));

        self.device.free_memory(self.data.vertex_buffer_memory, None);
        self.device.destroy_buffer(self.data.vertex_buffer, None);
        self.device.destroy_command_pool(self.data.command_pool, None);
        self.device.destroy_device(None);
        self.instance.destroy_surface_khr(self.data.surface, None);
        self.instance.destroy_instance(None);
    }
}