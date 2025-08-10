use crate::handle::{InputStream, MemoryAllocation};
use crate::misc::{PixelType, Ptr};
use crate::sys::*;
use anyhow::{Error, Result};
use std::ffi::{CStr, CString, c_char, c_void};
use std::mem::{ManuallyDrop, MaybeUninit};

/// This struct contains the version information of the libCZIApi-library. For versioning libCZI, SemVer2 (<https://semver.org/>) is used.
/// Note that the value of the tweak version number does not have a meaning (as far as SemVer2 is concerned).
#[derive(Clone, Debug)]
pub struct LibCZIVersionInfo(pub (crate) LibCZIVersionInfoInterop);

/// This struct gives information about the build of the libCZIApi-library.
/// Note that all strings must be freed by the caller (using libCZI_Free).
#[derive(Clone, Debug)]
pub struct LibCZIBuildInformation(pub (crate) LibCZIBuildInformationInterop);

#[derive(Clone, Debug)]
pub struct InputStreamClassInfo(pub (crate) InputStreamClassInfoInterop);

/// This structure gives additional information about an error that occurred in the external stream.
#[derive(Clone, Debug)]
pub struct ExternalStreamErrorInfo(pub (crate) ExternalStreamErrorInfoInterop);

/// This structure contains information about externally provided functions for reading data from an input stream,
/// and it is used to construct a stream-object to be used with libCZI.
/// Note on lifetime: The function pointers must remain valid until the function 'close_function' is called. The lifetime
/// may extend beyond calling the 'libCZI_ReleaseInputStream' function for the corresponding stream-object.
#[derive(Clone, Debug)]
pub struct ExternalInputStreamStruct(pub (crate) ExternalInputStreamStructInterop);

/// This structure contains information about externally provided functions for writing data to an output stream,
/// and it is used to construct a stream-object to be used with libCZI.
/// Note on lifetime: The function pointers must remain valid until the function 'close_function' is called. The lifetime
/// may extend beyond calling the 'libCZI_ReleaseOutputStream' function for the corresponding stream-object.
#[derive(Clone, Debug)]
pub struct ExternalOutputStreamStruct(pub (crate) ExternalOutputStreamStructInterop);

/// This structure gather the information needed to create a reader object.
#[derive(Clone, Debug)]
pub struct ReaderOpenInfo(pub (crate) ReaderOpenInfoInterop);

/// This structure describes a rectangle, given by its top-left corner and its width and height.
#[derive(Clone, Debug)]
pub struct IntRect(pub (crate) IntRectInterop);

/// This structure describes a size, given by its width and height.
#[derive(Clone, Debug)]
pub struct IntSize(pub (crate) IntSizeInterop);

/// This structure gives the bounds for a set of dimensions.
/// The bit at position `i` in `dimensions_valid` indicates whether the interval for dimension `i+1` is valid. So, bit 0
/// is corresponding to dimension 1 (=Z), bit 1 to dimension 2 (=C), and so on.
/// In the fixed-sized arrays `start` and `size`, the start and size values for the dimensions are stored. The elements at
/// position 0 corresponds to the first valid dimension, the element at position 1 to the second valid dimension, and so on.
/// An example would be: `dimensions_valid` = 0b00000011, `start` = { 0, 2 }, `size` = { 5, 6 }. This would mean that the
/// dimension 'Z' is valid, and the interval is [0, 5], and the dimension 'C' is valid, and the interval is [2, 8].
#[derive(Clone, Debug)]
pub struct DimBounds(pub (crate) DimBoundsInterop);

/// This structure gives the coordinates (of a sub-block) for a set of dimension.
/// The bit at position `i` in `dimensions_valid` indicates whether the coordinate for dimension `i+1` is valid. So, bit 0
/// is corresponding to dimension 1 (=Z), bit 1 to dimension 2 (=C), and so on.
/// In the fixed-sized array `value`, the coordinate for the dimensions is stored. The element at
/// position 0 corresponds to the first valid dimension, the element at position 1 to the second valid dimension, and so on.
/// An example would be: `dimensions_valid` = 0b00000011, `value` = { 0, 2 }. This would mean that the
/// dimension 'Z' is valid, and the coordinate for 'Z' is 0, and the dimension 'C' is valid, and the coordinate for 'C' is 2.
#[derive(Clone, Debug)]
pub struct Coordinate(pub (crate) CoordinateInterop);

/// This structure contains the bounding boxes for a scene.
#[derive(Clone, Debug)]
pub struct BoundingBoxes(pub (crate) BoundingBoxesInterop);

/// This structure contains basic statistics about an CZI-document.
#[derive(Clone, Debug)]
pub struct SubBlockStatistics(pub (crate) SubBlockStatisticsInterop);

/// This structure extends on the basic statistics about an CZI-document, and includes per-scene statistics.
#[derive(Debug)]
pub struct SubBlockStatisticsEx(pub (crate) SubBlockStatisticsInteropEx);

#[derive(Clone, Debug)]
pub struct MetadataAsXml(pub (crate) MetadataAsXmlInterop);

