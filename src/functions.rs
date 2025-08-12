use crate::handle::*;
use crate::interop::*;
use crate::misc::*;
use crate::sys::*;
use anyhow::{Error, Result};
use std::ffi::{CStr, CString, c_char, c_int, c_ulong, c_void};
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::Deref;

/// Release the memory - this function is to be used for freeing memory allocated by the libCZIApi-library
///  (and returned to the caller).
///
///  \\param  data    Pointer to the memory to be freed.
pub fn free<T: Ptr>(data: T) {
    let ptr = data.as_mut_ptr() as *mut c_void;
    unsafe { libCZI_Free(ptr) };
}

/// Allocate memory of the specified size.
///
///  \\param          size    The size of the memory block to be allocated in bytes.
///  \\param \[out\]    data    If successful, a pointer to the allocated memory is put here. The memory must be freed using 'libCZI_Free'.
///
///  \\returns    An error-code indicating success or failure of the operation.
pub fn allocate_memory<T: Ptr>(size: usize) -> Result<MaybeUninit<T>> {
    let mut data = MaybeUninit::<T>::uninit();
    let mut ptr = data.as_mut_ptr() as *mut c_void;
    LibCZIApiError::try_from(unsafe { libCZI_AllocateMemory(size as c_ulong, &mut ptr) })?;
    Ok(data)
}

impl LibCZIVersionInfo {
    /// Get version information about the libCZIApi-library.
    ///
    ///  \\param \[out\] version_info    If successful, the version information is put here.
    ///
    ///  \\returns    An error-code indicating success or failure of the operation.
    pub fn get_lib_czi_version_info() -> Result<LibCZIVersionInfo> {
        let mut version_info = MaybeUninit::uninit();
        let ptr = version_info.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_GetLibCZIVersionInfo(ptr) })?;
        Ok(unsafe { LibCZIVersionInfo::assume_init(version_info) })
    }
}

impl LibCZIBuildInformation {
    /// Get information about the build of the libCZIApi-library.
    ///
    ///  \\param \[out\] build_info  If successful, the build information is put here. Note that all strings must be freed by the caller (using 'libCZI_Free').
    ///
    ///  \\returns    An error-code indicating success or failure of the operation.
    pub fn get() -> Result<LibCZIBuildInformation> {
        let mut build_info = MaybeUninit::uninit();
        let ptr = build_info.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_GetLibCZIBuildInformation(ptr) })?;
        Ok(unsafe { LibCZIBuildInformation::assume_init(build_info) })
    }
}

