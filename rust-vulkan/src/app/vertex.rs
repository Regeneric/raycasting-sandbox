use super::AppData;
use super::Commons;
use super::Result;

use std        :: mem     :: size_of;
use vulkanalia :: prelude :: v1_0 :: *;
use std        :: ptr     :: copy_nonoverlapping as memcpy;

use nalgebra_glm as glm;
use lazy_static  :: lazy_static;


// This is known as interleaving vertex attributes.
lazy_static! {
    static ref VERTICES: Vec<AppVertex> = vec![
        AppVertex::new(glm::vec2( 0.0, -0.5), glm::vec3(1.0, 0.0, 0.0)),
        AppVertex::new(glm::vec2( 0.5,  0.5), glm::vec3(0.0, 1.0, 0.0)),
        AppVertex::new(glm::vec2(-0.5,  0.5), glm::vec3(0.0, 0.0, 1.0)),
    ];
}


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AppVertex{
    pos:   glm::Vec2,
    color: glm::Vec3,
}

impl AppVertex {
    pub fn new(pos: glm::Vec2, color: glm::Vec3) -> Self {
        Self{pos, color}
    }
    
    pub fn binding_description() -> vk::VertexInputBindingDescription {
        // All of our per-vertex data is packed together in one array, so we're only going to have one binding
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(size_of::<AppVertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        let pos = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(0)
            .build();

        let color = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(size_of::<glm::Vec2>() as u32)
            .build();

        [pos, color]
    }

    pub unsafe fn create_vertex_buffer(instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
        let buffer_info = vk::BufferCreateInfo::builder()
            .size((size_of::<AppVertex>() * VERTICES.len()) as u64)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .flags(vk::BufferCreateFlags::empty());

        data.vertex_buffer = device.create_buffer(&buffer_info, None)?;

        let requirements = device.get_buffer_memory_requirements(data.vertex_buffer);
        let memory_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(Commons::get_memory_type_index(instance, data, 
                                                        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE, 
                                                        requirements)?);
        
        data.vertex_buffer_memory = device.allocate_memory(&memory_info, None)?;
        device.bind_buffer_memory(data.vertex_buffer, data.vertex_buffer_memory, 0)?;
        
        // Mapping the buffer memory into CPU accessible memory
        let memory = device.map_memory(data.vertex_buffer_memory, 
                                       0, buffer_info.size, 
                                       vk::MemoryMapFlags::empty())?;

        memcpy(VERTICES.as_ptr(), memory.cast(), VERTICES.len());
        device.unmap_memory(data.vertex_buffer_memory);

        Ok(())
    }
}