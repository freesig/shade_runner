use crate::error::CompileError;
use shaderc::{IncludeType, ResolvedInclude};
use shaderc::{ShaderKind, CompileOptions};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn compile<T>(path: T, shader_kind: ShaderKind) -> Result<Vec<u32>, CompileError>
where
    T: AsRef<Path>,
{
    // TODO Probably shouldn't create this every time.
    let mut compiler = shaderc::Compiler::new().ok_or(CompileError::CreateCompiler)?;
    let mut options = CompileOptions::new().ok_or(CompileError::CreateCompiler)?;
    let mut f = File::open(&path).map_err(CompileError::Open)?;
    let mut src = String::new();
    f.read_to_string(&mut src).map_err(CompileError::Open)?;
    options.set_include_callback(|path, include_type, folder_path, depth| {
        get_include(path, include_type, folder_path, depth)
    });
    let result = compiler
        .compile_into_spirv(
            src.as_str(),
            shader_kind,
            path.as_ref().to_str().ok_or(CompileError::InvalidPath)?,
            "main",
            Some(&options),
        )
        .map_err(CompileError::Compile)?;
    let data = result.as_binary();
    Ok(data.to_owned())
}

fn get_include(
    path: &str,
    include_type: IncludeType,
    _folder_path: &str,
    _depth: usize,
) -> Result<ResolvedInclude, String> {
    match include_type {
        IncludeType::Relative => {
            let p = Path::new(path);
            if !p.is_file() {
                return Err("Include doesn't point to file".to_string());
            }
            let resolved_name = p
                .to_str()
                .ok_or("Path has invalid characters".to_string())?
                .to_owned();
            let p = p.canonicalize().map_err(|_|"Failed to parse include path".to_string())?;
            let mut content = String::new();
            File::open(p)
                .map_err(|_|"Couldn't open include directory".to_string())?
                .read_to_string(&mut content)
                .map_err(|_|"Failed to read included shader".to_string())?;
            Ok(ResolvedInclude {
                resolved_name,
                content,
            })
        }
        IncludeType::Standard => Err("Standard includes are unimplemented".to_string()),
    }
}
