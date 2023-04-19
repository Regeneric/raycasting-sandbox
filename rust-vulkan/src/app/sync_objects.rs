use vulkanalia::prelude::v1_0::*;

use super::Device;
use super::AppData;
use super::Result;

use super::MAX_FRAMES_IN_FLIGHT;

pub struct AppSyncObjects {}
impl AppSyncObjects {
    pub unsafe fn create_sync_objects(device: &Device, data: &mut AppData) -> Result<()> {
        let semaphore_info = vk::SemaphoreCreateInfo::builder();
        let fence_info = vk::FenceCreateInfo::builder()
            .flags(vk::FenceCreateFlags::SIGNALED);
    
        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            data.image_available_semaphore.push(device.create_semaphore(&semaphore_info, None)?);
            data.image_finished_semaphore.push(device.create_semaphore(&semaphore_info, None)?);
        
            data.in_flight_fences.push(device.create_fence(&fence_info, None)?);
        }
    
        data.images_in_flight = data.swapchain_images
            .iter()
            .map(|_| vk::Fence::null())
            .collect();
    
        Ok(())
    }
}