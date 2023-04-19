pub mod queue_family_indices;
pub mod swapchain_support;

use crate :: app :: device :: queue_family_indices :: QueueFamilyIndices;
use crate :: app :: device :: swapchain_support    :: SwapchainSupport;

use vulkanalia::prelude::v1_0::*;
use log::*;

use super::Error;
use super::AppData;
use super::{anyhow, Result, Ok};
use super::HashSet;

use super::DEVICE_EXTENSIONS;
use super::VALIDATION_ENABLED;
use super::VALIDATION_LAYER;


#[derive(Debug, Error)]
#[error("Missing {0}.")]
pub struct SuitabilityError(pub &'static str);
pub struct AppDevice {}
impl AppDevice {
    pub unsafe fn pick_physical_device(instance: &Instance, data: &mut AppData) -> Result<()> {
        for physical_device in instance.enumerate_physical_devices()? {
            let properties = instance.get_physical_device_properties(physical_device);

            if let Err(error) = Self::check_physical_device(instance, data, physical_device) {
                warn!("Skipping physical device (`{}`): {}", properties.device_name, error);
            } else {
                info!("Selected physical device (`{}`).", properties.device_name);
                data.physical_device = physical_device;
                return Ok(());
            }
        }
    
        Err(anyhow!("Failed to find suitable physical device."))
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

    unsafe fn check_physical_device(instance: &Instance, data: &AppData, physical_device: vk::PhysicalDevice) -> Result<()> {
        QueueFamilyIndices::get(instance, data, physical_device)?;
        Self::check_physical_device_extensions(instance, physical_device)?;
    
        let support = SwapchainSupport::get(instance, data, physical_device)?;
        if support.formats.is_empty() || support.present_modes.is_empty() {
            return Err(anyhow!(SuitabilityError("Insufficient swapchain support.")));
        } 

        Ok(())
    }

    pub unsafe fn create_logical_device(entry: &Entry, instance: &Instance, data: &mut AppData) -> Result<Device> {
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
}