impl CziReader {
    /// Create a new CZI-reader object.
    ///
    ///  \\param \[out\] reader_object If the operation is successful, a handle to the newly created reader object is put here.
    ///
    ///  \\returns    An error-code indicating success or failure of the operation.
    pub fn create() -> Result<Self> {
        let mut reader = MaybeUninit::uninit();
        let ptr = reader.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_CreateReader(ptr) })?;
        Ok(unsafe { Self::assume_init(reader) })
    }

    /// Instruct the specified reader-object to open a CZI-document. The 'open_info' parameter contains
    ///  a handle to a stream-object which is used to read the document.
    ///
    ///  \\param  reader_object A handle representing the reader-object.
    ///  \\param  open_info     Parameters controlling the operation.
    ///
    ///  \\returns    An error-code indicating success or failure of the operation.
    pub fn open(&self, open_info: ReaderOpenInfo) -> Result<()> {
        LibCZIApiError::try_from(unsafe { libCZI_ReaderOpen(**self, open_info.as_ptr()) })?;
        Ok(())
    }

    /// Get information about the file-header of the CZI document. The information is put into the 'file_header_info_interop' structure.
    ///  This file_header_info_interop structure contains the GUID of the CZI document and the version levels of CZI.
    ///
    ///  \\param          reader_object               The reader object.
    ///  \\param \[out\]    file_header_info_interop    If successful, the retrieved information is put here.
    ///
    ///  \\returns An error-code indicating success or failure of the operation.
    pub fn get_file_header_info(&self) -> Result<FileHeaderInfo> {
        let mut file_header_info = MaybeUninit::uninit();
        let ptr = file_header_info.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_ReaderGetFileHeaderInfo(**self, ptr) })?;
        Ok(unsafe { FileHeaderInfo::assume_init(file_header_info) })
    }

    /// Reads the sub-block identified by the specified index. If there is no sub-block present (for the
    ///  specified index) then the function returns 'LibCZIApi_ErrorCode_OK', but the 'sub_block_object'
    ///  is set to 'kInvalidObjectHandle'.
    ///
    ///  \\param          reader_object       The reader object.
    ///  \\param          index               Index of the sub-block.
    ///  \\param \[out\]    sub_block_object    If successful, a handle to the sub-block object is put here; otherwise 'kInvalidObjectHandle'.
    ///
    ///  \\returns    An error-code indicating success or failure of the operation.
    pub fn read_sub_block(&self, index: i32) -> Result<SubBlock> {
        let mut sub_block = MaybeUninit::uninit();
        let ptr = sub_block.as_mut_ptr();
        LibCZIApiError::try_from(unsafe {
            libCZI_ReaderReadSubBlock(**self, index as c_int, ptr)
        })?;
        Ok(unsafe { SubBlock::assume_init(sub_block) })
    }

    /// Get statistics about the sub-blocks in the CZI-document. This function provides a simple version of the statistics, the
    ///  information retrieved does not include the per-scene statistics.
    ///
    ///  \\param          reader_object   The reader object.
    ///  \\param \[out\]    statistics      If non-null, the simple statistics will be put here.
    ///
    ///  \\returns    An error-code indicating success or failure of the operation.
    pub fn get_statistics_simple(&self) -> Result<SubBlockStatistics> {
        let mut statistics = MaybeUninit::uninit();
        let ptr = statistics.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_ReaderGetStatisticsSimple(**self, ptr) })?;
        Ok(unsafe { SubBlockStatistics::assume_init(statistics) })
    }

    /// Get extended statistics about the sub-blocks in the CZI-document. This function provides a more detailed version of the statistics,
    ///  including the per-scene statistics. Note that the statistics is of variable size, and the semantic is as follows:
    ///  - On input, the argument 'number_of_per_channel_bounding_boxes' must point to an integer which describes the size of the argument 'statistics'.
    ///    This number gives how many elements the array 'per_scenes_bounding_boxes' in 'SubBlockStatisticsInteropEx' can hold. Only that number of
    ///    per-scene information elements will be put into the 'statistics' structure at most, in any case.
    ///  - On output, the argument 'number_of_per_channel_bounding_boxes' will be set to the number of per-channel bounding boxes that were actually
    ///    available.
    ///  - In the returned 'SubBlockStatisticsInteropEx' structure, the 'number_of_per_scenes_bounding_boxes' field will be set to the number of per-scene
    ///    information that is put into this struct (which may be less than number of scenes that are available).
    ///
    ///  So, the caller is expected to check the returned 'number_of_per_channel_bounding_boxes' to see how many per-channel bounding boxes are available.
    ///  If this number is greater than the number of elements (given with the 'number_of_per_scenes_bounding_boxes' value in the 'statistics' structure),
    ///  then the caller should allocate a larger 'statistics' structure and call this function again (with an increased 'number_of_per_scenes_bounding_boxes').
    ///
    ///  \\param          reader_object                           The reader object.
    ///  \\param \[out\]    statistics                              If non-null, the statistics will be put here.
    ///  \\param \[in,out\] number_of_per_channel_bounding_boxes    On input, it gives the number of elements that can be put into the 'per_scenes_bounding_boxes' array.
    ///                                                          On output, it gives the number of elements which are available.
    ///
    ///  \\returns    An error-code indicating success or failure of the operation.
    pub fn get_statistics_ex(
        &self,
        number_of_per_channel_bounding_boxes: i32,
    ) -> Result<(SubBlockStatisticsEx, i32)> {
        let mut statistics = MaybeUninit::uninit();
        let ptr = statistics.as_mut_ptr();
        let number_of_per_channel_bounding_boxes =
            Box::into_raw(Box::new(number_of_per_channel_bounding_boxes));
        LibCZIApiError::try_from(unsafe {
            libCZI_ReaderGetStatisticsEx(**self, ptr, number_of_per_channel_bounding_boxes)
        })?;
        Ok(unsafe {
            (
                SubBlockStatisticsEx::assume_init(statistics),
                *Box::from_raw(number_of_per_channel_bounding_boxes),
            )
        })
    }

    /// Get \"pyramid-statistics\" about the CZI-document. This function provides a JSON-formatted string which contains information about the pyramid.
    ///  The JSON-schema is as follows:
    ///  \\code
    ///  {
    ///      \"scenePyramidStatistics\": {
    ///          \<sceneIndex\>: [
    ///          {
    ///              \"layerInfo\": {
    ///              \"minificationFactor\": \<number\>,
    ///              \"pyramidLayerNo\" : \<number\>
    ///          },
    ///          \"count\" : \<number\>
    ///          }
    ///      ]}
    ///  }
    ///  \\endcode
    ///  It resembles the corresponding C++-structure 'PyramidStatistics' in the libCZI-library.
    ///
    ///  \\param          reader_object              The reader object.
    ///  \\param \[out\]    pyramid_statistics_as_json If successful, a pointer to a JSON-formatted string is placed here. The caller
    ///                                              is responsible for freeing this memory (by calling libCZI_Free).
    ///
    ///  \\returns An error-code indicating success or failure of the operation.
    pub fn get_pyramid_statistics(&self) -> Result<String> {
        let mut ptr = MaybeUninit::<*mut c_char>::uninit();
        LibCZIApiError::try_from(unsafe {
            libCZI_ReaderGetPyramidStatistics(**self, ptr.as_mut_ptr())
        })?;
        let ptr = unsafe { ptr.assume_init() };
        let statistics = unsafe { CStr::from_ptr(ptr) }.to_str()?.to_owned();
        unsafe { libCZI_Free(ptr as *mut c_void) };
        Ok(statistics)
    }

    /// Create a metadata-segment object from the reader-object. The metadata-segment object can be used to retrieve the XML-metadata of the CZI-document.
    ///
    /// \\param          reader_object           The reader object.
    /// \\param \[out\]    metadata_segment_object If successful, a handle to the metadata-segment object is put here.
    ///
    /// \\returns An error-code indicating success or failure of the operation.
    pub fn get_metadata_segment(&self) -> Result<MetadataSegment> {
        let mut metadata_segment = MaybeUninit::uninit();
        let ptr = metadata_segment.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_ReaderGetMetadataSegment(**self, ptr) })?;
        Ok(unsafe { MetadataSegment::assume_init(metadata_segment) })
    }

    /// Get the number of attachments available.
    ///
    /// \\param          reader_object           The reader object.
    /// \\param \[out\]    count                   The number of available attachments is put here.
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn get_attachment_count(&self) -> Result<i32> {
        let mut count = MaybeUninit::<c_int>::uninit();
        LibCZIApiError::try_from(unsafe {
            libCZI_ReaderGetAttachmentCount(**self, count.as_mut_ptr())
        })?;
        Ok(unsafe { count.assume_init() })
    }

    /// Get information about the attachment at the specified index. The information is put into the 'attachment_info_interop' structure.
    /// If the index is not valid, then the function returns 'LibCZIApi_ErrorCode_IndexOutOfRange'.
    ///
    /// \\param          reader_object           The reader object.
    /// \\param          index                   The index of the attachment to query information for.
    /// \\param \[out\]    attachment_info_interop If successful, the retrieved information is put here.
    ///
    /// \\returns An error-code indicating success or failure of the operation.
    pub fn get_attachment_info_from_directory(&self, index: i32) -> Result<AttachmentInfo> {
        let mut attachment_info = MaybeUninit::uninit();
        let ptr = attachment_info.as_mut_ptr();
        LibCZIApiError::try_from(unsafe {
            libCZI_ReaderGetAttachmentInfoFromDirectory(**self, index, ptr)
        })?;
        Ok(unsafe { AttachmentInfo::assume_init(attachment_info) })
    }

    /// Read the attachment with the specified index and create an attachment object representing it. If the specified index
    /// is invalid, then the returned attachment-object handle will have the value 'kInvalidObjectHandle'.
    /// \\param       reader_object              The reader object.
    /// \\param       index                      The index of the attachment to get.
    /// \\param \[out\] attachment_object          If successful and index is valid, a handle representing the attachment object is put here. If the index is
    ///                                         invalid, then the handle will have the value 'kInvalidObjectHandle'.
    /// \\returns  An error-code indicating success or failure of the operation.
    pub fn read_attachment(&self, index: i32) -> Result<Attachment> {
        let mut attachment = MaybeUninit::uninit();
        let ptr = attachment.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_ReaderReadAttachment(**self, index, ptr) })?;
        Ok(unsafe { Attachment::assume_init(attachment) })
    }

    /// Release the specified reader-object. After this function is called, the handle is no
    /// longer valid.
    ///
    /// \\param  reader_object   The reader object.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn release(&self) -> Result<()> {
        LibCZIApiError::try_from(unsafe { libCZI_ReleaseReader(**self) })?;
        Ok(())
    }

    /// Get information about the sub-block with the specified index. The information is put into the 'sub_block_info_interop' structure.
    /// If the index is not valid, then the function returns 'LibCZIApi_ErrorCode_IndexOutOfRange'.
    ///
    /// \\param          reader_object           The reader object.
    /// \\param          index                   The index of the attachment to query information for.
    /// \\param \[out\]    sub_block_info_interop  If successful, the retrieved information is put here.
    ///
    /// \\returns An error-code indicating success or failure of the operation.
    pub fn try_get_sub_block_info_for_index(&self, index: i32) -> Result<SubBlockInfo> {
        let mut sub_block_info = MaybeUninit::uninit();
        let ptr = sub_block_info.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_TryGetSubBlockInfoForIndex(**self, index, ptr) })?;
        Ok(unsafe { SubBlockInfo::assume_init(sub_block_info) })
    }

    /// Create a single channel scaling tile accessor.
    ///
    /// \\param reader_object            A handle representing the reader-object.
    /// \\param accessor_object \[out\]    If the operation is successful, a handle to the newly created single-channel-scaling-tile-accessor is put here.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn create_single_channel_tile_accessor(&self) -> Result<SingleChannelScalingTileAccessor> {
        let mut accessor = MaybeUninit::uninit();
        let ptr = accessor.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_CreateSingleChannelTileAccessor(**self, ptr) })?;
        Ok(unsafe { SingleChannelScalingTileAccessor::assume_init(accessor) })
    }
}

