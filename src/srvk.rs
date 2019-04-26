use crate::sr; 
use crate::vk;
use vk::descriptor::descriptor::*;
use vk::pipeline::shader::ShaderInterfaceDefEntry;
use vk::format::Format;

pub struct SpirvTy<T> {
    inner: T,
}

pub struct DescriptorDescInfo {
    descriptor_type: sr::types::ReflectDescriptorType,
    image: sr::types::ReflectImageTraits,
}

impl<T> SpirvTy<T> {
    pub fn inner(self) -> T {
        self.inner
    }
}

impl From<DescriptorDescInfo> for SpirvTy<DescriptorDescTy> {
    fn from(d: DescriptorDescInfo) -> Self {
        use sr::types::ReflectDescriptorType as SR;
        use DescriptorDescTy as VK;
        let t = match d.descriptor_type {
            SR::Undefined => unreachable!(),
            SR::Sampler => VK::Sampler,
            SR::CombinedImageSampler => VK::CombinedImageSampler(SpirvTy::from(d.image).inner()),
            SR::SampledImage => unreachable!(),
            SR::StorageImage => unreachable!(),
            SR::UniformTexelBuffer => unreachable!(),
            SR::StorageTexelBuffer => unreachable!(),
            SR::UniformBuffer => unreachable!(),
            SR::StorageBuffer => unreachable!(),
            SR::UniformBufferDynamic => unreachable!(),
            SR::StorageBufferDynamic => unreachable!(),
            SR::InputAttachment => unreachable!(),
            SR::AccelerationStructureNV => unreachable!(),
        };
        SpirvTy {
            inner: t,
        }
    }
}

impl From<sr::types::ReflectImageTraits> for SpirvTy<DescriptorImageDesc> {
    fn from(d: sr::types::ReflectImageTraits) -> Self {
        let conv_array_layers = |a, d|{
            if a != 0 {
                DescriptorImageDescArray::Arrayed{max_layers: Some(d)}
            } else {
                DescriptorImageDescArray::NonArrayed
            }
        };
        let t = DescriptorImageDesc {
            sampled: d.sampled != 0,
            dimensions: SpirvTy::from(d.dim).inner(),
            format: Some(SpirvTy::from(d.image_format).inner()),
            multisampled: d.ms != 0,
            array_layers: conv_array_layers(d.arrayed, d.depth),
        };
        SpirvTy{inner: t}
    }
}

impl From<sr::types::variable::ReflectDimension> for SpirvTy<DescriptorImageDescDimensions> {
    fn from(d: sr::types::variable::ReflectDimension) -> Self {
        unimplemented!()
    }
}

impl From<sr::types::image::ReflectImageFormat> for SpirvTy<Format> {
    fn from(d: sr::types::image::ReflectImageFormat) -> Self {
        unimplemented!()
    }
}

impl From<sr::types::ReflectFormat> for SpirvTy<Format> {
    fn from(f: sr::types::ReflectFormat) -> Self {
        use sr::types::ReflectFormat::*;
        use Format::*;
        let t = match f {
            Undefined => unreachable!(),
            R32_UINT => R32Uint,
            R32_SINT => R32Sint,
            R32_SFLOAT => R32Sfloat,
            R32G32_UINT => R32G32Uint,
            R32G32_SINT => R32G32Sint,
            R32G32_SFLOAT => R32G32Sfloat,
            R32G32B32_UINT => R32G32B32Uint,
            R32G32B32_SINT => R32G32B32Sint,
            R32G32B32_SFLOAT => R32G32B32Sfloat,
            R32G32B32A32_UINT => R32G32B32A32Uint,
            R32G32B32A32_SINT => R32G32B32A32Sint,
            R32G32B32A32_SFLOAT => R32G32B32A32Sfloat,
        };
        SpirvTy { inner: t }
    }
}