/// Information about the bitmap represented by a bitmap-object.
#[derive(Clone, Debug)]
pub struct BitmapInfo(pub (crate) BitmapInfoInterop);

/// This structure contains information about a locked bitmap-object, allowing direct
/// access to the pixel data.
#[derive(Clone, Debug)]
pub struct BitmapLockInfo(pub (crate) BitmapLockInfoInterop);

/// This structure contains the information about a sub-block.
#[derive(Clone, Debug)]
pub struct SubBlockInfo(pub (crate) SubBlockInfoInterop);

/// This structure contains the information about an attachment.
/// Note that performance reasons we use a fixed-size array for the name. In the rare case that the name is too long to fit into the
/// fixed-size array, the 'overflow' field is set to true. In this case, the name is truncated and the 'overflow' field is set to true.
/// In addition, the field 'name_in_case_of_overflow' then contains the full text, allocated with 'libCZI_AllocateString' (and responsibility
/// for releasing the memory is with the caller).
#[derive(Clone, Debug)]
pub struct AttachmentInfo(pub (crate) AttachmentInfoInterop);

/// This structure contains the information about file-header.
#[derive(Clone, Debug)]
pub struct FileHeaderInfo(pub (crate) FileHeaderInfoInterop);

/// This structure is used to pass the subblock information to libCZIAPI, describing a subblock to be added to a CZI-file.
#[derive(Clone, Debug)]
pub struct AddSubBlockInfo(pub (crate) AddSubBlockInfoInterop);

/// This structure is used to pass the attachment information to libCZIAPI, describing an attachment to be added to a CZI-file.
#[derive(Clone, Debug)]
pub struct AddAttachmentInfo(pub (crate) AddAttachmentInfoInterop);

/// This structure is used to pass the metadata information to libCZIAPI.
#[derive(Clone, Debug)]
pub struct WriteMetadataInfo(pub (crate) WriteMetadataInfoInterop);

/// This structure is used to pass the accessor options to libCZIAPI.
#[derive(Clone, Debug)]
pub struct AccessorOptions(pub (crate) AccessorOptionsInterop);

/// This structure gathers all information about a channel for the purpose of multi-channel-composition.
#[derive(Clone, Debug)]
pub struct CompositionChannelInfo(pub (crate) CompositionChannelInfoInterop);

/// This structure gathers the information about the scaling.
#[derive(Clone, Debug)]
pub struct ScalingInfo(pub (crate) ScalingInfoInterop);

macro_rules! impl_ptr {
    ($($n:ident: $t:ty: $s:ty $(,)?)*) => {
        $(
            impl Ptr for $t {
                type Pointer = $s;

                unsafe fn assume_init(ptr: MaybeUninit<Self::Pointer>) -> Self {
                    Self(unsafe { ptr.assume_init() })
                }

                fn as_mut_ptr(&self) -> *mut Self::Pointer {
                    // Box::into_raw(Box::new(self.0))
                    &self.0 as *const _ as *mut _
                }

                fn as_ptr(&self) -> *const Self::Pointer {
                    &self.0 as *const _ as *const _
                    // Box::into_raw(Box::new(self.0)) as *const Self::Pointer
                }
            }
        )*
    };
}

impl_ptr! {
    LibCZIVersionInfo: LibCZIVersionInfo: LibCZIVersionInfoInterop,
    LibCZIBuildInformation: LibCZIBuildInformation: LibCZIBuildInformationInterop,
    InputStreamClassInfo: InputStreamClassInfo: InputStreamClassInfoInterop,
    ExternalStreamErrorInfo: ExternalStreamErrorInfo: ExternalStreamErrorInfoInterop,
    ExternalInputStreamStruct: ExternalInputStreamStruct: ExternalInputStreamStructInterop,
    ExternalOutputStreamStruct: ExternalOutputStreamStruct: ExternalOutputStreamStructInterop,
    ReaderOpenInfo: ReaderOpenInfo: ReaderOpenInfoInterop,
    IntRect: IntRect: IntRectInterop,
    IntSize: IntSize: IntSizeInterop,
    DimBounds: DimBounds: DimBoundsInterop,
    Coordinate: Coordinate: CoordinateInterop,
    BoundingBoxes: BoundingBoxes: BoundingBoxesInterop,
    SubBlockStatistics: SubBlockStatistics: SubBlockStatisticsInterop,
    SubBlockStatisticsEx: SubBlockStatisticsEx: SubBlockStatisticsInteropEx,
    MetadataAsXml: MetadataAsXml: MetadataAsXmlInterop,
    BitmapInfo: BitmapInfo: BitmapInfoInterop,
    BitmapLockInfo: BitmapLockInfo: BitmapLockInfoInterop,
    SubBlockInfo: SubBlockInfo: SubBlockInfoInterop,
    AttachmentInfo: AttachmentInfo: AttachmentInfoInterop,
    FileHeaderInfo: FileHeaderInfo: FileHeaderInfoInterop,
    AddSubBlockInfo: AddSubBlockInfo: AddSubBlockInfoInterop,
    AddAttachmentInfo: AddAttachmentInfo: AddAttachmentInfoInterop,
    WriteMetadataInfo: WriteMetadataInfo: WriteMetadataInfoInterop,
    AccessorOptions: AccessorOptions: AccessorOptionsInterop,
    CompositionChannelInfo: CompositionChannelInfo: CompositionChannelInfoInterop,
    ScalingInfo: ScalingInfo: ScalingInfoInterop,
}

