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
use vulkanalia::vk::ExtDebugUtilsExtension;
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

        Ok(Self{entry, instance, data, device})     // It's like in JS: `const a = 1; const obj {a};`  
                                                    // Instead of `const a = 1; const obj {a: a};`
    }

    // Renders a frame
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    // Destroys Vulkan app
    unsafe fn destroy(&mut self) {
        // Manually destroying all things we used
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