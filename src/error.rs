#[derive(Debug)]
pub enum Error {
    Compile(shaderc::Error),
    Layout(ConvertError),
}

#[derive(Debug)]
pub enum ConvertError {
    Unimplemented,
}