impl Drop for CziReader {
    fn drop(&mut self) {
        self.release().ok();
    }
}

/// Get information about the stream class at the specified index.
///
/// \\param          index                   Zero-based index of the stream class to query information about.
/// \\param \[out\]    input_stream_class_info If successful, information about the stream class is put here. Note that the strings in the structure
///                                         must be freed (by the caller) using 'libCZI_Free'.
///
/// \\returns An error-code indicating success or failure of the operation.
pub fn get_stream_classes_count(index: i32) -> Result<InputStreamClassInfo> {
    let mut input_stream_class_info = MaybeUninit::uninit();
    let ptr = input_stream_class_info.as_mut_ptr();
    LibCZIApiError::try_from(unsafe { libCZI_GetStreamClassInfo(index, ptr) })?;
    Ok(unsafe { InputStreamClassInfo::assume_init(input_stream_class_info) })
}

impl InputStream {
    /// Create an input stream object of the specified type, using the specified JSON-formatted property bag and
    /// the specified file identifier as input.
    ///
    /// \\param          stream_class_name       Name of the stream class to be instantiated.
    /// \\param          creation_property_bag   JSON formatted string (containing additional parameters for the stream creation) in UTF8-encoding.
    /// \\param          stream_identifier       The filename (or, more generally, a URI of some sort) identifying the file to be opened in UTF8-encoding.
    /// \\param \[out\]    stream_object           If successful, a handle representing the newly created stream object is put here.
    ///
    /// \\returns    An error-code that indicates whether the operation is successful or not.
    pub fn create(
        stream_class_name: impl AsRef<str>,
        creation_property_bag: impl AsRef<str>,
        stream_identifier: impl AsRef<str>,
    ) -> Result<Self> {
        let mut stream = MaybeUninit::uninit();
        let ptr = stream.as_mut_ptr();
        let stream_class_name = ManuallyDrop::new(CString::new(stream_class_name.as_ref())?);
        let creation_property_bag =
            ManuallyDrop::new(CString::new(creation_property_bag.as_ref())?);
        let stream_identifier = ManuallyDrop::new(CString::new(stream_identifier.as_ref())?);
        LibCZIApiError::try_from(unsafe {
            libCZI_CreateInputStream(
                stream_class_name.as_ptr(),
                creation_property_bag.as_ptr(),
                stream_identifier.as_ptr(),
                ptr,
            )
        })?;
        Ok(unsafe { Self::assume_init(stream) })
    }

    /// Create an input stream object for a file identified by its filename, which is given as a wide string. Note that wchar_t on
    /// Windows is 16-bit wide, and on Unix-like systems it is 32-bit wide.
    ///
    /// \\param  \[in\]    filename        Filename of the file which is to be opened (zero terminated wide string). Note that on Windows, this
    ///                                 is a string with 16-bit code units, and on Unix-like systems it is typically a string with 32-bit code units.
    ///
    /// \\param  \[out\]   stream_object   The output stream object that will hold the created stream.
    /// \\return         An error-code that indicates whether the operation is successful or not. Non-positive values indicates successful, positive values
    ///                 indicates unsuccessful operation.
    pub fn create_from_file_wide(file_name: Vec<u32>) -> Result<Self> {
        let mut stream = MaybeUninit::uninit();
        let ptr = stream.as_mut_ptr();
        LibCZIApiError::try_from(unsafe {
            libCZI_CreateInputStreamFromFileWide(file_name.as_ptr(), ptr)
        })?;
        Ok(unsafe { Self::assume_init(stream) })
    }

    /// Create an input stream object for a file identified by its filename, which is given as an UTF8-encoded string.
    ///
    /// \\param  \[in\]    filename        Filename of the file which is to be opened (in UTF8 encoding).
    /// \\param  \[out\]   stream_object   The output stream object that will hold the created stream.
    /// \\return         An error-code that indicates whether the operation is successful or not. Non-positive values indicates successful, positive values
    ///                 indicates unsuccessful operation.
    pub fn create_from_file_utf8<S: AsRef<str>>(file_name: S) -> Result<Self> {
        let mut stream = MaybeUninit::uninit();
        let ptr = stream.as_mut_ptr();
        let file_name = ManuallyDrop::new(CString::new(file_name.as_ref())?);
        // let file_name = file_name.as_ref().as_bytes().to_vec();
        LibCZIApiError::try_from(unsafe {
            libCZI_CreateInputStreamFromFileUTF8(file_name.as_ptr() as *const c_char, ptr)
        })?;
        Ok(unsafe { Self::assume_init(stream) })
    }

