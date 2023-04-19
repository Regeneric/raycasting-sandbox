use vulkanalia::prelude::v1_0::*;

use super::{anyhow, Result, Ok};

pub struct ShaderModule {}
impl ShaderModule {
    pub unsafe fn create_shader_module(device: &Device, bytecode: &[u8]) -> Result<vk::ShaderModule> {
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
}