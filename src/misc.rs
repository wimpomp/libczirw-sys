use anyhow::{Error, Result, anyhow};
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

#[derive(Clone, Debug)]
pub enum Dimension {
    /// The Z-dimension.
    Z = 1,
    /// The C-dimension ("channel").
    C = 2,
    /// The T-dimension ("time").
    T = 3,
    /// The R-dimension ("rotation").
    R = 4,
    /// The S-dimension ("scene").
    S = 5,
    /// The I-dimension ("illumination").
    I = 6,
    /// The H-dimension ("phase").
    H = 7,
    /// The V-dimension ("view").
    V = 8,
    /// The B-dimension ("block") - its use is deprecated.
    B = 9,
}

impl Dimension {
    pub fn vec_from_bitflags(bit_flags: u32) -> Vec<Dimension> {
        let mut bit_flags = bit_flags;
        let mut dimensions = Vec::with_capacity(9);
        for i in 1..=9 {
            if (bit_flags & 1) > 0 {
                dimensions.push(Dimension::try_from(i).expect("i must be 0 <= i <= 9"));
            }
            bit_flags >>= 1;
        }
        dimensions
    }
}

impl TryFrom<i32> for Dimension {
    type Error = Error;

    fn try_from(dimension: i32) -> Result<Self> {
        match dimension {
            1 => Ok(Dimension::Z),
            2 => Ok(Dimension::C),
            3 => Ok(Dimension::T),
            4 => Ok(Dimension::R),
            5 => Ok(Dimension::S),
            6 => Ok(Dimension::I),
            7 => Ok(Dimension::H),
            8 => Ok(Dimension::V),
            9 => Ok(Dimension::B),
            _ => Err(anyhow!("Unknown dimension value {}", dimension)),
        }
    }
}

/// enum for SubBlock.get_raw_data
#[derive(Clone, Debug)]
pub enum RawDataType {
    Data = 0,
    Metadata = 1,
}

impl TryFrom<i32> for RawDataType {
    type Error = Error;

    fn try_from(raw_data_type: i32) -> Result<Self> {
        match raw_data_type {
            0 => Ok(RawDataType::Data),
            1 => Ok(RawDataType::Metadata),
            _ => Err(anyhow!("Unknown data type {}", raw_data_type)),
        }
    }
}

/// pixel type
#[derive(Clone, Debug)]
pub enum PixelType {
    Gray8 = 0,
    Gray16 = 1,
    Gray32Float = 2,
    Bgr24 = 3,
    Bgr48 = 4,
    Bgr96Float = 8,
    Bgra32 = 9,
    Gray64ComplexFloat = 10,
    Bgr192ComplexFloat = 11,
    Gray32 = 12,
    Gray64Float = 13,
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
