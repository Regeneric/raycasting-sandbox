#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use thiserror::Error;
use anyhow::{anyhow, Result, Ok};
use log::*;

use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;

use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::window as vk_window;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::Version;
use vulkanalia::vk::{ExtDebugUtilsExtension, ShaderStageFlags};
use vulkanalia::vk::KhrSurfaceExtension;
use vulkanalia::vk::KhrSwapchainExtension;

use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};


// Value based on whether the program is being compiled in debug mode or not.
const VALIDATION_ENABLED: bool =
    cfg!(debug_assertions);

const VALIDATION_LAYER: vk::ExtensionName =
    vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");


const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];



fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Vulkan")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    let mut app = unsafe {App::create(&window)?};
    let mut destroying = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // Event handle - like in any other lib (eg. SFML)
        match event {
            Event::MainEventsCleared if !destroying => {
                unsafe {app.render(&window)}.unwrap();
            },
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                unsafe {app.destroy();}
            }
            _ => {}
        }
    });
}   



#[derive(Clone, Debug)]
struct App {
    entry: Entry,
    instance: Instance,
    data: AppData,
    device: Device,
}
impl App {
    // Creates Vulkan app
    unsafe fn create(window: &Window) -> Result<Self> {
        let mut data = AppData::default();

        let loader = LibloadingLoader::new(LIBRARY)?;                               // Load initial Vulkan commands from shared library
        let entry = Entry::new(loader).map_err(|e| anyhow!("{}", e))?;              // Load all commands we need to manage instances
        let instance = create_instance(window, &entry)?;                            // Create instance with provided obejcts
        data.surface = vk_window::create_surface(&instance, &window, &window)?;     // Platform independent window creation
        pick_physical_device(&instance, &mut  data)?;                               // Pick physical device (choose features, type etc.)
        let device = create_logical_device(&entry, &instance, &mut data)?;          // Set up a logical device to interface with physical device
        create_swapchain(window, &instance, &device, &mut data)?;                   // Infrastructure that will own the buffers (there's no `default buffer`)
        create_swapchain_image_views(&device, &mut data)?;                          // Create a basic image view for every image in the swapchain
        create_render_pass(&instance, &device, &mut data)?;                         // Tell Vulkan about the framebuffer attachments that will be used while rendering.
        create_pipeline(&device, &mut data)?;                                       // Sequence of operations that take the vertices and textures in the render targets
        create_framebuffer(&device, &mut data)?;                                    // Create a framebuffer for all of the images in the swapchain
        create_command_pool(&instance, &device, &mut data)?;                        // Command pools manage the memory that is used to store the buffers and command buffers are allocated from them
        create_command_buffer(&device, &mut data)?;                                 // Start allocating command buffers and recording drawing commands in them
        create_sync_objects(&device, &mut data)?;                                   // Semaphores signals - image ready for rendering; And another one - rendering has finished

        Ok(Self{entry, instance, data, device})
    }

    // Renders a frame
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        // Acquire an image from the swapchain
        // Execute the command buffer with that image as attachment in the framebuffer
        // Return the image to the swapchain for presentation

        let image_index = self
            .device
            .acquire_next_image_khr(
                self.data.swapchain, 
                u64::max_value(), 
                self.data.image_available_semaphore, 
                vk::Fence::null()
            )?.0 as usize;

        let wait_semaphores = &[self.data.image_available_semaphore];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = &[self.data.command_buffers[image_index as usize]];
        let signal_semaphores = &[self.data.image_finished_semaphore];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores);

        self.device.queue_submit(
            self.data.graphics_queue, 
            &[submit_info], 
            vk::Fence::null())?;

        Ok(())
    }

    // Destroys Vulkan app
    unsafe fn destroy(&mut self) {
        // Manually destroying all things we used
        self.device.destroy_semaphore(self.data.image_available_semaphore, None);
        self.device.destroy_semaphore(self.data.image_finished_semaphore, None);
        self.device.destroy_command_pool(self.data.command_pool, None);

        self.data.framebuffers
            .iter()
            .for_each(|f| self.device.destroy_framebuffer(*f, None));

        self.device.destroy_pipeline(self.data.pipeline, None);
        self.device.destroy_pipeline_layout(self.data.pipeline_layout, None);
        self.device.destroy_render_pass(self.data.render_pass, None);

        self.data.swapchain_image_views
            .iter()
            .for_each(|iv| self.device.destroy_image_view(*iv, None));

        self.device.destroy_swapchain_khr(self.data.swapchain, None);
        self.device.destroy_device(None);
        self.instance.destroy_surface_khr(self.data.surface, None);
        self.instance.destroy_instance(None);
    }
}