impl LibCZIVersionInfo {
    pub fn get_major(&self) -> i32 {
        self.0.major
    }
    pub fn get_minor(&self) -> i32 {
        self.0.minor
    }
    pub fn get_patch(&self) -> i32 {
        self.0.patch
    }
    pub fn get_tweak(&self) -> i32 {
        self.0.tweak
    }
}

impl LibCZIBuildInformation {
    pub fn get_compiler_information(&self) -> Result<&str> {
        Ok(unsafe { CStr::from_ptr(self.0.compilerIdentification) }.to_str()?)
    }
    pub fn get_repository_url(&self) -> Result<&str> {
        Ok(unsafe { CStr::from_ptr(self.0.repositoryUrl) }.to_str()?)
    }
    pub fn get_repository_branch(&self) -> Result<&str> {
        Ok(unsafe { CStr::from_ptr(self.0.repositoryBranch) }.to_str()?)
    }
    pub fn get_repository_tag(&self) -> Result<&str> {
        Ok(unsafe { CStr::from_ptr(self.0.repositoryTag) }.to_str()?)
    }
}

impl Drop for LibCZIBuildInformation {
    fn drop(&mut self) {
        unsafe {
            libCZI_Free(self.0.compilerIdentification as *mut c_void);
            libCZI_Free(self.0.repositoryUrl as *mut c_void);
            libCZI_Free(self.0.repositoryBranch as *mut c_void);
            libCZI_Free(self.0.repositoryTag as *mut c_void);
        }
    }
}

impl InputStreamClassInfo {
    pub fn get_name(&self) -> Result<&str> {
        Ok(unsafe { CStr::from_ptr(self.0.name) }.to_str()?)
    }
    pub fn get_description(&self) -> Result<&str> {
        Ok(unsafe { CStr::from_ptr(self.0.description) }.to_str()?)
    }
}

impl Drop for InputStreamClassInfo {
    fn drop(&mut self) {
        unsafe {
            libCZI_Free(self.0.name as *mut c_void);
            libCZI_Free(self.0.description as *mut c_void);
        }
    }
}

impl ExternalStreamErrorInfo {
    pub fn get_error_code(&self) -> i32 {
        self.0.error_code
    }

    pub fn get_error_message(&self) -> MemoryAllocation {
        MemoryAllocation(self.0.error_message)
    }
}

// TODO
impl ExternalInputStreamStruct {
    /// A user parameter which is passed to the callback function.
    pub fn get_opaque_handle1(&self) -> u64 {
        self.0.opaque_handle1
    }
    /// A user parameter which is passed to the callback function.
    pub fn get_opaque_handle2(&self) -> u64 {
        self.0.opaque_handle2
    }
    pub fn set_opaque_handle1(&mut self, handle: u64) {
        self.0.opaque_handle1 = handle;
    }
    pub fn set_opaque_handle2(&mut self, handle: u64) {
        self.0.opaque_handle2 = handle;
    }
}

// TODO
impl ExternalOutputStreamStruct {
    /// A user parameter which is passed to the callback function.
    pub fn get_opaque_handle1(&self) -> u64 {
        self.0.opaque_handle1
    }
    /// A user parameter which is passed to the callback function.
    pub fn get_opaque_handle2(&self) -> u64 {
        self.0.opaque_handle2
    }
    pub fn set_opaque_handle1(&mut self, handle: u64) {
        self.0.opaque_handle1 = handle;
    }
    pub fn set_opaque_handle2(&mut self, handle: u64) {
        self.0.opaque_handle2 = handle;
    }
}

/// This structure gather the information needed to create a reader object.
impl ReaderOpenInfo {
    pub fn new(stream: &InputStream) -> Self {
        Self(ReaderOpenInfoInterop {
            streamObject: stream.handle(),
        })
    }
    pub fn get_stream(&self) -> InputStream {
        InputStream(self.0.streamObject)
    }
}

/// This structure describes a rectangle, given by its top-left corner and its width and height.
impl IntRect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self(IntRectInterop { x, y, w, h })
    }
    pub fn get_x(&self) -> i32 {
        self.0.x
    }
    pub fn get_y(&self) -> i32 {
        self.0.y
    }
    pub fn get_w(&self) -> i32 {
        self.0.w
    }
    pub fn get_h(&self) -> i32 {
        self.0.h
    }
    pub fn set_x(&mut self, x: i32) {
        self.0.x = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.0.y = y;
    }
    pub fn set_w(&mut self, w: i32) {
        self.0.w = w;
    }
    pub fn set_h(&mut self, h: i32) {
        self.0.h = h;
    }
}

impl IntSize {
    pub fn new(w: i32, h: i32) -> Self {
        Self(IntSizeInterop { w, h })
    }
    pub fn get_w(&self) -> i32 {
        self.0.w
    }
    pub fn get_h(&self) -> i32 {
        self.0.h
    }
    pub fn set_w(&mut self, w: i32) {
        self.0.w = w;
    }
    pub fn set_h(&mut self, h: i32) {
        self.0.h = h;
    }
}

