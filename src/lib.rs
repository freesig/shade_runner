mod compiler;
mod error;
mod reflection;
mod srvk;
pub mod layouts;
mod watch;

pub use layouts::*;
pub use reflection::LayoutData;
pub use watch::{Message, Watch};
pub use error::{Error, ConvertError};

use spirv_reflect as sr;
use vulkano as vk;
use std::path::Path;
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

pub fn parse(code: &CompiledShaders) -> Result<Entry, Error> {
    reflection::create_entry(code)
}