// The Vulkan handles and associated properties used by Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {
    surface: vk::SurfaceKHR,
    physical_device: vk::PhysicalDevice,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<vk::Image>,
    swapchain_image_views: Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    framebuffers: Vec<vk::Framebuffer>,
    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,
    image_available_semaphore: vk::Semaphore,
    image_finished_semaphore: vk::Semaphore,
}



unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance> {
    let app_info = vk::ApplicationInfo::builder()
        .application_name(b"Rust Vulkan\0")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"No engine\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));

    // Collect the supported instance layers into a HashSet
    let available_layers = entry
        .enumerate_instance_layer_properties()?
        .iter()
        .map(|l| l.layer_name)
        .collect::<HashSet<_>>();

    // Check if the validation layer is available
    if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
        return Err(anyhow!("Validation layer requested but not supported."));
    }


    // No ternary operator ?: in Rust
    // Create a list of layer names containing the validation layer
    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {Vec::new()};

    
    // Enumerate the required global extensions and convert them into null-terminated C strings (*const c_char)
    let extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    let info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions);

    Ok(entry.create_instance(&info, None)?)

    // The general pattern that object creation function parameters in Vulkan follow is:
    //  - Reference to struct with creation info
    //  - Optional reference to custom allocator callbacks

    // When we call our create_instance function, what we get back is not a raw Vulkan instance 
    // as would be returned by the Vulkan command `vkCreateInstance (vk::Instance)`. 
    // Instead what we got back is a custom type defined by `vulkanalia` which combines both 
    // a raw Vulkan instance and the commands loaded for that specific instance.
}



#[derive(Debug, Error)]
#[error("Missing {0}.")]
pub struct SuitabilityError(pub &'static str);
unsafe fn pick_physical_device(instance: &Instance, data: &mut AppData) -> Result<()> {
    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);

        if let Err(error) = check_physical_device(instance, data, physical_device) {
            warn!("Skipping physical device (`{}`): {}", properties.device_name, error);
        } else {
            info!("Selected physical device (`{}`).", properties.device_name);
            data.physical_device = physical_device;
            return Ok(());
        }
    }

    Err(anyhow!("Failed to find suitable physical device."))
}

unsafe fn check_physical_device(instance: &Instance, data: &AppData, physical_device: vk::PhysicalDevice) -> Result<()> {
    QueueFamilyIndices::get(instance, data, physical_device)?;
    check_physical_device_extensions(instance, physical_device)?;

    let support = SwapchainSupport::get(instance, data, physical_device)?;
    if support.formats.is_empty() || support.present_modes.is_empty() {
        return Err(anyhow!(SuitabilityError("Insufficient swapchain support.")));
    } 
    
    Ok(())
}

unsafe fn check_physical_device_extensions(instance: &Instance, physical_device: vk::PhysicalDevice) -> Result<()> {
    let extensions = instance
        .enumerate_device_extension_properties(physical_device, None)?
        .iter()
        .map(|e| e.extension_name)
        .collect::<HashSet<_>>();

    if DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {Ok(())}
    else {Err(anyhow!(SuitabilityError("Missing required device extensions.")))}
}

unsafe fn create_logical_device(entry: &Entry, instance: &Instance, data: &mut AppData) -> Result<Device> {
    // This structure describes the number of queues we want for a single queue family. 
    // Right now we're only interested in a queue with graphics capabilities.
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

    // Unique queue families that are necessary for the required queues
    let mut unique_indices = HashSet::new();
    unique_indices.insert(indices.graphics);
    unique_indices.insert(indices.present);

    let queue_priorities = &[1.0];
    let queue_infos = unique_indices
        .iter()
        .map(|i| {
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(*i)
                .queue_priorities(queue_priorities)
        })
        .collect::<Vec<_>>();

    // We wont be enabling any device extensions yet, 
    // so we will just construct a list of layer names 
    // containing the validation layer if validation is enabled.
    let layers = if VALIDATION_ENABLED {vec
        ![VALIDATION_LAYER.as_ptr()]
    } else {vec![]};

    // Available extensions
    let extensions = DEVICE_EXTENSIONS
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    // Set of device features that we'll be using
    let features = vk::PhysicalDeviceFeatures::builder();

    let info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&features);
    let device = instance.create_device(data.physical_device, &info, None)?;

    data.graphics_queue = device.get_device_queue(indices.graphics, 0);
    data.present_queue = device.get_device_queue(indices.present, 0);

    Ok(device)
}



