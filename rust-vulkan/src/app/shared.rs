use vulkanalia :: prelude :: v1_0 :: *;

use super::AppData;
use super::Result;
use super::anyhow;

pub struct Commons {}
impl Commons {
    pub unsafe fn get_memory_type_index(instance: &Instance, data: &AppData,
                                  properties: vk::MemoryPropertyFlags,
                                  requirements: vk::MemoryRequirements) -> Result<u32> {


        // If there is a memory type suitable for the buffer 
        // that also has all of the properties we need, 
        // then we return its index, otherwise we return an error.
        let memory = instance.get_physical_device_memory_properties(data.physical_device);
        (0..memory.memory_type_count)
            .find(|i| {
                let suitable = (requirements.memory_type_bits & (1<<i)) != 0;
                let memory_type = memory.memory_types[*i as usize];
                suitable && memory_type.property_flags.contains(properties)
            })
            .ok_or_else(|| anyhow!("Failed to find suitable memory type."))
    }
}