impl DimBounds {
    pub fn new(dimensions_valid: u32, start: [i32; 9], size: [i32; 9]) -> Self {
        Self(DimBoundsInterop {
            dimensions_valid,
            start,
            size,
        })
    }
    pub fn get_dimensions_valid(&self) -> u32 {
        self.0.dimensions_valid
    }
    pub fn get_start(&self) -> [i32; 9] {
        self.0.start
    }
    pub fn get_size(&self) -> [i32; 9] {
        self.0.size
    }
    pub fn set_dimensions_valid(&mut self, dimensions_valid: u32) {
        self.0.dimensions_valid = dimensions_valid;
    }
    pub fn set_start(&mut self, start: [i32; 9]) {
        self.0.start = start;
    }
    pub fn set_size(&mut self, size: [i32; 9]) {
        self.0.size = size;
    }
}

impl Coordinate {
    pub fn new(dimensions_valid: u32, value: [i32; 9]) -> Self {
        Self(CoordinateInterop {
            dimensions_valid,
            value,
        })
    }
    pub fn get_dimensions_valid(&self) -> u32 {
        self.0.dimensions_valid
    }
    pub fn get_value(&self) -> [i32; 9] {
        self.0.value
    }
    pub fn set_dimensions_valid(&mut self, dimensions_valid: u32) {
        self.0.dimensions_valid = dimensions_valid;
    }
    pub fn set_value(&mut self, value: [i32; 9]) {
        self.0.value = value;
    }
}

impl BoundingBoxes {
    pub fn new(
        scene_index: i32,
        bounding_box: IntRectInterop,
        bounding_box_layer0_only: IntRectInterop,
    ) -> Self {
        Self(BoundingBoxesInterop {
            sceneIndex: scene_index,
            bounding_box,
            bounding_box_layer0_only,
        })
    }
    pub fn get_scene_index(&self) -> i32 {
        self.0.sceneIndex
    }
    pub fn get_bounding_box(&self) -> IntRectInterop {
        self.0.bounding_box
    }
    pub fn get_bounding_box_layer0_only(&self) -> IntRectInterop {
        self.0.bounding_box_layer0_only
    }
    pub fn set_scene_index(&mut self, scene_index: i32) {
        self.0.sceneIndex = scene_index;
    }
    pub fn set_bounding_box(&mut self, bounding_box: IntRectInterop) {
        self.0.bounding_box = bounding_box;
    }
    pub fn set_bounding_box_layer0_only(&mut self, bounding_box_layer0_only: IntRectInterop) {
        self.0.bounding_box_layer0_only = bounding_box_layer0_only
    }
}

impl SubBlockStatistics {
    pub fn new(
        sub_block_count: i32,
        min_m_index: i32,
        max_m_index: i32,
        bounding_box: IntRect,
        bounding_box_layer0: IntRect,
        dim_bounds: DimBounds,
    ) -> Self {
        Self(SubBlockStatisticsInterop {
            sub_block_count,
            min_m_index,
            max_m_index,
            bounding_box: bounding_box.0,
            bounding_box_layer0: bounding_box_layer0.0,
            dim_bounds: dim_bounds.0,
        })
    }
    pub fn get_sub_block_count(&self) -> i32 {
        self.0.sub_block_count
    }
    pub fn get_min_m_index(&self) -> i32 {
        self.0.min_m_index
    }
    pub fn get_max_m_index(&self) -> i32 {
        self.0.max_m_index
    }
    pub fn get_bounding_box(&self) -> IntRect {
        IntRect(self.0.bounding_box)
    }
    pub fn get_bounding_box_layer0(&self) -> IntRect {
        IntRect(self.0.bounding_box_layer0)
    }
    pub fn get_dim_bounds(&self) -> DimBounds {
        DimBounds(self.0.dim_bounds)
    }
    pub fn set_sub_block_count(&mut self, sub_block_count: i32) {
        self.0.sub_block_count = sub_block_count;
    }
    pub fn set_min_m_index(&mut self, min_m_index: i32) {
        self.0.min_m_index = min_m_index;
    }
    pub fn set_max_m_index(&mut self, max_m_index: i32) {
        self.0.max_m_index = max_m_index;
    }
    pub fn set_bounding_box(&mut self, bounding_box: IntRect) {
        self.0.bounding_box = bounding_box.0
    }
    pub fn set_bounding_box_layer0(&mut self, bounding_box_layer0: IntRect) {
        self.0.bounding_box_layer0 = bounding_box_layer0.0
    }
    pub fn set_dim_bounds(&mut self, dim_bounds: DimBounds) {
        self.0.dim_bounds = dim_bounds.0
    }
}

impl MetadataAsXml {
    fn get_data(&self) -> Result<String> {
        let xml_data = unsafe {
            Vec::from_raw_parts(
                self.0.data as *mut u8,
                self.0.size as usize,
                self.0.size as usize,
            )
        };
        Ok(String::from_utf8(xml_data)?)
    }
}

