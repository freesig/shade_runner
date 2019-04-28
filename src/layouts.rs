use crate::vk;
use vk::pipeline::shader::*;
pub use vk::pipeline::shader::ShaderInterfaceDef;
use vk::descriptor::descriptor::*;
use vk::descriptor::pipeline_layout::*;
use crate::reflection::LayoutData;

#[derive(Debug, Clone)]
pub struct Entry {
    pub frag_input: FragInput,
    pub frag_output: FragOutput,
    pub frag_layout: FragLayout,
    pub vert_input: VertInput,
    pub vert_output: VertOutput,
    pub vert_layout: VertLayout,
}

#[derive(Debug, Clone)]
pub struct FragInput {
    pub inputs: Vec<ShaderInterfaceDefEntry>,
}

unsafe impl ShaderInterfaceDef for FragInput {
    type Iter = FragInputIter;

    fn elements(&self) -> FragInputIter {
        self.inputs.clone().into_iter()
    }
}

pub type FragInputIter = std::vec::IntoIter<ShaderInterfaceDefEntry>;

#[derive(Debug, Clone)]
pub struct FragOutput {
    pub outputs: Vec<ShaderInterfaceDefEntry>,
}

unsafe impl ShaderInterfaceDef for FragOutput {
    type Iter = FragOutputIter;

    fn elements(&self) -> FragOutputIter {
        self.outputs.clone().into_iter()
    }
}

pub type FragOutputIter = std::vec::IntoIter<ShaderInterfaceDefEntry>;

// Layout same as with vertex shader.
#[derive(Debug, Clone)]
pub struct FragLayout {
    pub stages: ShaderStages,
    pub layout_data: LayoutData,
}
unsafe impl PipelineLayoutDesc for FragLayout {
    fn num_sets(&self) -> usize {
        self.layout_data.num_sets
    }
    fn num_bindings_in_set(&self, set: usize) -> Option<usize> {
        self.layout_data.num_bindings.get(&set).map(|&i| i)
    }
    fn descriptor(&self, set: usize, binding: usize) -> Option<DescriptorDesc> {
        self.layout_data.descriptions.get(&set)
            .and_then(|s|s.get(&binding))
            .map(|desc| {
                let mut desc = desc.clone();
                desc.stages = self.stages.clone();
                desc
            })
        
    }
    fn num_push_constants_ranges(&self) -> usize {
        self.layout_data.num_constants
    }
    fn push_constants_range(&self, num: usize) -> Option<PipelineLayoutDescPcRange> {
        self.layout_data.pc_ranges.get(num)
            .map(|desc| {
                let mut desc = desc.clone();
                desc.stages = self.stages.clone();
                desc
            })

    }
}

#[derive(Debug, Clone)]
pub struct VertInput {
    pub inputs: Vec<ShaderInterfaceDefEntry>,
}

unsafe impl ShaderInterfaceDef for VertInput {
    type Iter = VertInputIter;

    fn elements(&self) -> VertInputIter {
        self.inputs.clone().into_iter()
    }
}

pub type VertInputIter = std::vec::IntoIter<ShaderInterfaceDefEntry>;

#[derive(Debug, Clone)]
pub struct VertOutput {
    pub outputs: Vec<ShaderInterfaceDefEntry>,
}

unsafe impl ShaderInterfaceDef for VertOutput {
    type Iter = VertOutputIter;

    fn elements(&self) -> VertOutputIter {
        self.outputs.clone().into_iter()
    }
}

pub type VertOutputIter = std::vec::IntoIter<ShaderInterfaceDefEntry>;

// This structure describes layout of this stage.
#[derive(Debug, Clone)]
pub struct VertLayout {
    pub stages: ShaderStages,
    pub layout_data: LayoutData,
}
unsafe impl PipelineLayoutDesc for VertLayout {
    fn num_sets(&self) -> usize {
        self.layout_data.num_sets
    }
    fn num_bindings_in_set(&self, set: usize) -> Option<usize> {
        self.layout_data.num_bindings.get(&set).map(|&i| i)
    }
    fn descriptor(&self, set: usize, binding: usize) -> Option<DescriptorDesc> {
        self.layout_data.descriptions.get(&set)
            .and_then(|s|s.get(&binding))
            .map(|desc| {
                let mut desc = desc.clone();
                desc.stages = self.stages.clone();
                desc
            })
        
    }
    fn num_push_constants_ranges(&self) -> usize {
        self.layout_data.num_constants
    }
    fn push_constants_range(&self, num: usize) -> Option<PipelineLayoutDescPcRange> {
        self.layout_data.pc_ranges.get(num)
            .map(|desc| {
                let mut desc = desc.clone();
                desc.stages = self.stages.clone();
                desc
            })

    }
}
