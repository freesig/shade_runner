use color_backtrace;
use difference::{Changeset, Difference};
use shade_runner::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use vulkano::descriptor::descriptor::ShaderStages;
use vulkano::format::*;
use vulkano::pipeline::shader::ShaderInterfaceDefEntry;

fn setup() {
    color_backtrace::install();
}

fn difference(e: &str, t: &str) -> String {
    let diffs = Changeset::new(&e, &t, "");
    diffs
        .diffs
        .iter()
        .filter(|d| match d {
            Difference::Add(_) => true,
            Difference::Rem(_) => true,
            _ => false,
        })
        .map(|d| match d {
            Difference::Add(a) => format!("add: {}", a),
            Difference::Rem(a) => format!("remove: {}", a),
            _ => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn parse<T>(vertex: T, fragment: T) -> shade_runner::Entry
where
    T: AsRef<Path>,
{
    let project_root = std::env::current_dir().expect("failed to get root directory");
    let mut path = project_root.clone();
    path.push(PathBuf::from("tests/shaders/"));
    let mut vertex_path = path.clone();
    vertex_path.push(vertex);
    let mut fragment_path = path.clone();
    fragment_path.push(fragment);
    let shader = shade_runner::load(vertex_path, fragment_path).expect("Failed to compile");
    shade_runner::parse(&shader)
}

#[test]
fn test_shade1() {
    setup();
    let target = Entry {
        frag_input: FragInput { inputs: Vec::new() },
        frag_output: FragOutput {
            outputs: vec![ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32B32A32Sfloat,
                name: Some(Cow::Borrowed("f_color")),
            }],
        },
        frag_layout: FragLayout {
            stages: ShaderStages {
                fragment: true,
                ..ShaderStages::none()
            },
            layout_data: LayoutData {
                num_sets: 0,
                num_bindings: HashMap::new(),
            },
        },
        vert_input: VertInput {
            inputs: vec![ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32Sfloat,
                name: Some(Cow::Borrowed("position")),
            }],
        },
        vert_output: VertOutput {
            outputs: Vec::new(),
        },
        vert_layout: VertLayout(ShaderStages {
            vertex: true,
            ..ShaderStages::none()
        }),
    };
    let entry = parse("vert1.glsl", "frag1.glsl");
    let entry = format!("{:?}", entry);
    let target = format!("{:?}", target);
    assert_eq!(
        &entry,
        &target,
        "\n\nDifference: {}",
        difference(&entry, &target)
    );
}

#[test]
fn test_shade2() {
    setup();
    let target = Entry {
        frag_input: FragInput { 
            inputs: vec![
                ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32B32A32Sfloat,
                name: Some(Cow::Borrowed("cool")),
            },
                ShaderInterfaceDefEntry {
                location: 1..2,
                format: Format::R32G32Sfloat,
                name: Some(Cow::Borrowed("yep")),
            },
                ShaderInterfaceDefEntry {
                location: 2..3,
                format: Format::R32Sfloat,
                name: Some(Cow::Borrowed("monkey")),
            },
            ],
        },
        frag_output: FragOutput {
            outputs: vec![ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32B32A32Sfloat,
                name: Some(Cow::Borrowed("f_color")),
            }],
        },
        frag_layout: FragLayout {
            stages: ShaderStages {
                fragment: true,
                ..ShaderStages::none()
            },
            layout_data: LayoutData {
                num_sets: 0,
                num_bindings: HashMap::new(),
            },
        },
        vert_input: VertInput {
            inputs: vec![ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32Sfloat,
                name: Some(Cow::Borrowed("position")),
            }],
        },
        vert_output: VertOutput {
            outputs: vec![
                ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32B32A32Sfloat,
                name: Some(Cow::Borrowed("cool")),
            },
                ShaderInterfaceDefEntry {
                location: 1..2,
                format: Format::R32G32Sfloat,
                name: Some(Cow::Borrowed("yep")),
            },
                ShaderInterfaceDefEntry {
                location: 2..3,
                format: Format::R32Sfloat,
                name: Some(Cow::Borrowed("monkey")),
            },
            ],
        },
        vert_layout: VertLayout(ShaderStages {
            vertex: true,
            ..ShaderStages::none()
        }),
    };
    let entry = parse("vert2.glsl", "frag2.glsl");
    let entry = format!("{:?}", entry);
    let target = format!("{:?}", target);
    assert_eq!(
        &entry,
        &target,
        "\n\nDifference: {}",
        difference(&entry, &target)
    );
}

