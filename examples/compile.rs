use shade_runner as sr;
use std::path::PathBuf;

fn main() {
    let project_root = std::env::current_dir().expect("failed to get root directory");
    let mut vert_path = project_root.clone();
    vert_path.push(PathBuf::from("examples/shaders/vert.glsl"));
    let mut frag_path = project_root.clone();
    frag_path.push(PathBuf::from("examples/shaders/frag.glsl"));
    let shader = sr::load(vert_path, frag_path).expect("Failed to compile");
    let vulkano_entry = sr::parse(&shader);
    dbg!(vulkano_entry);
}