    /// Create an input stream object which is using externally provided functions for operation
    /// and reading the data. Please refer to the documentation of
    /// 'ExternalInputStreamStructInterop' for more information.
    ///
    /// \\param          external_input_stream_struct    Structure containing the information about the externally provided functions.
    /// \\param \[out\]    stream_object                   If successful, the handle to the newly created input stream object is put here.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn create_from_external(external_input_stream: ExternalInputStreamStruct) -> Result<Self> {
        let mut stream = MaybeUninit::uninit();
        let ptr = stream.as_mut_ptr();
        LibCZIApiError::try_from(unsafe {
            libCZI_CreateInputStreamFromExternal(external_input_stream.as_ptr(), ptr)
        })?;
        Ok(unsafe { Self::assume_init(stream) })
    }

    /// Release the specified input stream object. After this function is called, the handle is no
    /// longer valid. Note that calling this function will only decrement the usage count of the
    /// underlying object; whereas the object itself (and the resources it holds) will only be
    /// released when the usage count reaches zero.
    ///
    /// \\param  stream_object   The input stream object.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn release(&self) -> Result<()> {
        LibCZIApiError::try_from(unsafe { libCZI_ReleaseInputStream(**self) })?;
        Ok(())
    }
}

impl Drop for InputStream {
    fn drop(&mut self) {
        self.release().ok();
    }
}

impl SubBlock {
    /// Create a bitmap object from the specified sub-block object. The bitmap object can be used to access the pixel
    /// data contained in the sub-block. If the subblock contains compressed data, then decompression will be performed
    /// in this call.
    ///
    /// \\param          sub_block_object The sub-block object.
    /// \\param \[out\]    bitmap_object    If successful, the handle to the newly created bitmap object is put here.
    ///
    /// \\returns An error-code indicating success or failure of the operation.
    pub fn create_bitmap(&self) -> Result<Bitmap> {
        let mut bitmap = MaybeUninit::uninit();
        let ptr = bitmap.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_SubBlockCreateBitmap(**self, ptr) })?;
        Ok(unsafe { Bitmap::assume_init(bitmap) })
    }

    /// Get Information about the sub-block.
    ///
    /// \\param       sub_block_object The sub-block object.
    /// \\param \[out\] sub_block_info   If successful, information about the sub-block object is put here.
    ///
    /// \\returns An error-code indicating success or failure of the operation.
    pub fn get_info(&self) -> Result<SubBlockInfo> {
        let mut sub_block_info = MaybeUninit::uninit();
        let ptr = sub_block_info.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_SubBlockGetInfo(**self, ptr) })?;
        Ok(unsafe { SubBlockInfo::assume_init(sub_block_info) })
    }

    /// Copy the raw data from the specified sub-block object to the specified memory buffer. The value of the 'size' parameter
    /// on input is the size of the buffer pointed to by 'data'. On output, the value of 'size' is the actual size of the data. At most
    /// the initial value of 'size' bytes are copied to the buffer. If the initial value of 'size' is zero (0) or 'data' is null, then
    /// no data is copied.
    /// For the 'type' parameter, the following values are valid: 0 (data) and 1 (metadata).
    /// For 0 (data), the data is the raw pixel data of the bitmap. This data may be compressed.
    /// For 1 (metadata), the data is the raw metadata in XML-format (UTF8-encoded).
    ///
    /// \\param          sub_block_object    The sub block object.
    /// \\param          type                The type - 0 for \"pixel-data\", 1 for \"sub-block metadata\".
    /// \\param \[in,out\] size                On input, the size of the memory block pointed to by 'data', on output the actual size of the available data.
    /// \\param \[out\]    data                Pointer where the data is to be copied to. At most the initial content of 'size' bytes are copied.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn get_raw_data(&self, tp: RawDataType, size: i32) -> Result<(i32, Vec<u8>)> {
        let mut data = Vec::<u8>::with_capacity(size as usize);
        let size = Box::into_raw(Box::new(size as c_ulong));
        LibCZIApiError::try_from(unsafe {
            libCZI_SubBlockGetRawData(**self, tp as c_int, size, data.as_mut_ptr() as *mut c_void)
        })?;
        Ok((unsafe { *Box::from_raw(size) as i32 }, data))
    }

    /// Release the specified sub-block object.
    ///
    /// \\param  sub_block_object The sub block object to be released.
    ///
    /// \\returns An error-code indicating success or failure of the operation.
    pub fn release(&self) -> Result<()> {
        LibCZIApiError::try_from(unsafe { libCZI_ReleaseSubBlock(**self) })?;
        Ok(())
    }
}

impl Drop for SubBlock {
    fn drop(&mut self) {
        self.release().ok();
    }
}

impl Attachment {
    /// Get information about the specified attachment object.
    /// \\param attachment_object            The attachment object.
    /// \\param \[out\]    attachment_info     Information about the attachment.
    /// \\returns     An error-code indicating success or failure of the operation.
    pub fn get_info(&self) -> Result<AttachmentInfo> {
        let mut attachment_info = MaybeUninit::uninit();
        let ptr = attachment_info.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_AttachmentGetInfo(**self, ptr) })?;
        Ok(unsafe { AttachmentInfo::assume_init(attachment_info) })
    }

    /// Copy the raw data from the specified attachment object to the specified memory buffer. The value of the 'size' parameter
    /// on input is the size of the buffer pointed to by 'data'. On output, the value of 'size' is the actual size of the data. At most
    /// the initial value of 'size' bytes are copied to the buffer. If the initial value of 'size' is zero (0) or 'data' is null, then
    /// no data is copied.
    /// \\param          attachment_object   The attachment object.
    /// \\param \[in,out\] size                On input, the size of the memory block pointed to by 'data', on output the actual size of the available data.
    /// \\param \[out\]    data                Pointer where the data is to be copied to. At most the initial content of 'size' bytes are copied.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn get_raw_data(&self, size: i32) -> Result<(i32, Vec<u8>)> {
        let mut data = Vec::<u8>::with_capacity(size as usize);
        let size = Box::into_raw(Box::new(size as c_ulong));
        LibCZIApiError::try_from(unsafe {
            libCZI_AttachmentGetRawData(**self, size, data.as_mut_ptr() as *mut c_void)
        })?;
        Ok((unsafe { *Box::from_raw(size) as i32 }, data))
    }

    /// Release the specified attachment object.
    ///
    /// \\param  attachment_object The attachment object to be released.
    ///
    /// \\returns An error-code indicating success or failure of the operation.
    pub fn release(&self) -> Result<()> {
        LibCZIApiError::try_from(unsafe { libCZI_ReleaseAttachment(**self) })?;
        Ok(())
    }
}

