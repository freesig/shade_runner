use crate::sr; 
use crate::vk;
use crate::error::{ConvertError, Error};
use vk::descriptor::descriptor::*;
use vk::format::Format;
use std::convert::TryFrom;

pub struct SpirvTy<T> {
    inner: T,
}

pub struct DescriptorDescInfo {
    pub descriptor_type: sr::types::ReflectDescriptorType,
    pub image: sr::types::ReflectImageTraits,
}


impl<T> SpirvTy<T> {
    pub fn inner(self) -> T {
        self.inner
    }
}

impl TryFrom<DescriptorDescInfo> for SpirvTy<DescriptorDescTy> {
    type Error = Error;
    fn try_from(d: DescriptorDescInfo) -> Result<Self, Self::Error> {
        use sr::types::ReflectDescriptorType as SR;
        use DescriptorDescTy as VK;
        match d.descriptor_type {
            SR::Undefined => Err(ConvertError::Unimplemented),
            SR::Sampler => Ok(VK::Sampler),
            SR::CombinedImageSampler => Ok(VK::CombinedImageSampler(SpirvTy::try_from(d.image)?.inner())),
            SR::SampledImage => Err(ConvertError::Unimplemented),
            SR::StorageImage => Err(ConvertError::Unimplemented),
            SR::UniformTexelBuffer => Err(ConvertError::Unimplemented),
            SR::StorageTexelBuffer => Err(ConvertError::Unimplemented),
            SR::UniformBuffer => Err(ConvertError::Unimplemented),
            SR::StorageBuffer => Err(ConvertError::Unimplemented),
            SR::UniformBufferDynamic => Err(ConvertError::Unimplemented),
            SR::StorageBufferDynamic => Err(ConvertError::Unimplemented),
            SR::InputAttachment => Err(ConvertError::Unimplemented),
            SR::AccelerationStructureNV => Err(ConvertError::Unimplemented),
        }
        .map(|t| SpirvTy{ inner: t })
        .map_err(Error::Layout)
    }
}

impl TryFrom<sr::types::ReflectImageTraits> for SpirvTy<DescriptorImageDesc> {
    type Error = Error;
    fn try_from(d: sr::types::ReflectImageTraits) -> Result<Self, Self::Error> {
        let conv_array_layers = |a, d|{
            if a != 0 {
                DescriptorImageDescArray::Arrayed{max_layers: Some(d)}
            } else {
                DescriptorImageDescArray::NonArrayed
            }
        };
        let t = DescriptorImageDesc {
            sampled: d.sampled != 0,
            dimensions: SpirvTy::try_from(d.dim)?.inner(),
            // TODO figure out how to do format correctly
            //format: Some(SpirvTy::from(d.image_format).inner()),
            format: None,
            multisampled: d.ms != 0,
            array_layers: conv_array_layers(d.arrayed, d.depth),
        };
        Ok(SpirvTy{inner: t})
    }
}

impl TryFrom<sr::types::variable::ReflectDimension> for SpirvTy<DescriptorImageDescDimensions> {
    type Error = Error;
    fn try_from(d: sr::types::variable::ReflectDimension) -> Result<Self, Self::Error> {
        use sr::types::variable::ReflectDimension::*;
        use DescriptorImageDescDimensions::*;
        match d {
            Type1d => Ok(OneDimensional),
            Type2d => Ok(TwoDimensional),
            Type3d => Ok(ThreeDimensional),
            sr::types::variable::ReflectDimension::Cube => Ok(DescriptorImageDescDimensions::Cube),
            _ => Err(ConvertError::Unimplemented),
        }
        .map(|t| SpirvTy{ inner: t })
        .map_err(Error::Layout)
    }
}

// I think this is wrong and currently is unused
impl From<sr::types::image::ReflectImageFormat> for SpirvTy<Format> {
    fn from(d: sr::types::image::ReflectImageFormat) -> Self {
        use sr::types::image::ReflectImageFormat::*;
        use Format::*;
        let inner = match d {
            Undefined => unimplemented!(),
            RGBA32_FLOAT => R32G32B32A32Sfloat,
            RGBA16_FLOAT => R16G16B16A16Sfloat,
            R32_FLOAT => R32Sfloat,
            RGBA8 => unimplemented!(),
            RGBA8_SNORM => R8G8B8A8Snorm,
            RG32_FLOAT => R32G32Sfloat,
            RG16_FLOAT => R16G16Sfloat,
            R11G11B10_FLOAT => unimplemented!(),
            R16_FLOAT => R16Sfloat,
            RGBA16 => unimplemented!(),
            RGB10A2 => unimplemented!(), 
            RG16 => unimplemented!(),
            RG8 => unimplemented!(),
            R16 => unimplemented!(),
            R8 => unimplemented!(),
            RGBA16_SNORM => R16G16B16A16Snorm,
            RG16_SNORM => R16G16Snorm,
            RG8_SNORM => R8G8Snorm,
            R16_SNORM => R16Snorm,
            R8_SNORM => R8Snorm,
            RGBA32_INT => R32G32B32A32Sint,
            RGBA16_INT => R16G16B16A16Sint,
            RGBA8_INT => R8G8B8A8Sint,
            R32_INT => R32Sint,
            RG32_INT => R32G32Sint,
            RG16_INT => R16G16Sint,
            RG8_INT => R8G8Sint,
            R16_INT => R16Sint,
            R8_INT => R8Sint,
            RGBA32_UINT => R32G32B32A32Uint,
            RGBA16_UINT => R16G16B16A16Uint,
            RGBA8_UINT => R8G8B8A8Uint,
            R32_UINT => R32Uint,
            RGB10A2_UINT => A2R10G10B10UintPack32,
            RG32_UINT => R32G32Uint,
            RG16_UINT => R16G16Uint,
            RG8_UINT => R8G8Uint,
            R16_UINT =>R16Uint,
            R8_UINT =>R8Uint,
        };
        SpirvTy{ inner };
        // This function shouldn't be called yet because 
        // it is not implemented correctly
        unreachable!()
    }
}

impl TryFrom<sr::types::ReflectFormat> for SpirvTy<Format> {
    type Error = Error;
    fn try_from(f: sr::types::ReflectFormat) -> Result<Self, Self::Error> {
        use sr::types::ReflectFormat::*;
        use Format::*;
        let t = match f {
            Undefined => Err(Error::Layout(ConvertError::Unimplemented))?,
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
        Ok(SpirvTy { inner: t })
    }
}
