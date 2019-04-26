mod compiler;
mod error;
mod reflection;
mod srvk;
mod layouts;

pub use layouts::*;
pub use reflection::LayoutData;

use spirv_reflect as sr;
use vulkano as vk;
use std::path::Path;
use error::Error;
use shaderc::ShaderKind;

pub struct CompiledShaders {
    pub vertex: Vec<u32>,
    pub fragment: Vec<u32>,
}

pub fn load<T>(vertex: T, fragment: T) -> Result<CompiledShaders, Error>
where
    T: AsRef<Path>,
{
    let vertex = compiler::compile(vertex, ShaderKind::Vertex).map_err(|e| Error::Compile(e))?;
    let fragment = compiler::compile(fragment, ShaderKind::Fragment).map_err(|e| Error::Compile(e))?;
    Ok(CompiledShaders{ vertex, fragment })
}

pub fn parse(code: &CompiledShaders) -> Entry {
    reflection::create_entry(code)
}