impl Drop for Attachment {
    fn drop(&mut self) {
        self.release().ok();
    }
}

impl Bitmap {
    /// Get information about the specified bitmap object.
    ///
    /// \\param          bitmap_object The bitmap object.
    /// \\param \[out\]    bitmap_info   If successful, information about the bitmap object is put here.
    ///
    /// \\returns An error-code indicating success or failure of the operation.
    pub fn get_info(&self) -> Result<BitmapInfo> {
        let mut bitmap_info = MaybeUninit::uninit();
        let ptr = bitmap_info.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_BitmapGetInfo(**self, ptr) })?;
        Ok(unsafe { BitmapInfo::assume_init(bitmap_info) })
    }

    /// Locks the bitmap object. Once the bitmap is locked, the pixel data can be accessed. Memory access to the
    /// pixel data must only occur while the bitmap is locked. The lock must be released by calling 'libCZI_BitmapUnlock'.
    /// It is a fatal error if the bitmap is destroyed while still being locked. Calls to Lock and Unlock are counted, and
    /// they must be balanced.
    ///
    /// \\param          bitmap_object The bitmap object.
    /// \\param \[out\]    lockInfo      If successful, information about how to access the pixel data is put here.
    ///
    /// \\returns An error-code indicating success or failure of the operation.
    pub fn lock(self) -> Result<LockedBitmap> {
        let mut bitmap_info = MaybeUninit::uninit();
        let ptr = bitmap_info.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_BitmapLock(*self, ptr) })?;
        let bitmap_lock_info = unsafe { BitmapLockInfo::assume_init(bitmap_info) };
        Ok(LockedBitmap {
            bitmap: self,
            lock_info: bitmap_lock_info,
        })
    }

    /// Release the specified bitmap object.
    /// It is a fatal error trying to release a bitmap object that is still locked.
    ///
    /// \\param  bitmap_object The bitmap object.
    ///
    /// \\returns An error-code indicating success or failure of the operation.
    pub fn release(&self) -> Result<()> {
        LibCZIApiError::try_from(unsafe { libCZI_ReleaseBitmap(**self) })?;
        Ok(())
    }
}

impl TryFrom<&SubBlock> for Bitmap {
    type Error = Error;

    fn try_from(sub_block: &SubBlock) -> Result<Self> {
        sub_block.create_bitmap()
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        self.release().ok();
    }
}

/// Locked version of bitmap so that the data can be accessed
pub struct LockedBitmap {
    bitmap: Bitmap,
    pub lock_info: BitmapLockInfo,
}

impl Deref for LockedBitmap {
    type Target = Bitmap;

    fn deref(&self) -> &Self::Target {
        &self.bitmap
    }
}

impl Drop for LockedBitmap {
    fn drop(&mut self) {
        unsafe { libCZI_BitmapUnlock(self.handle()) };
    }
}

impl LockedBitmap {
    /// Unlock the bitmap object. Once the bitmap is unlocked, the pixel data must not be accessed anymore.
    ///
    /// \\param  bitmap_object The bitmap object.
    ///
    /// \\returns An error-code indicating success or failure of the operation.
    pub fn unlock(self) -> Result<Bitmap> {
        LibCZIApiError::try_from(unsafe { libCZI_BitmapUnlock(**self) })?;
        Ok(self.bitmap.clone())
    }

    /// Copy the pixel data from the specified bitmap object to the specified memory buffer. The specified
    /// destination bitmap must have same width, height and pixel type as the source bitmap.
    ///
    /// \\param          bitmap_object The bitmap object.
    /// \\param          width         The width of the destination bitmap.
    /// \\param          height        The height of the destination bitmap.
    /// \\param          pixel_type    The pixel type.
    /// \\param          stride        The stride (given in bytes).
    /// \\param \[out\]    ptr           Pointer to the memory location where the bitmap is to be copied to.
    ///
    /// \\returns A LibCZIApiErrorCode.
    pub fn copy(
        &self,
        width: u32,
        height: u32,
        pixel_type: PixelType,
        stride: u32,
    ) -> Result<Bitmap> {
        let mut data = MaybeUninit::<Self>::uninit();
        LibCZIApiError::try_from(unsafe {
            libCZI_BitmapCopyTo(
                ***self,
                width,
                height,
                pixel_type.into(),
                stride,
                data.as_mut_ptr() as *mut c_void,
            )
        })?;
        Ok(unsafe { data.assume_init().unlock()? })
    }
}

impl MetadataSegment {
    /// Get the XML-metadata information from the specified metadata-segment object.
    /// Note that the XML-metadata is returned as a pointer to the data (in the 'data' field of the 'MetadataAsXmlInterop' structure), which
    /// must be freed by the caller using 'libCZI_Free'.
    ///
    /// \\param          metadata_segment_object The metadata segment object.
    /// \\param \[out\]    metadata_as_xml_interop If successful, the XML-metadata information is put here.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn get_metadata_as_xml(&self) -> Result<MetadataAsXml> {
        let mut metadata_as_xml_interop = MaybeUninit::uninit();
        let ptr = metadata_as_xml_interop.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_MetadataSegmentGetMetadataAsXml(**self, ptr) })?;
        Ok(unsafe { MetadataAsXml::assume_init(metadata_as_xml_interop) })
    }

    /// Create a CZI-document-information object from the specified metadata-segment object.
    ///
    /// \\param          metadata_segment_object The metadata segment object.
    /// \\param \[in,out\] czi_document_info       If successful, a handle to the newly created CZI-document-info object is put here.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn get_czi_document_info(&self) -> Result<CziDocumentInfo> {
        let mut czi_document = MaybeUninit::uninit();
        let ptr = czi_document.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_MetadataSegmentGetCziDocumentInfo(**self, ptr) })?;
        Ok(unsafe { CziDocumentInfo::assume_init(czi_document) })
    }

    /// Release the specified metadata-segment object.
    ///
    /// \\param  metadata_segment_object The metadata-segment object to be released.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn release(&self) -> Result<()> {
        LibCZIApiError::try_from(unsafe { libCZI_ReleaseMetadataSegment(**self) })?;
        Ok(())
    }
}

impl Drop for MetadataSegment {
    fn drop(&mut self) {
        self.release().ok();
    }
}