impl Drop for MetadataAsXml {
    fn drop(&mut self) {
        unsafe {
            libCZI_Free(Box::into_raw(Box::new(self.0.data)) as *mut c_void);
        }
    }
}

impl TryFrom<&MetadataAsXml> for String {
    type Error = Error;

    fn try_from(value: &MetadataAsXml) -> std::result::Result<Self, Self::Error> {
        value.get_data()
    }
}

impl BitmapInfo {
    pub fn new(width: u32, height: u32, pixel_type: PixelType) -> Self {
        Self(BitmapInfoInterop {
            width,
            height,
            pixelType: pixel_type.into(),
        })
    }
    pub fn get_width(&self) -> u32 {
        self.0.width
    }
    pub fn get_height(&self) -> u32 {
        self.0.height
    }
    pub fn get_pixel_type(&self) -> Result<PixelType> {
        PixelType::try_from(self.0.pixelType)
    }
    pub fn set_width(&mut self, width: u32) {
        self.0.width = width;
    }
    pub fn set_height(&mut self, height: u32) {
        self.0.height = height;
    }
    pub fn set_pixel_type(&mut self, pixel_type: PixelType) {
        self.0.pixelType = pixel_type.into();
    }
}

impl BitmapLockInfo {
    pub fn get_data_roi(&self) -> Vec<u8> {
        unsafe {
            Vec::from_raw_parts(
                self.0.ptrDataRoi as *mut u8,
                self.0.size as usize,
                self.0.size as usize,
            )
        }
    }
}

impl SubBlockInfo {
    pub fn new(
        compression_mode_raw: i32,
        pixel_type: PixelType,
        coordinate: Coordinate,
        logical_rect: IntRect,
        physical_size: IntSize,
        m_index: i32,
    ) -> Self {
        Self(SubBlockInfoInterop {
            compression_mode_raw,
            pixel_type: pixel_type.into(),
            coordinate: coordinate.0,
            logical_rect: logical_rect.0,
            physical_size: physical_size.0,
            m_index,
        })
    }
    pub fn get_compression_mode_raw(&self) -> i32 {
        self.0.compression_mode_raw
    }
    pub fn get_pixel_type(&self) -> Result<PixelType> {
        PixelType::try_from(self.0.pixel_type)
    }
    pub fn get_coordinate(&self) -> Coordinate {
        Coordinate(self.0.coordinate)
    }
    pub fn get_logical_rect(&self) -> IntRect {
        IntRect(self.0.logical_rect)
    }
    pub fn get_physical_size(&self) -> IntSize {
        IntSize(self.0.physical_size)
    }
    pub fn get_m_index(&self) -> i32 {
        self.0.m_index
    }
    pub fn set_compression_mode_raw(&mut self, compression_mode_raw: i32) {
        self.0.compression_mode_raw = compression_mode_raw
    }
    pub fn set_pixel_type(&mut self, pixel_type: PixelType) {
        self.0.pixel_type = pixel_type.into();
    }
    pub fn set_coordinate(&mut self, coordinate: Coordinate) {
        self.0.coordinate = coordinate.0
    }
    pub fn set_logical_rect(&mut self, logical_rect: IntRect) {
        self.0.logical_rect = logical_rect.0
    }
    pub fn set_physical_size(&mut self, physical_size: IntSize) {
        self.0.physical_size = physical_size.0
    }
    pub fn set_m_index(&mut self, m_index: i32) {
        self.0.m_index = m_index
    }
}

impl AttachmentInfo {
    pub fn get_guid(&self) -> [u8; 16] {
        self.0.guid
    }
    pub fn get_content_file_type(&self) -> [u8; 9] {
        self.0.content_file_type
    }
    pub fn get_name(&self) -> Result<String> {
        Ok(
            CStr::from_bytes_until_nul(&self.0.name.iter().map(|&i| i as u8).collect::<Vec<_>>())?
                .to_str()?
                .to_string(),
        )
    }
    pub fn get_name_overflow(&self) -> bool {
        self.0.name_overflow
    }
    pub fn get_name_in_case_of_overflow(&self) -> Result<String> {
        Ok(
            unsafe { CString::from_raw(self.0.name_in_case_of_overflow as *mut c_char) }
                .to_str()?
                .to_string(),
        )
    }
}

impl Drop for AttachmentInfo {
    fn drop(&mut self) {
        if self.0.name_overflow {
            unsafe { libCZI_Free(self.0.name_in_case_of_overflow) }
        }
    }
}

impl FileHeaderInfo {
    pub fn new(guid: [u8; 16], major_version: i32, minor_version: i32) -> Self {
        Self(FileHeaderInfoInterop {
            guid,
            majorVersion: major_version,
            minorVersion: minor_version,
        })
    }
    pub fn get_guid(&self) -> [u8; 16] {
        self.0.guid
    }
    pub fn get_major_version(&self) -> i32 {
        self.0.majorVersion
    }
    pub fn get_minor_version(&self) -> i32 {
        self.0.minorVersion
    }
    pub fn set_guid(&mut self, guid: [u8; 16]) {
        self.0.guid = guid
    }
    pub fn set_major_version(&mut self, major_version: i32) {
        self.0.majorVersion = major_version
    }
    pub fn set_minor_version(&mut self, minor_version: i32) {
        self.0.minorVersion = minor_version
    }
}

