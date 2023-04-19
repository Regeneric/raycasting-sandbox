use vulkanalia::prelude::v1_0::*;

mod shader_module;
use crate::app::pipeline::shader_module::ShaderModule;

use super::AppData;
use anyhow::{anyhow, Result, Ok};

pub struct AppPipeline {}
impl AppPipeline {
    pub unsafe fn create_pipeline(device: &Device, data: &mut AppData) -> Result<()> {
        let vert = include_bytes!("../shaders/vert.spv");
        let frag = include_bytes!("../shaders/frag.spv");
    
        let vert_shader_module = ShaderModule::create_shader_module(device, &vert[..])?;
        let frag_shader_module = ShaderModule::create_shader_module(device, &frag[..])?;
    
    
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
}