impl CziDocumentInfo {
    /// Get \"general document information\" from the specified czi-document information object. The information is returned as a JSON-formatted string.
    /// The JSON returned is an object, with the following possible key-value pairs:
    /// \"name\" : \<name of the document\>, type string
    /// \"title\" : \<title of the document\>, type string
    /// \"user_name\" : \<user name\>, type string
    /// \"description\" : \<description\>, type string
    /// \"comment\" : \<comment\>, type string
    /// \"keywords\" : \<keyword1\>,\<keyword2\>,...\", type string
    /// \"rating\" : \<rating\>, type integer
    /// \"creation_date\" : \<creation date\>, type string, conforming to ISO 8601
    ///
    /// \\param          czi_document_info           The CZI-document-info object.
    /// \\param \[out\]    general_document_info_json  If successful, the general document information is put here. Note that the data must be freed using 'libCZI_Free' by the caller.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn get_general_document_info(&self) -> Result<String> {
        let mut ptr = MaybeUninit::<*mut c_char>::uninit();
        LibCZIApiError::try_from(unsafe {
            libCZI_CziDocumentInfoGetGeneralDocumentInfo(
                **self,
                ptr.as_mut_ptr() as *mut *mut c_void,
            )
        })?;
        let ptr = unsafe { ptr.assume_init() };
        let info = unsafe { CStr::from_ptr(ptr) }.to_str()?.to_owned();
        unsafe { libCZI_Free(ptr as *mut c_void) };
        Ok(info)
    }

    /// Get scaling information from the specified czi-document information object. The information gives the size of an image pixels.
    ///
    /// \\param          czi_document_info           Handle to the CZI-document-info object from which the scaling information will be retrieved.
    /// \\param \[out\]    scaling_info_interop        If successful, the scaling information is put here.
    ///
    /// \\returns        An error-code indicating success or failure of the operation.
    pub fn get_scaling_info(&self) -> Result<ScalingInfo> {
        let mut scaling_info_interop = MaybeUninit::uninit();
        let ptr = scaling_info_interop.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_CziDocumentInfoGetScalingInfo(**self, ptr) })?;
        Ok(unsafe { ScalingInfo::assume_init(scaling_info_interop) })
    }

    /// Get the display-settings from the document's XML-metadata. The display-settings are returned in the form of an object,
    /// for which a handle is returned.
    ///
    /// \\param          czi_document_info       The CZI-document-info object.
    /// \\param \[in,out\] display_settings_handle If successful, a handle to the display-settings object is put here.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn get_display_settings(&self) -> Result<DisplaySettings> {
        let mut display_settings = MaybeUninit::uninit();
        let ptr = display_settings.as_mut_ptr();
        LibCZIApiError::try_from(unsafe { libCZI_CziDocumentInfoGetDisplaySettings(**self, ptr) })?;
        Ok(unsafe { DisplaySettings::assume_init(display_settings) })
    }

    /// Get the dimension information from the document's XML-metadata. The information is returned as a JSON-formatted string.
    ///
    /// \\param          czi_document_info       Handle to the CZI-document-info object from which the dimension information will be retrieved.
    /// \\param          dimension_index         Index of the dimension.
    /// \\param \[out\]    dimension_info_json     If successful, the information is put here as JSON format. Note that the data must be freed using 'libCZI_Free' by the caller.
    ///
    /// \\returns        An error-code indicating success or failure of the operation.
    pub fn get_dimension_info(&self, dimension_index: u32) -> Result<String> {
        let mut ptr = MaybeUninit::<*mut c_char>::uninit();
        LibCZIApiError::try_from(unsafe {
            libCZI_CziDocumentInfoGetDimensionInfo(
                **self,
                dimension_index,
                ptr.as_mut_ptr() as *mut *mut c_void,
            )
        })?;
        let ptr = unsafe { ptr.assume_init() };
        let info = unsafe { CStr::from_ptr(ptr) }.to_str()?.to_owned();
        unsafe { libCZI_Free(ptr as *mut c_void) };
        Ok(info)
    }

    /// Release the specified CZI-document-info object.
    ///
    /// \\param  czi_document_info The CZI-document-info object.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn release(&self) -> Result<()> {
        LibCZIApiError::try_from(unsafe { libCZI_ReleaseCziDocumentInfo(**self) })?;
        Ok(())
    }
}

impl Drop for CziDocumentInfo {
    fn drop(&mut self) {
        self.release().ok();
    }
}

impl OutputStream {
    /// Create an output stream object for a file identified by its filename, which is given as a wide string. Note that wchar_t on
    /// Windows is 16-bit wide, and on Unix-like systems it is 32-bit wide.
    ///
    /// \\param          filename                Filename of the file which is to be opened (zero terminated wide string). Note that on Windows, this
    ///                                         is a string with 16-bit code units, and on Unix-like systems it is typically a string with 32-bit code units.
    /// \\param          overwrite               Indicates whether the file should be overwritten.
    /// \\param \[out\]    output_stream_object    The output stream object that will hold the created stream.
    ///
    /// \\return         An error-code that indicates whether the operation is successful or not. Non-positive values indicates successful, positive values
    ///                 indicates unsuccessful operation.
    pub fn create_for_file_wide(file_name: Vec<u32>, overwrite: bool) -> Result<Self> {
        let mut output_stream = MaybeUninit::uninit();
        let ptr = output_stream.as_mut_ptr();
        LibCZIApiError::try_from(unsafe {
            libCZI_CreateOutputStreamForFileWide(file_name.as_ptr(), overwrite, ptr)
        })?;
        Ok(unsafe { Self::assume_init(output_stream) })
    }

    /// Create an input stream object for a file identified by its filename, which is given as an UTF8 - encoded string.
    ///
    /// \\param          filename                Filename of the file which is to be opened (in UTF8 encoding).
    /// \\param          overwrite               Indicates whether the file should be overwritten.
    /// \\param \[out\]    output_stream_object    The output stream object that will hold the created stream.
    ///
    /// \\return         An error-code that indicates whether the operation is successful or not. Non-positive values indicates successful, positive values
    ///                 indicates unsuccessful operation.
    pub fn create_for_file_utf8<S: AsRef<str>>(file_name: S, overwrite: bool) -> Result<Self> {
        let mut output_stream = MaybeUninit::uninit();
        let ptr = output_stream.as_mut_ptr();
        let file_name = ManuallyDrop::new(CString::new(file_name.as_ref())?);
        LibCZIApiError::try_from(unsafe {
            libCZI_CreateOutputStreamForFileUTF8(file_name.as_ptr(), overwrite, ptr)
        })?;
        Ok(unsafe { Self::assume_init(output_stream) })
    }

