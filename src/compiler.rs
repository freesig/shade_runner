use std::fs::File;
use std::path::Path;
use shaderc::ShaderKind;
use std::io::Read;
use crate::error::CompileError;

pub fn compile<T>(path: T, shader_kind: ShaderKind) -> Result<Vec<u32>, CompileError>
where
    T: AsRef<Path>,
{
    // TODO Probably shouldn't create this every time.
    let mut compiler = shaderc::Compiler::new().ok_or(CompileError::CreateCompiler)?;
    let mut f = File::open(&path).map_err(CompileError::Open)?;
    let mut src = String::new();
    f.read_to_string(&mut src).map_err(CompileError::Open)?;
    let result = compiler.compile_into_spirv(
        src.as_str(),
        shader_kind,
        path.as_ref().to_str().ok_or(CompileError::InvalidPath)?,
        "main",
        None,
    ).map_err(CompileError::Compile)?;
    let data = result.as_binary();
    Ok(data.to_owned())
}
