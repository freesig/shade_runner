mod compiler;
pub mod error;
mod reflection;
mod srvk;
pub mod layouts;
mod watch;

pub use layouts::*;
pub use reflection::LayoutData;
pub use watch::{Message, Watch};
pub use error::*;

use spirv_reflect as sr;
use vulkano as vk;
use std::path::Path;
use shaderc::ShaderKind;

pub struct CompiledShaders {
    pub vertex: Vec<u32>,
    pub fragment: Vec<u32>,
    pub compute: Vec<u32>,
}

/// Loads and compiles the vertex and fragment GLSL shaders from files
pub fn load<T>(vertex: T, fragment: T) -> Result<CompiledShaders, Error>
where
    T: AsRef<Path>,
{
    let vertex = compiler::compile(vertex, ShaderKind::Vertex).map_err(Error::Compile)?;
    let fragment = compiler::compile(fragment, ShaderKind::Fragment).map_err(Error::Compile)?;
    Ok(CompiledShaders{ vertex, fragment, compute: Vec::new() })
}

// TODO this should be incorpoarted into load but that would be
// a breaking change. Do this in next major version
pub fn load_compute<T>(compute: T) -> Result<CompiledShaders, Error>
where
    T: AsRef<Path>,
{
    let compute = compiler::compile(compute, ShaderKind::Compute).map_err(Error::Compile)?;
    Ok(CompiledShaders{ vertex: Vec::new(), fragment: Vec::new(), compute })
}

pub fn parse_compute(code: &CompiledShaders) -> Result<Entry, Error> {
    reflection::create_compute_entry(code)
}

/// Parses the shaders and gives an entry point
pub fn parse(code: &CompiledShaders) -> Result<Entry, Error> {
    reflection::create_entry(code)
}