    /// Release the specified output stream object. After this function is called, the handle is no
    /// longer valid. Note that calling this function will only decrement the usage count of the
    /// underlying object; whereas the object itself (and the resources it holds) will only be
    /// released when the usage count reaches zero.
    ///
    /// \\param  output_stream_object   The output stream object.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn release(&self) -> Result<()> {
        LibCZIApiError::try_from(unsafe { libCZI_ReleaseOutputStream(**self) })?;
        Ok(())
    }

    /// Create an output stream object which is using externally provided functions for operation
    /// and writing the data. Please refer to the documentation of
    /// 'ExternalOutputStreamStructInterop' for more information.
    ///
    /// \\param          external_output_stream_struct    Structure containing the information about the externally provided functions.
    /// \\param \[out\]    output_stream_object             If successful, the handle to the newly created output stream object is put here.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn create_from_external(external_input_stream: ExternalOutputStreamStruct) -> Result<Self> {
        let mut stream = MaybeUninit::uninit();
        let ptr = stream.as_mut_ptr();
        LibCZIApiError::try_from(unsafe {
            libCZI_CreateOutputStreamFromExternal(external_input_stream.as_ptr(), ptr)
        })?;
        Ok(unsafe { Self::assume_init(stream) })
    }
}

impl Drop for OutputStream {
    fn drop(&mut self) {
        self.release().ok();
    }
}

impl CziWriter {
    /// Create a writer object for authoring a document in CZI-format. The options string is a JSON-formatted string, here
    /// is an example:
    /// \\code
    /// {
    /// \"allow_duplicate_subblocks\" : true
    /// }
    /// \\endcode
    ///
    /// \\param \[out\] writer_object If the operation is successful, a handle to the newly created writer object is put here.
    /// \\param       options       A JSON-formatted zero-terminated string (in UTF8-encoding) containing options for the writer creation.
    ///
    /// \\returns An error-code indicating success or failure of the operation.
    pub fn create<S: AsRef<str>>(options: S) -> Result<Self> {
        let mut writer = MaybeUninit::uninit();
        let ptr = writer.as_mut_ptr();
        let options = ManuallyDrop::new(CString::new(options.as_ref())?);
        LibCZIApiError::try_from(unsafe { libCZI_CreateWriter(ptr, options.as_ptr()) })?;
        Ok(unsafe { Self::assume_init(writer) })
    }

    /// Initializes the writer object with the specified output stream object. The options string is a JSON-formatted string, here
    /// is an example:
    /// \\code
    /// {
    /// \"file_guid\" : \"123e4567-e89b-12d3-a456-426614174000\",
    /// \"reserved_size_attachments_directory\" : 4096,
    /// \"reserved_size_metadata_segment\" : 50000,
    /// \"minimum_m_index\" : 0,
    /// \"maximum_m_index\" : 100
    /// }
    /// \\endcode
    ///
    /// \\param \[out\] writer_object If the operation is successful, a handle to the newly created writer object is put here.
    /// \\param       output_stream_object The output stream object to be used for writing the CZI data.
    /// \\param       parameters       A JSON-formatted zero-terminated string (in UTF8-encoding) containing options for the writer initialization.
    ///
    /// \\returns An error-code indicating success or failure of the operation.
    pub fn init<S: AsRef<str>>(&self, output_stream: &OutputStream, parameters: S) -> Result<()> {
        let parameters = ManuallyDrop::new(CString::new(parameters.as_ref())?);
        LibCZIApiError::try_from(unsafe {
            libCZI_WriterCreate(**self, **output_stream, parameters.as_ptr())
        })?;
        Ok(())
    }

    /// Add the specified sub-block to the writer object. The sub-block information is provided in the 'add_sub_block_info_interop' structure.
    ///
    /// \\param  writer_object               The writer object.
    /// \\param  add_sub_block_info_interop  Information describing the sub-block to be added.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn add_sub_block(&self, add_sub_block_info: AddSubBlockInfo) -> Result<()> {
        LibCZIApiError::try_from(unsafe {
            libCZI_WriterAddSubBlock(**self, add_sub_block_info.as_ptr())
        })?;
        Ok(())
    }

    /// Add the specified attachment to the writer object. The attachment is provided in the 'add_attachment_info_interop' structure.
    ///
    /// \\param  writer_object               The writer object.
    /// \\param  add_attachment_info_interop Information describing the attachment to be added.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn add_attachement(&self, add_attachment_info: AddAttachmentInfo) -> Result<()> {
        LibCZIApiError::try_from(unsafe {
            libCZI_WriterAddAttachment(**self, add_attachment_info.as_ptr())
        })?;
        Ok(())
    }

    /// Add the specified metadata to the writer object. The metadata is provided in the 'write_metadata_info_interop' structure.
    ///
    /// \\param  writer_object               Handle to the writer object to which the metadata will be added.
    /// \\param  write_metadata_info_interop Information describing the metadata to be added.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn write_metadata(&self, write_metadata_info: WriteMetadataInfo) -> Result<()> {
        LibCZIApiError::try_from(unsafe {
            libCZI_WriterWriteMetadata(**self, write_metadata_info.as_ptr())
        })?;
        Ok(())
    }

    /// inalizes the CZI (i.e. writes out the final directory-segments) and closes the file.
    /// Note that this method must be called explicitly in order to get a valid CZI - calling 'libCZI_ReleaseWriter' without
    /// a prior call to this method will close the file immediately without finalization.
    ///
    /// \\param  writer_object   Handle to the writer object that is to be closed.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn close(&self) -> Result<()> {
        LibCZIApiError::try_from(unsafe { libCZI_WriterClose(**self) })?;
        Ok(())
    }

    /// Release the specified writer object.
    ///
    /// \\param  writer_object Handle to the writer object that is to be released.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn release(&self) -> Result<()> {
        LibCZIApiError::try_from(unsafe { libCZI_ReleaseWriter(**self) })?;
        Ok(())
    }
}

impl Drop for CziWriter {
    fn drop(&mut self) {
        self.close().ok();
        self.release().ok();
    }
}

