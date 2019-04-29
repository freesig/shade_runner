#[derive(Debug)]
pub enum Error {
    Compile(CompileError),
    Layout(ConvertError),
    LoadingData(String),
    FileWatch(notify::Error),
}

#[derive(Debug)]
pub enum ConvertError {
    Unimplemented,
}

#[derive(Debug)]
pub enum CompileError {
    Compile(shaderc::Error),
    Open(std::io::Error),
    InvalidPath,
    CreateCompiler,
}