#[derive(Copy, Clone, Debug)]
struct QueueFamilyIndices {
    graphics: u32,
    present: u32, 
}
impl QueueFamilyIndices {
    unsafe fn get(instance: &Instance, data: &AppData, physical_device: vk::PhysicalDevice) -> Result<Self> {
        let properties = instance
            .get_physical_device_queue_family_properties(physical_device);

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        let mut present = None;
        for (index, props) in properties.iter().enumerate() {
            if instance.get_physical_device_surface_support_khr(physical_device, index as u32, data.surface)? {
                present = Some(index as u32);
                break;
            }
        }

        if let (Some(graphics), Some(present)) = (graphics, present) {
            Ok(Self{graphics, present})
        } else {Err(anyhow!(SuitabilityError("Missing required queue families.")))}
    }
}



#[derive(Clone, Debug)]
struct SwapchainSupport {
    capabilities: vk::SurfaceCapabilitiesKHR,
    formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
}
impl SwapchainSupport {
    unsafe fn get(instance: &Instance, data: &AppData, physical_device: vk::PhysicalDevice) -> Result<Self> {
        Ok(Self{
            capabilities: instance
                .get_physical_device_surface_capabilities_khr(physical_device, data.surface)?,
            formats: instance
                .get_physical_device_surface_formats_khr(physical_device, data.surface)?,
            present_modes: instance
                .get_physical_device_surface_present_modes_khr(physical_device, data.surface)?,
        })
    }
}

fn get_swapchain_surface_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
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

