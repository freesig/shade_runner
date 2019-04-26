#[derive(Debug)]
pub enum Error {
    Compile(shaderc::Error),
}
