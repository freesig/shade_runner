use crate::sr; 
use crate::srvk::{SpirvTy,DescriptorDescInfo};
use std::borrow::Cow;
use crate::vk::pipeline::shader::ShaderInterfaceDefEntry;
use crate::vk::descriptor::descriptor::*;
use std::collections::HashMap;
use crate::CompiledShaders;
use crate::layouts::*;

pub struct ShaderInterfaces {
    pub inputs: Vec<ShaderInterfaceDefEntry>,
    pub outputs: Vec<ShaderInterfaceDefEntry>,
}

#[derive(Debug, Clone, Default)]
pub struct LayoutData {
    pub num_sets: usize,
    pub num_bindings: HashMap<usize, usize>,
    pub descriptions: HashMap<usize, HashMap<usize, DescriptorDesc>>,
}

pub fn create_entry(shaders: &CompiledShaders) -> Entry {
    let vertex_interfaces = create_interfaces(&shaders.vertex);
    let fragment_interfaces = create_interfaces(&shaders.fragment);
    let fragment_layout = create_layouts(&shaders.fragment);
    let frag_input = FragInput{ inputs: fragment_interfaces.inputs };
    let frag_output = FragOutput{ outputs: fragment_interfaces.outputs };
    let frag_layout = FragLayout {
        stages: ShaderStages {
                fragment: true,
                ..ShaderStages::none()
        },
        layout_data: fragment_layout,
    };
    let vert_input = VertInput{ inputs: vertex_interfaces.inputs };
    let vert_output = VertOutput{ outputs: vertex_interfaces.outputs };
    let vert_layout = VertLayout(ShaderStages {
                vertex: true,
                ..ShaderStages::none()
            });
    Entry {
        frag_input,
        frag_output,
        vert_input,
        vert_output,
        frag_layout,
        vert_layout,
    }

}

fn create_interfaces(data: &[u32]) -> ShaderInterfaces {
    sr::ShaderModule::load_u32_data(data)
        .map(|m| {
            let inputs = m
                .enumerate_input_variables(None)
                .map(|inputs| {
                    inputs
                        .iter()
                        .filter(|i| {
                            !i.decoration_flags
                                .contains(sr::types::ReflectDecorationFlags::BUILT_IN)
                        })
                        .map(|i| ShaderInterfaceDefEntry {
                            location: i.location..(i.location + 1),
                            format: SpirvTy::from(i.format).inner(),
                            name: Some(Cow::from(i.name.clone())),
                        })
                        .collect::<Vec<ShaderInterfaceDefEntry>>()
                })
                .expect("Failed to pass inputs");
            let outputs = m
                .enumerate_output_variables(None)
                .map(|outputs| {
                    outputs
                        .iter()
                        .filter(|i| {
                            !i.decoration_flags
                                .contains(sr::types::ReflectDecorationFlags::BUILT_IN)
                        })
                        .map(|i| ShaderInterfaceDefEntry {
                            location: i.location..(i.location + 1),
                            format: SpirvTy::from(i.format).inner(),
                            name: Some(Cow::from(i.name.clone())),
                        })
                        .collect::<Vec<ShaderInterfaceDefEntry>>()
                })
                .expect("Failed to pass outputs");
            ShaderInterfaces { inputs, outputs }
        })
        .expect("failed to load module")
}

fn create_layouts(data: &[u32]) -> LayoutData {
    sr::ShaderModule::load_u32_data(data)
        .map(|m| {
            m.enumerate_descriptor_sets(None)
                .map(|sets| {
                    let num_sets = sets.len();
                    let num_bindings = sets
                        .iter()
                        .map(|i| {
                            dbg!(&i);
                            (i.set as usize, i.bindings.len())
                        })
                        .collect::<HashMap<usize, usize>>();
                    let descriptions = sets
                        .iter()
                        .map(|i| {
                            let desc = i.bindings.iter()
                                .map(|b| {
                                    let info = DescriptorDescInfo{
                                        descriptor_type: b.descriptor_type,
                                        image: b.image,
                                    };
                                    let ty = SpirvTy::<DescriptorDescTy>::from(info).inner();
                                    let stages = ShaderStages::none();
                                    let d = DescriptorDesc {
                                        ty,
                                        array_count: b.count,
                                        stages,
                                        // TODO this is what vulkan_shaders does but I don't think
                                        // it's correct
                                        readonly: true,
                                    };
                                    (b.binding as usize, d)
                                })
                            .collect::<HashMap<usize, DescriptorDesc>>();
                            (i.set as usize, desc)
                        })
                        .collect::<HashMap<usize, HashMap<usize, DescriptorDesc>>>();
                    LayoutData {
                        num_sets,
                        num_bindings,
                        descriptions,
                    }
                })
                .expect("Failed to pass outputs")
        })
        .expect("failed to load module")
}