impl SingleChannelScalingTileAccessor {
    /// Gets the size information of the specified tile accessor based on the region of interest and zoom factor.
    ///
    /// \\param  accessor_object     Handle to the tile accessor object for which the size is to be calculated. This object is responsible for managing the access to the tiles within the specified plane.
    /// \\param  roi                 The region of interest that defines the region of interest within the plane for which the size is to be calculated.
    /// \\param  zoom                A floating-point value representing the zoom factor.
    /// \\param  size \[out\]          The size of the tile accessor. It contains width and height information.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn calc_size(&self, roi: IntRect, zoom: f32) -> Result<IntSize> {
        let mut size = MaybeUninit::uninit();
        let ptr = size.as_mut_ptr();
        LibCZIApiError::try_from(unsafe {
            libCZI_SingleChannelTileAccessorCalcSize(**self, roi.as_ptr(), zoom, ptr)
        })?;
        Ok(unsafe { IntSize::assume_init(size) })
    }

    /// Gets the tile bitmap of the specified plane and the specified roi with the specified zoom factor.
    ///
    /// \\param  accessor_object         Handle to the tile accessor object. This object is responsible for managing the access to the tiles within the specified plane.
    /// \\param  coordinate              Pointer to a `CoordinateInterop` structure that specifies the coordinates within the plane from which the tile bitmap is to be retrieved.
    /// \\param  roi                     The region of interest that defines within the plane for which the tile bitmap is requested.
    /// \\param  zoom                    A floating-point value representing the zoom factor.
    /// \\param  options                 A pointer to an AccessorOptionsInterop structure that may contain additional options for accessing the tile bitmap.
    /// \\param  bitmap_object \[out\]     If the operation is successful, the created bitmap object will be put here.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn get(
        &self,
        coordinate: Coordinate,
        roi: IntRect,
        zoom: f32,
        options: AccessorOptions,
    ) -> Result<Bitmap> {
        let mut bitmap = MaybeUninit::uninit();
        let ptr = bitmap.as_mut_ptr();
        LibCZIApiError::try_from(unsafe {
            libCZI_SingleChannelTileAccessorGet(
                **self,
                coordinate.as_ptr(),
                roi.as_ptr(),
                zoom,
                options.as_ptr(),
                ptr,
            )
        })?;
        Ok(unsafe { Bitmap::assume_init(bitmap) })
    }

    /// Release the specified accessor object.
    ///
    /// \\param  accessor_object      The accessor object.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn release(&self) -> Result<()> {
        LibCZIApiError::try_from(unsafe { libCZI_ReleaseCreateSingleChannelTileAccessor(**self) })?;
        Ok(())
    }
}

impl Drop for SingleChannelScalingTileAccessor {
    fn drop(&mut self) {
        self.release().ok();
    }
}

impl DisplaySettings {
    /// Given a display-settings object and the channel-number, this function fills out the
    /// composition-channel-information which is needed for the multi-channel-composition.
    /// Note that in the returned 'CompositionChannelInfoInterop' structure, the 'lut' field is a pointer to the LUT-data,
    /// which must be freed with 'libCZI_Free' by the caller.
    ///
    /// \\param          display_settings_handle             The display settings handle.
    /// \\param          channel_index                       The channel-index (referring to the display settings object) we are concerned with.
    /// \\param          sixteen_or_eight_bits_lut           True for generating a 16-bit LUT; if false, then an 8-bit LUT is generated.
    /// \\param \[out\]    composition_channel_info_interop    The composition channel information is put here.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn compositor_fill_out_composition_channel_info_interop(
        &self,
        channel_index: i32,
        sixteen_or_eight_bits_lut: bool,
    ) -> Result<CompositionChannelInfo> {
        let mut composition_channel_info = MaybeUninit::uninit();
        let ptr = composition_channel_info.as_mut_ptr();
        LibCZIApiError::try_from(unsafe {
            libCZI_CompositorFillOutCompositionChannelInfoInterop(
                **self,
                channel_index,
                sixteen_or_eight_bits_lut,
                ptr,
            )
        })?;
        Ok(unsafe { CompositionChannelInfo::assume_init(composition_channel_info) })
    }

    pub fn get_channel_display_settings(&self, channel_id: i32) -> Result<ChannelDisplaySettings> {
        let mut channel_display_setting = MaybeUninit::uninit();
        let ptr = channel_display_setting.as_mut_ptr();
        LibCZIApiError::try_from(unsafe {
            libCZI_DisplaySettingsGetChannelDisplaySettings(**self, channel_id, ptr)
        })?;
        Ok(unsafe { ChannelDisplaySettings::assume_init(channel_display_setting) })
    }

    /// Release the specified display settings object.
    ///
    /// \\param  display_settings_handle      The display settings object.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn release(&self) -> Result<()> {
        LibCZIApiError::try_from(unsafe { libCZI_ReleaseDisplaySettings(**self) })?;
        Ok(())
    }
}

impl Drop for DisplaySettings {
    fn drop(&mut self) {
        self.release().ok();
    }
}

/// Perform a multi-channel-composition operation. The source bitmaps are provided in the 'source_bitmaps' array, and the
/// array of 'CompositionChannelInfoInterop' structures provide the information needed for the composition. The resulting bitmap
/// is then put into the 'bitmap_object' handle.
///
/// \\param       channelCount       The number of channels - this defines the size of the 'source_bitmaps' and 'channel_info' arrays.
/// \\param       source_bitmaps     The array of source bitmaps.
/// \\param       channel_info       The array of channel information.
/// \\param \[out\] bitmap_object      The resulting bitmap is put here.
///
/// \\return     An error-code indicating success or failure of the operation.
pub fn compositor_do_multi_channel_composition(
    channel_count: i32,
    source_bitmaps: Vec<Bitmap>,
    channel_info: CompositionChannelInfo,
) -> Result<Bitmap> {
    let mut bitmap = MaybeUninit::uninit();
    let ptr = bitmap.as_mut_ptr();
    LibCZIApiError::try_from(unsafe {
        libCZI_CompositorDoMultiChannelComposition(
            channel_count,
            source_bitmaps.as_ptr() as *const BitmapObjectHandle,
            channel_info.as_ptr(),
            ptr,
        )
    })?;
    Ok(unsafe { Bitmap::assume_init(bitmap) })
}

impl ChannelDisplaySettings {
    /// Release the specified channel-display settings object.
    ///
    /// \\param  channel_display_settings_handle      The channel-display settings object.
    ///
    /// \\returns    An error-code indicating success or failure of the operation.
    pub fn release(&self) -> Result<()> {
        LibCZIApiError::try_from(unsafe { libCZI_ReleaseDisplaySettings(**self) })?;
        Ok(())
    }
}

impl Drop for ChannelDisplaySettings {
    fn drop(&mut self) {
        self.release().ok();
    }
}
