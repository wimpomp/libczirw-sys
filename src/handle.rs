use crate::misc::Ptr;
use crate::sys::*;
use std::mem::MaybeUninit;
use std::ops::Deref;

/// CZI-reader object.
#[derive(Clone, Debug)]
pub struct CziReader(pub(crate) CziReaderObjectHandle);

/// sub-block object.
#[derive(Clone, Debug)]
pub struct SubBlock(pub(crate) SubBlockObjectHandle);

/// input stream object.
#[derive(Clone, Debug)]
pub struct InputStream(pub(crate) InputStreamObjectHandle);

/// output stream object.
#[derive(Clone, Debug)]
pub struct OutputStream(pub(crate) OutputStreamObjectHandle);

/// memory allocation object - which is a pointer to a memory block, which must be
/// freed with 'libCZI_Free'.
/// TODO(JBL): this is not really used so far, should be removed I guess.
#[derive(Clone, Debug)]
pub struct MemoryAllocation(pub(crate) MemoryAllocationObjectHandle);

/// bitmap object.
#[derive(Clone, Debug)]
pub struct Bitmap(pub(crate) BitmapObjectHandle);

/// metadata segment object.
#[derive(Clone, Debug)]
pub struct MetadataSegment(pub(crate) MetadataSegmentObjectHandle);

/// attachment object.
#[derive(Clone, Debug)]
pub struct Attachment(pub(crate) AttachmentObjectHandle);

/// writer object.
#[derive(Clone, Debug)]
pub struct CziWriter(pub(crate) CziWriterObjectHandle);

/// single-channel-scaling-tile-accessor.
#[derive(Clone, Debug)]
pub struct SingleChannelScalingTileAccessor(
    pub(crate) SingleChannelScalingTileAccessorObjectHandle,
);

/// document info object.
#[derive(Clone, Debug)]
pub struct CziDocumentInfo(pub(crate) CziDocumentInfoHandle);

/// display settings object.
#[derive(Clone, Debug)]
pub struct DisplaySettings(pub(crate) DisplaySettingsHandle);

/// channel display settings object.
#[derive(Clone, Debug)]
pub struct ChannelDisplaySettings(pub(crate) ChannelDisplaySettingsHandle);

macro_rules! impl_struct {
  ($($n:ident: $t:ty: $s:ty $(,)?)*) => {
    $(
      impl $t {
        #[allow(dead_code)]
        pub (crate) fn handle(&self) -> ObjectHandle { self.0 }
      }

      impl Ptr for $t {
        type Pointer = $s;

        unsafe fn assume_init(ptr: MaybeUninit<Self::Pointer>) -> Self {
          Self(unsafe { ptr.assume_init() })
        }

        fn as_mut_ptr(&self) -> *mut Self::Pointer {
          &self.0 as *const _ as *mut _
        }

        fn as_ptr(&self) -> *const Self::Pointer {
          &self.0 as *const _ as *const _
        }
      }

      impl Deref for $t {
        type Target = ObjectHandle;

        fn deref(&self) -> &Self::Target {
          &self.0
        }
      }
    )*
  };
}

impl_struct! {
  CziReader: CziReader: CziReaderObjectHandle,
  SubBlock: SubBlock: SubBlockObjectHandle,
  InputStream: InputStream: InputStreamObjectHandle,
  OutputStream: OutputStream: OutputStreamObjectHandle,
  MemoryAllocation: MemoryAllocation: MemoryAllocationObjectHandle,
  Bitmap: Bitmap: BitmapObjectHandle,
  MetadataSegment: MetadataSegment: MetadataSegmentObjectHandle,
  Attachment: Attachment: AttachmentObjectHandle,
  CziWriter: CziWriter: CziWriterObjectHandle,
  SingleChannelScalingTileAccessor: SingleChannelScalingTileAccessor: SingleChannelScalingTileAccessorObjectHandle,
  CziDocumentInfo: CziDocumentInfo: CziDocumentInfoHandle,
  DisplaySettings: DisplaySettings: DisplaySettingsHandle,
  ChannelDisplaySettings: ChannelDisplaySettings: ChannelDisplaySettingsHandle,
}