impl AddSubBlockInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        coordinate: Coordinate,
        m_index_valid: u8,
        m_index: i32,
        x: i32,
        y: i32,
        logical_width: i32,
        logical_height: i32,
        physical_width: i32,
        physical_height: i32,
        pixel_type: PixelType,
        compression_mode_raw: i32,
        size_data: u32,
        data: &[u8],
        stride: u32,
        size_metadata: u32,
        metadata: &[u8],
        size_attachment: u32,
        attachment: &[u8],
    ) -> Self {
        let data = ManuallyDrop::new(data.to_vec());
        let metadata = ManuallyDrop::new(metadata.to_vec());
        let attachment = ManuallyDrop::new(attachment.to_vec());

        Self(AddSubBlockInfoInterop {
            coordinate: coordinate.0,
            m_index_valid,
            m_index,
            x,
            y,
            logical_width,
            logical_height,
            physical_width,
            physical_height,
            pixel_type: pixel_type.into(),
            compression_mode_raw,
            size_data,
            data: data.as_ptr() as *const c_void,
            stride,
            size_metadata,
            metadata: metadata.as_ptr() as *const c_void,
            size_attachment,
            attachment: attachment.as_ptr() as *const c_void,
        })
    }
    pub fn get_coordinate(&self) -> Coordinate {
        Coordinate(self.0.coordinate)
    }
    pub fn get_m_index_valid(&self) -> u8 {
        self.0.m_index_valid
    }
    pub fn get_m_index(&self) -> i32 {
        self.0.m_index
    }
    pub fn get_x(&self) -> i32 {
        self.0.x
    }
    pub fn get_y(&self) -> i32 {
        self.0.y
    }
    pub fn get_logical_width(&self) -> i32 {
        self.0.logical_width
    }
    pub fn get_logical_height(&self) -> i32 {
        self.0.logical_height
    }
    pub fn get_physical_width(&self) -> i32 {
        self.0.physical_width
    }
    pub fn get_physical_height(&self) -> i32 {
        self.0.physical_height
    }
    pub fn get_pixel_type(&self) -> Result<PixelType> {
        PixelType::try_from(self.0.pixel_type)
    }
    pub fn get_compression_mode_raw(&self) -> i32 {
        self.0.compression_mode_raw
    }
    pub fn get_size_data(&self) -> u32 {
        self.0.size_data
    }
    pub fn get_data(&self) -> Vec<u8> {
        unsafe {
            Vec::from_raw_parts(
                self.0.data as *mut u8,
                self.0.size_data as usize,
                self.0.size_data as usize,
            )
        }
    }
    pub fn get_size_metadata(&self) -> u32 {
        self.0.size_metadata
    }
    pub fn get_metadata(&self) -> Vec<u8> {
        unsafe {
            Vec::from_raw_parts(
                self.0.metadata as *mut u8,
                self.0.size_metadata as usize,
                self.0.size_metadata as usize,
            )
        }
    }
    pub fn get_size_attachment(&self) -> u32 {
        self.0.size_attachment
    }
    pub fn get_attachment(&self) -> Vec<u8> {
        unsafe {
            Vec::from_raw_parts(
                self.0.attachment as *mut u8,
                self.0.attachment as usize,
                self.0.attachment as usize,
            )
        }
    }
    pub fn set_coordinate(&mut self, coordinate: Coordinate) {
        self.0.coordinate = coordinate.0
    }
    pub fn set_m_index_valid(&mut self, m_index_valid: u8) {
        self.0.m_index_valid = m_index_valid
    }
    pub fn set_m_index(&mut self, m_index: i32) {
        self.0.m_index = m_index
    }
    pub fn set_x(&mut self, x: i32) {
        self.0.x = x
    }
    pub fn set_y(&mut self, y: i32) {
        self.0.y = y
    }
    pub fn set_logical_width(&mut self, logical_width: i32) {
        self.0.logical_width = logical_width
    }
    pub fn set_logical_height(&mut self, logical_height: i32) {
        self.0.logical_height = logical_height
    }
    pub fn set_physical_width(&mut self, physical_width: i32) {
        self.0.physical_width = physical_width
    }
    pub fn set_physical_height(&mut self, physical_height: i32) {
        self.0.physical_height = physical_height
    }
    pub fn set_pixel_type(&mut self, pixel_type: PixelType) {
        self.0.pixel_type = pixel_type.into()
    }
    pub fn set_compression_mode_raw(&mut self, compression_mode_raw: i32) {
        self.0.compression_mode_raw = compression_mode_raw
    }
    pub fn set_size_data(&mut self, size_data: u32) {
        self.0.size_data = size_data
    }
    pub fn set_data(&mut self, data: &[u8]) {
        let data = ManuallyDrop::new(data.to_vec());
        self.0.data = data.as_ptr() as *const c_void;
        self.0.size_data = data.len() as u32;
    }
    pub fn set_size_metadata(&mut self, size_metadata: u32) {
        self.0.size_metadata = size_metadata
    }
    pub fn set_metadata(&mut self, metadata: &[u8]) {
        let metadata = ManuallyDrop::new(metadata.to_vec());
        self.0.metadata = metadata.as_ptr() as *const c_void;
        self.0.size_metadata = metadata.len() as u32;
    }
    pub fn set_size_attachment(&mut self, size_attachment: u32) {
        self.0.size_attachment = size_attachment
    }
    pub fn set_attachment(&mut self, attachment: &[u8]) {
        let attachment = ManuallyDrop::new(attachment.to_vec());
        self.0.attachment = attachment.as_ptr() as *const c_void;
        self.0.size_attachment = attachment.len() as u32;
    }
}

