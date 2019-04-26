use std::fs::File;
use std::path::Path;
use shaderc::ShaderKind;
use std::io::Read;

pub fn compile<T>(path: T, shader_kind: ShaderKind) -> shaderc::Result<Vec<u32>>
where
    T: AsRef<Path>,
{
    // TODO Probably shouldn't create this every time.
    let mut compiler = shaderc::Compiler::new().expect("failed to create compiler");
    let mut f = File::open(&path).expect("failed to open shader src");
    let mut src = String::new();
    f.read_to_string(&mut src).expect("failed to read src");
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.add_macro_definition("EP", Some("main"));
    let result = compiler.compile_into_spirv(
        src.as_str(),
        shader_kind,
        path.as_ref().to_str().expect("failed to make path string"),
        "main",
        None,
    )?;
    let data = result.as_binary();
    Ok(data.to_owned())
}
