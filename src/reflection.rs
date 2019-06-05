use crate::error::Error;
use crate::layouts::*;
use crate::sr;
use crate::srvk::{DescriptorDescInfo, SpirvTy};
use crate::vk::descriptor::descriptor::*;
use crate::vk::descriptor::pipeline_layout::PipelineLayoutDescPcRange;
use crate::vk::pipeline::shader::ShaderInterfaceDefEntry;
use crate::CompiledShaders;
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryFrom;

pub struct ShaderInterfaces {
    pub inputs: Vec<ShaderInterfaceDefEntry>,
    pub outputs: Vec<ShaderInterfaceDefEntry>,
}

#[derive(Debug, Clone, Default)]
pub struct LayoutData {
    pub num_sets: usize,
    pub num_bindings: HashMap<usize, usize>,
    pub descriptions: HashMap<usize, HashMap<usize, DescriptorDesc>>,
    pub num_constants: usize,
    pub pc_ranges: Vec<PipelineLayoutDescPcRange>,
}

pub fn create_entry(shaders: &CompiledShaders) -> Result<Entry, Error> {
    let vertex_interfaces = create_interfaces(&shaders.vertex)?;
    let vertex_layout = create_layouts(&shaders.vertex)?;
    let fragment_interfaces = create_interfaces(&shaders.fragment)?;
    let fragment_layout = create_layouts(&shaders.fragment)?;
    let frag_input = FragInput {
        inputs: fragment_interfaces.inputs,
    };
    let frag_output = FragOutput {
        outputs: fragment_interfaces.outputs,
    };
    let frag_layout = FragLayout {
        layout_data: fragment_layout,
    };
    let vert_input = VertInput {
        inputs: vertex_interfaces.inputs,
    };
    let vert_output = VertOutput {
        outputs: vertex_interfaces.outputs,
    };
    let vert_layout = VertLayout {
        layout_data: vertex_layout,
    };
    Ok(Entry {
        frag_input,
        frag_output,
        vert_input,
        vert_output,
        frag_layout,
        vert_layout,
        compute_layout: Default::default(),
    })
}

pub fn create_compute_entry(shaders: &CompiledShaders) -> Result<Entry, Error> {
    create_layouts(&shaders.compute).map(|layout_data| {
        let mut entry = Entry::default();
        entry.compute_layout = ComputeLayout{ layout_data };
        entry
    })
}

fn create_interfaces(data: &[u32]) -> Result<ShaderInterfaces, Error> {
    sr::ShaderModule::load_u32_data(data)
        .map_err(|e| Error::LoadingData(e.to_string()))
        .map(|m| {
            let inputs = m
                .enumerate_input_variables(None)
                .map_err(|e| Error::LoadingData(e.to_string()))
                .and_then(|inputs| {
                    inputs
                        .iter()
                        .filter(|i| {
                            !i.decoration_flags
                                .contains(sr::types::ReflectDecorationFlags::BUILT_IN)
                        })
                        .map(|i| Ok(ShaderInterfaceDefEntry {
                            location: i.location..(i.location + 1),
                            format: SpirvTy::try_from(i.format)?.inner(),
                            name: Some(Cow::from(i.name.clone())),
                        }))
                        .collect::<Result<Vec<ShaderInterfaceDefEntry>, _>>()
                });
            let outputs = m
                .enumerate_output_variables(None)
                .map_err(|e| Error::LoadingData(e.to_string()))
                .and_then(|outputs| {
                    outputs
                        .iter()
                        .filter(|i| {
                            !i.decoration_flags
                                .contains(sr::types::ReflectDecorationFlags::BUILT_IN)
                        })
                        .map(|i| Ok(ShaderInterfaceDefEntry {
                            location: i.location..(i.location + 1),
                            format: SpirvTy::try_from(i.format)?.inner(),
                            name: Some(Cow::from(i.name.clone())),
                        }))
                        .collect::<Result<Vec<ShaderInterfaceDefEntry>, _>>()
                });
            inputs.and_then(|inputs| outputs.map(|outputs| ShaderInterfaces { inputs, outputs } ))
        })
    .and_then(|t| t)
}

fn create_layouts(data: &[u32]) -> Result<LayoutData, Error> {
    sr::ShaderModule::load_u32_data(data)
        .map(|m| {
            let descs: Result<_, Error> = m
                .enumerate_descriptor_sets(None)
                .map_err(|e| Error::LoadingData(e.to_string()))
                .and_then(|sets| {
                    let num_sets = sets.len();
                    let num_bindings = sets
                        .iter()
                        .map(|i| (i.set as usize, i.bindings.len()))
                        .collect::<HashMap<usize, usize>>();
                    let descriptions = sets
                        .iter()
                        .map(|i| {
                            let desc = i
                                .bindings
                                .iter()
                                .map(|b| {
                                    let info = DescriptorDescInfo {
                                        descriptor_type: b.descriptor_type,
                                        image: b.image,
                                    };
                                    let ty = SpirvTy::<DescriptorDescTy>::try_from(info)?.inner();
                                    let stages = ShaderStages::none();
                                    let d = DescriptorDesc {
                                        ty,
                                        array_count: b.count,
                                        stages,
                                        // TODO this is what vulkan_shaders does but I don't think
                                        // it's correct
                                        readonly: true,
                                    };
                                    Ok((b.binding as usize, d))
                                })
                                .collect::<Result<HashMap<usize, DescriptorDesc>, Error>>();
                            desc.and_then(|d| Ok((i.set as usize, d)))
                        })
                        .collect::<Result<HashMap<usize, _>, Error>>();
                    descriptions.map(|d| (num_sets, num_bindings, d))
                });
            let pcs = m
                .enumerate_push_constant_blocks(None)
                .map_err(|e| Error::LoadingData(e.to_string()))
                .map(|constants| {
                    let num_constants = constants.len();
                    let pc_ranges = constants
                        .iter()
                        .map(|pc| PipelineLayoutDescPcRange {
                            offset: pc.offset as usize,
                            size: pc.size as usize,
                            stages: ShaderStages::all(),
                        })
                        .collect::<Vec<PipelineLayoutDescPcRange>>();
                    (num_constants, pc_ranges)
                });
            descs.and_then(|(num_sets, num_bindings, descriptions)| {
                pcs.map(|(num_constants, pc_ranges)| LayoutData {
                    num_sets,
                    num_bindings,
                    descriptions,
                    num_constants,
                    pc_ranges,
                })
            })
        })
        .map_err(|e| Error::LoadingData(e.to_string()))
        .and_then(|t| t)
}