impl AddAttachmentInfo {
    pub fn new(
        guid: [u8; 16],
        content_file_type: [u8; 8],
        name: [u8; 80],
        size_attachment_data: u32,
        attachment_data: &[u8],
    ) -> Self {
        let attachment_data = ManuallyDrop::new(attachment_data.to_vec());
        Self(AddAttachmentInfoInterop {
            guid,
            contentFileType: content_file_type,
            name,
            size_attachment_data,
            attachment_data: attachment_data.as_ptr() as *const c_void,
        })
    }
    pub fn get_guid(&self) -> [u8; 16] {
        self.0.guid
    }
    pub fn get_content_file_type(&self) -> [u8; 8] {
        self.0.contentFileType
    }
    pub fn get_name(&self) -> [u8; 80] {
        self.0.name
    }
    pub fn get_size_attachment_data(&self) -> u32 {
        self.0.size_attachment_data
    }
    pub fn get_attachment_data(&self) -> Vec<u8> {
        unsafe {
            Vec::from_raw_parts(
                self.0.attachment_data as *mut u8,
                self.0.attachment_data as usize,
                self.0.attachment_data as usize,
            )
        }
    }
    pub fn set_guid(&mut self, guid: [u8; 16]) {
        self.0.guid = guid
    }
    pub fn set_content_file_type(&mut self, content_file_type: [u8; 8]) {
        self.0.contentFileType = content_file_type
    }
    pub fn set_name(&mut self, name: [u8; 80]) {
        self.0.name = name
    }
    pub fn set_size_attachment_data(&mut self, size_attachment_data: u32) {
        self.0.size_attachment_data = size_attachment_data
    }
    pub fn set_attachment_data(&mut self, attachment_data: &[u8]) {
        let attachment_data = ManuallyDrop::new(attachment_data.to_vec());
        self.0.attachment_data = attachment_data.as_ptr() as *const c_void;
        self.0.size_attachment_data = attachment_data.len() as u32;
    }
}

impl WriteMetadataInfo {
    pub fn new(size_metadata: u32, metadata: &[u8]) -> Self {
        let metadata = ManuallyDrop::new(metadata.to_vec());
        Self(WriteMetadataInfoInterop {
            size_metadata,
            metadata: metadata.as_ptr() as *const c_void,
        })
    }
    pub fn get_size_metadata(&self) -> u32 {
        self.0.size_metadata
    }
    pub fn get_metadata(&self) -> Vec<u8> {
        unsafe {
            Vec::from_raw_parts(
                self.0.metadata as *mut u8,
                self.0.metadata as usize,
                self.0.metadata as usize,
            )
        }
    }
    pub fn set_size_metadata(&mut self, size_metadata: u32) {
        self.0.size_metadata = size_metadata
    }
    pub fn set_metadata(&mut self, metadata: &[u8]) {
        let metadata = ManuallyDrop::new(metadata.to_vec());
        self.0.metadata = metadata.as_ptr() as *const c_void;
        self.0.size_metadata = metadata.len() as u32;
    }
}

impl AccessorOptions {
    pub fn new<S: AsRef<str>>(
        back_ground_color_r: f32,
        back_ground_color_g: f32,
        back_ground_color_b: f32,
        sort_by_m: bool,
        use_visibility_check_optimization: bool,
        additional_parameters: S,
    ) -> Result<Self> {
        let additional_parameters =
            ManuallyDrop::new(CString::new(additional_parameters.as_ref())?);
        Ok(Self(AccessorOptionsInterop {
            back_ground_color_r,
            back_ground_color_g,
            back_ground_color_b,
            sort_by_m,
            use_visibility_check_optimization,
            additional_parameters: additional_parameters.as_ptr(),
        }))
    }
    pub fn get_background_color_r(&self) -> f32 {
        self.0.back_ground_color_r
    }
    pub fn get_background_color_g(&self) -> f32 {
        self.0.back_ground_color_g
    }
    pub fn get_background_color_b(&self) -> f32 {
        self.0.back_ground_color_b
    }
    pub fn get_sort_by_m(&self) -> bool {
        self.0.sort_by_m
    }
    pub fn get_use_visibility_check_optimization(&self) -> bool {
        self.0.use_visibility_check_optimization
    }
    pub fn get_additional_parameters(&self) -> Result<String> {
        Ok(unsafe { CStr::from_ptr(self.0.additional_parameters) }
            .to_str()?
            .to_string())
    }
    pub fn set_background_color_r(&mut self, back_ground_color_r: f32) {
        self.0.back_ground_color_r = back_ground_color_r
    }
    pub fn set_background_color_g(&mut self, back_ground_color_g: f32) {
        self.0.back_ground_color_g = back_ground_color_g
    }
    pub fn set_background_color_b(&mut self, back_ground_color_b: f32) {
        self.0.back_ground_color_b = back_ground_color_b
    }
    pub fn set_sort_by_m(&mut self, sort_by_m: bool) {
        self.0.sort_by_m = sort_by_m
    }
    pub fn set_use_visibility_check_optimization(
        &mut self,
        use_visibility_check_optimization: bool,
    ) {
        self.0.use_visibility_check_optimization = use_visibility_check_optimization
    }
    pub fn set_additional_parameters<S: AsRef<str>>(
        &mut self,
        additional_parameters: S,
    ) -> Result<()> {
        let additional_parameters =
            ManuallyDrop::new(CString::new(additional_parameters.as_ref())?);
        self.0.additional_parameters = additional_parameters.as_ptr();
        Ok(())
    }
}

