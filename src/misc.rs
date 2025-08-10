use anyhow::{anyhow, Error, Result};
use std::fmt;
use std::mem::MaybeUninit;
use std::os::raw::c_int;

/// the error type for libCZIAPI
#[derive(Clone, Debug)]
pub enum LibCZIApiError {
    OK,
    InvalidArgument,
    InvalidHandle,
    OutOfMemory,
    IndexOutOfRange,
    LockUnlockSemanticViolated,
    UnspecifiedError,
}

impl std::error::Error for LibCZIApiError {}

impl TryFrom<c_int> for LibCZIApiError {
    type Error = Error;

    fn try_from(code: c_int) -> Result<Self> {
        match code {
            0 => Ok(LibCZIApiError::OK),
            1 => Err(Error::from(LibCZIApiError::InvalidArgument)),
            2 => Err(Error::from(LibCZIApiError::InvalidHandle)),
            3 => Err(Error::from(LibCZIApiError::OutOfMemory)),
            4 => Err(Error::from(LibCZIApiError::IndexOutOfRange)),
            20 => Err(Error::from(LibCZIApiError::LockUnlockSemanticViolated)),
            50 => Err(Error::from(LibCZIApiError::UnspecifiedError)),
            _ => Err(anyhow!("Unknown error code {}", code)),
        }
    }
}

impl fmt::Display for LibCZIApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LibCZIApi {self:?}")
    }
}

/// enum for SubBlock.get_raw_data
pub enum RawDataType {
    Data,
    Metadata,
}

/// pixel type
pub enum PixelType {
    Gray8,
    Gray16,
    Gray32Float,
    Bgr24,
    Bgr48,
    Bgr96Float,
    Bgra32,
    Gray64ComplexFloat,
    Bgr192ComplexFloat,
    Gray32,
    Gray64Float,
}

impl TryFrom<i32> for PixelType {
    type Error = Error;

    fn try_from(pixel_type: i32) -> Result<Self> {
        match pixel_type {
            0 => Ok(PixelType::Gray8),
            1 => Ok(PixelType::Gray16),
            2 => Ok(PixelType::Gray32Float),
            3 => Ok(PixelType::Bgr24),
            4 => Ok(PixelType::Bgr48),
            8 => Ok(PixelType::Bgr96Float),
            9 => Ok(PixelType::Bgra32),
            10 => Ok(PixelType::Gray64ComplexFloat),
            11 => Ok(PixelType::Bgr192ComplexFloat),
            12 => Ok(PixelType::Gray32),
            13 => Ok(PixelType::Gray64Float),
            _ => Err(anyhow!("Unknown pixel type {}", pixel_type)),
        }
    }
}

impl From<PixelType> for i32 {
    fn from(pixel_type: PixelType) -> Self {
        match pixel_type {
            PixelType::Gray8 => 0,
            PixelType::Gray16 => 1,
            PixelType::Gray32Float => 2,
            PixelType::Bgr24 => 3,
            PixelType::Bgr48 => 4,
            PixelType::Bgr96Float => 8,
            PixelType::Bgra32 => 9,
            PixelType::Gray64ComplexFloat => 10,
            PixelType::Bgr192ComplexFloat => 11,
            PixelType::Gray32 => 12,
            PixelType::Gray64Float => 13,
        }
    }
}

pub trait Ptr {
    type Pointer;

    unsafe fn assume_init(ptr: MaybeUninit<Self::Pointer>) -> Self;

    fn as_mut_ptr(&self) -> *mut Self::Pointer
    where
        Self: Sized;

    fn as_ptr(&self) -> *const Self::Pointer
    where
        Self: Sized;
}