fn get_swapchain_present_mode(present_modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
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

fn get_swapchain_extent(window: &Window, capabilities: vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
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

unsafe fn create_swapchain(window: &Window, instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;
    let support = SwapchainSupport::get(instance, data, data.physical_device)?;

    let surface_format = get_swapchain_surface_format(&support.formats);
    let present_mode = get_swapchain_present_mode(&support.present_modes);
    let extent = get_swapchain_extent(window, support.capabilities);

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

unsafe fn create_swapchain_image_views(device: &Device, data: &mut AppData) -> Result<()> {
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



unsafe fn create_pipeline(device: &Device, data: &mut AppData) -> Result<()> {
    let vert = include_bytes!("shaders/vert.spv");
    let frag = include_bytes!("shaders/frag.spv");

    let vert_shader_module = create_shader_module(device, &vert[..])?;
    let frag_shader_module = create_shader_module(device, &frag[..])?;


    // The first step is telling Vulkan in which pipeline stage the shader is going to be used. 
    // There is a variant for each of the programmable stages
    let vert_stage = vk::PipelineShaderStageCreateInfo::builder()
        .stage(vk::ShaderStageFlags::VERTEX)
        .module(vert_shader_module)
        .name(b"main\0");

    let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
        .stage(vk::ShaderStageFlags::FRAGMENT)
        .module(frag_shader_module)
        .name(b"main\0");


    // vk::PrimitiveTopology::POINT_LIST     – points from vertices
    // vk::PrimitiveTopology::LINE_LIST      – line from every 2 vertices without reuse
    // vk::PrimitiveTopology::LINE_STRIP     – the end vertex of every line is used as start vertex for the next line
    // vk::PrimitiveTopology::TRIANGLE_LIST  – triangle from every 3 vertices without reuse
    // vk::PrimitiveTopology::TRIANGLE_STRIP – the second and third vertex of every triangle are used as first two vertices of the next triangle

    let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder();     // Leaveing it at default
    let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false);


    // Viewports define the transformation from the image to the framebuffer 
    // Scissor rectangles define in which regions pixels will actually be stored.
    let viewport = vk::Viewport::builder()
        .x(0.0).y(0.0)
        .width(data.swapchain_extent.width as f32)
        .height(data.swapchain_extent.height as f32)
        .max_depth(0.0)
        .max_depth(1.0);

    let scissor = vk::Rect2D::builder()
        .offset(vk::Offset2D{x: 0, y: 0})
        .extent(data.swapchain_extent);

    let viewports = &[viewport];
    let scissors  = &[scissor];
    let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
        .viewports(viewports)
        .scissors(scissors);


    // vk::PolygonMode::FILL  – fill the area of the polygon with fragments
    // vk::PolygonMode::LINE  – polygon edges are drawn as lines
    // vk::PolygonMode::POINT – polygon vertices are drawn as points
    // Using any mode other than fill requires enabling a GPU feature.
    let rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(vk::PolygonMode::FILL)
        .line_width(1.0)    // Line thicker than 1.0 requires to enable the `wide_lines` GPU feature.
        .cull_mode(vk::CullModeFlags::BACK)
        .front_face(vk::FrontFace::CLOCKWISE)
        .depth_bias_enable(false);


    // MSAA
    // Enabling it requires enabling a GPU feature.
    let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
        .sample_shading_enable(false)
        .rasterization_samples(vk::SampleCountFlags::_1);


    // Color blending
    let attachment = vk::PipelineColorBlendAttachmentState::builder()
        .color_write_mask(vk::ColorComponentFlags::all())
        .blend_enable(false);
    let attachments = &[attachment];
    let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
        .logic_op_enable(false)
        .logic_op(vk::LogicOp::COPY)
        .attachments(attachments)
        .blend_constants([0.0, 0.0, 0.0, 0.0]);

    let dynamic_states = &[
        vk::DynamicState::VIEWPORT,
        vk::DynamicState::LINE_WIDTH
    ];
    let dynamic_state = vk::PipelineDynamicStateCreateInfo::builder()
        .dynamic_states(dynamic_states);

    let layout_info = vk::PipelineLayoutCreateInfo::builder();
    data.pipeline_layout = device.create_pipeline_layout(&layout_info, None)?;


    // There are actually two more parameters: 
    // `base_pipeline_handle` and `base_pipeline_index`. 
    // Vulkan allows to create a new graphics pipeline by deriving from an existing pipeline. 
    let stages = &[vert_stage, frag_stage];
    let info = vk::GraphicsPipelineCreateInfo::builder()
        .stages(stages)
        .vertex_input_state(&vertex_input_state)
        .input_assembly_state(&input_assembly_state)
        .viewport_state(&viewport_state)
        .rasterization_state(&rasterization_state)
        .multisample_state(&multisample_state)
        .color_blend_state(&color_blend_state)
        .layout(data.pipeline_layout)
        .render_pass(data.render_pass)
        .subpass(0);


    // `vk::GraphicsPipelineCreateInfo` is designed to take and create multiple vk::Pipeline objects in a single call.
    data.pipeline = device.create_graphics_pipelines(
        vk::PipelineCache::null(), &[info], None)?.0;


    device.destroy_shader_module(vert_shader_module, None);
    device.destroy_shader_module(frag_shader_module, None);

    Ok(())
}

unsafe fn create_shader_module(device: &Device, bytecode: &[u8]) -> Result<vk::ShaderModule> {
    // We only need to specify the length of our bytecode slice and the bytecode slice itself. 
    // The size of the bytecode is specified in bytes, but the bytecode slice expected by this struct is a `&[u32]` instead of a `&[u8]`
    let bytecode = Vec::<u8>::from(bytecode);   // We need to realign from u8 to u32, it may not fit in original array
    let (prefix, code, suffix) = bytecode.align_to::<u32>();

    if !prefix.is_empty() || !suffix.is_empty() {return Err(anyhow!("Shader bytecode is not properly aligned."));}

    // The middle slice returned by this method (code) is a &[u32] and is guaranteed to be correctly aligned. 
    // Any u8s in our bytecode slice that fell outside this alignment guarantee will appear in the first or third slices returned (prefix and suffix). 
    let info = vk::ShaderModuleCreateInfo::builder()
        .code_size(bytecode.len())
        .code(code);

    Ok(device.create_shader_module(&info, None)?)
}

unsafe fn create_render_pass(instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
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

    let attachments = &[color_attachment];
    let subpasses = &[subpass];
    let info = vk::RenderPassCreateInfo::builder()
        .attachments(attachments)
        .subpasses(subpasses);

    data.render_pass = device.create_render_pass(&info, None)?;
    Ok(())
}



unsafe fn create_framebuffer(device: &Device, data: &mut AppData) -> Result<()> {
    data.framebuffers = data
        .swapchain_image_views
        .iter()
        .map(|i| {
            let attachments = &[*i];
            let create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(data.render_pass)
                .attachments(attachments)
                .width(data.swapchain_extent.width)
                .height(data.swapchain_extent.height)
                .layers(1);

            device.create_framebuffer(&create_info, None)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}



unsafe fn create_command_pool(instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
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

unsafe fn create_command_buffer(device: &Device, data: &mut AppData) -> Result<()> {
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



unsafe fn create_sync_objects(device: &Device, data: &mut AppData) -> Result<()> {
    let semaphore_info = vk::SemaphoreCreateInfo::builder();

    data.image_available_semaphore = device.create_semaphore(&semaphore_info, None)?;
    data.image_finished_semaphore  = device.create_semaphore(&semaphore_info, None)?;    

    Ok(())
}