impl CompositionChannelInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        weight: f32,
        enable_tinting: u8,
        tinting_color_r: u8,
        tinting_color_g: u8,
        tinting_color_b: u8,
        black_point: f32,
        white_point: f32,
        look_up_table_element_count: i32,
        look_up_table: &[u8],
    ) -> Self {
        let mut look_up_table = ManuallyDrop::new(look_up_table.to_vec());
        Self(CompositionChannelInfoInterop {
            weight,
            enable_tinting,
            tinting_color_r,
            tinting_color_g,
            tinting_color_b,
            black_point,
            white_point,
            look_up_table_element_count,
            ptr_look_up_table: look_up_table.as_mut_ptr(),
        })
    }
    pub fn get_weight(&self) -> f32 {
        self.0.weight
    }
    pub fn get_enable_tinting(&self) -> u8 {
        self.0.enable_tinting
    }
    pub fn get_tinting_color_r(&self) -> u8 {
        self.0.tinting_color_r
    }
    pub fn get_tinting_color_g(&self) -> u8 {
        self.0.tinting_color_g
    }
    pub fn get_tinting_color_b(&self) -> u8 {
        self.0.tinting_color_b
    }
    pub fn get_black_point(&self) -> f32 {
        self.0.black_point
    }
    pub fn get_white_point(&self) -> f32 {
        self.0.white_point
    }
    pub fn get_look_up_table_element_count(&self) -> i32 {
        self.0.look_up_table_element_count
    }
    pub fn get_look_up_table(&self) -> Vec<u8> {
        unsafe {
            Vec::from_raw_parts(
                self.0.ptr_look_up_table,
                self.0.look_up_table_element_count as usize,
                self.0.look_up_table_element_count as usize,
            )
        }
    }
    pub fn set_weight(&mut self, weight: f32) {
        self.0.weight = weight
    }
    pub fn set_enable_tinting(&mut self, enable_tinting: u8) {
        self.0.enable_tinting = enable_tinting
    }
    pub fn set_tinting_color_r(&mut self, tinting_color_r: u8) {
        self.0.tinting_color_r = tinting_color_r
    }
    pub fn set_tinting_color_g(&mut self, tinting_color_g: u8) {
        self.0.tinting_color_g = tinting_color_g
    }
    pub fn set_tinting_color_b(&mut self, tinting_color_b: u8) {
        self.0.tinting_color_b = tinting_color_b
    }
    pub fn set_black_point(&mut self, black_point: f32) {
        self.0.black_point = black_point
    }
    pub fn set_white_point(&mut self, white_point: f32) {
        self.0.white_point = white_point
    }
    pub fn set_look_up_table_element_count(&mut self, look_up_table_element_count: i32) {
        self.0.look_up_table_element_count = look_up_table_element_count
    }
    pub fn set_look_up_table(&mut self, look_up_table: &[u8]) {
        let mut look_up_table = ManuallyDrop::new(look_up_table.to_vec());
        self.0.ptr_look_up_table = look_up_table.as_mut_ptr();
        self.0.look_up_table_element_count = look_up_table.len() as i32;
    }
}

impl ScalingInfo {
    pub fn new(scale_x: f64, scale_y: f64, scale_z: f64) -> Self {
        Self(ScalingInfoInterop {
            scale_x,
            scale_y,
            scale_z,
        })
    }
    pub fn get_scale_x(&self) -> f64 {
        self.0.scale_x
    }
    pub fn get_scale_y(&self) -> f64 {
        self.0.scale_y
    }
    pub fn get_scale_z(&self) -> f64 {
        self.0.scale_z
    }
    pub fn set_scale_x(&mut self, scale_x: f64) {
        self.0.scale_x = scale_x
    }
    pub fn set_scale_y(&mut self, scale_y: f64) {
        self.0.scale_y = scale_y
    }
    pub fn set_scale_z(&mut self, scale_z: f64) {
        self.0.scale_z = scale_z
    }
}
