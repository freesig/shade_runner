use color_backtrace;
use difference::{Changeset, Difference};
use shade_runner::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use vulkano::descriptor::descriptor::*;
use vulkano::format::*;
use vulkano::pipeline::shader::ShaderInterfaceDefEntry;
use vulkano::descriptor::pipeline_layout::PipelineLayoutDesc;

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

fn descriptor_layout<T>(desc: &T) -> String 
where
T: PipelineLayoutDesc,
{
    let num_sets = desc.num_sets();
    let mut r = format!("{:?}", num_sets);
    for n in 0..num_sets {
        let num_bindings = desc.num_bindings_in_set(n);
        r = format!("{:?}{:?}", r, num_bindings);
        for b in num_bindings {
            r = format!("{:?}{:?}", r, desc.descriptor(n, b));
        }
    }
    let num_push_constants = desc.num_push_constants_ranges();
    r = format!("{:?}{:?}", r, num_push_constants);
    for i in 0..num_push_constants {
        r = format!("{:?}{:?}", r, desc.push_constants_range(i));
    }
    r
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
                descriptions: HashMap::new(),
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
                descriptions: HashMap::new(),
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

#[test]
fn test_shade3() {
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
                num_sets: 1,
                num_bindings: vec![(0, 1)].into_iter().collect(),
                descriptions: vec![(
                    0,
                    vec![(
                        0,
                        DescriptorDesc {
                            ty: DescriptorDescTy::CombinedImageSampler(DescriptorImageDesc {
                                sampled: true,
                                dimensions: DescriptorImageDescDimensions::TwoDimensional,
                                format: None,
                                multisampled: false,
                                array_layers: DescriptorImageDescArray::NonArrayed,
                            }),
                            array_count: 1,
                            stages: ShaderStages {
                                fragment: true,
                                ..ShaderStages::none()
                            },
                            readonly: true,
                        },
                    )]
                    .into_iter()
                    .collect(),
                )]
                .into_iter()
                .collect(),
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
    let entry = parse("vert3.glsl", "frag3.glsl");
    do_test(&entry.frag_input, &target.frag_input);
    do_test(&entry.frag_output, &target.frag_output);
    do_test(&entry.vert_input, &target.vert_input);
    do_test(&entry.vert_output, &target.vert_output);
    do_test(&descriptor_layout(&entry.frag_layout), &descriptor_layout(&target.frag_layout));
    do_test(&descriptor_layout(&entry.vert_layout), &descriptor_layout(&target.vert_layout));

}

fn do_test<T>(a: &T, b: &T) 
where
T: std::fmt::Debug,
{
    let a = format!("{:?}", a);
    let b = format!("{:?}", b);
    assert_eq!(
        &a,
        &b,
        "\n\nDifference: {}",
        difference(&a, &b)
    );
}
