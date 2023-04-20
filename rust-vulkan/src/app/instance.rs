use vulkanalia::prelude::v1_0::*;

use super::{anyhow, Result};
use super::vk_window;
use super::Window;
use super::HashSet;

use super::VALIDATION_ENABLED;
use super::VALIDATION_LAYER;


pub struct AppInstance {}
impl AppInstance {
    pub unsafe fn new(window: &Window, entry: &Entry) -> Result<Instance> {
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
}