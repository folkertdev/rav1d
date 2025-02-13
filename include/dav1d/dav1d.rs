use crate::include::dav1d::picture::Dav1dPicAllocator;
use crate::include::dav1d::picture::Rav1dPicAllocator;
use crate::src::error::Rav1dError;
use crate::src::internal::Rav1dContext;
pub use crate::src::log::Dav1dLogger;
use crate::src::log::Rav1dLogger;
use crate::src::r#ref::Rav1dRef;
use bitflags::bitflags;
use std::ffi::c_int;
use std::ffi::c_uint;

pub type Dav1dContext = Rav1dContext;
pub type Dav1dRef = Rav1dRef;

pub type Dav1dInloopFilterType = c_uint;
pub const DAV1D_INLOOPFILTER_ALL: Dav1dInloopFilterType = 7;
pub const DAV1D_INLOOPFILTER_RESTORATION: Dav1dInloopFilterType = 4;
pub const DAV1D_INLOOPFILTER_CDEF: Dav1dInloopFilterType = 2;
pub const DAV1D_INLOOPFILTER_DEBLOCK: Dav1dInloopFilterType = 1;
pub const DAV1D_INLOOPFILTER_NONE: Dav1dInloopFilterType = 0;

pub(crate) type Rav1dInloopFilterType = c_uint;
pub(crate) const RAV1D_INLOOPFILTER_ALL: Rav1dInloopFilterType = DAV1D_INLOOPFILTER_ALL;
pub(crate) const RAV1D_INLOOPFILTER_RESTORATION: Rav1dInloopFilterType =
    DAV1D_INLOOPFILTER_RESTORATION;
pub(crate) const RAV1D_INLOOPFILTER_CDEF: Rav1dInloopFilterType = DAV1D_INLOOPFILTER_CDEF;
pub(crate) const RAV1D_INLOOPFILTER_DEBLOCK: Rav1dInloopFilterType = DAV1D_INLOOPFILTER_DEBLOCK;
pub(crate) const _RAV1D_INLOOPFILTER_NONE: Rav1dInloopFilterType = DAV1D_INLOOPFILTER_NONE;

pub type Dav1dDecodeFrameType = c_uint;
pub const DAV1D_DECODEFRAMETYPE_KEY: Dav1dDecodeFrameType = 3;
pub const DAV1D_DECODEFRAMETYPE_INTRA: Dav1dDecodeFrameType = 2;
pub const DAV1D_DECODEFRAMETYPE_REFERENCE: Dav1dDecodeFrameType = 1;
pub const DAV1D_DECODEFRAMETYPE_ALL: Dav1dDecodeFrameType = 0;

pub(crate) type Rav1dDecodeFrameType = c_uint;
pub(crate) const RAV1D_DECODEFRAMETYPE_KEY: Rav1dDecodeFrameType = DAV1D_DECODEFRAMETYPE_KEY;
pub(crate) const RAV1D_DECODEFRAMETYPE_INTRA: Rav1dDecodeFrameType = DAV1D_DECODEFRAMETYPE_INTRA;
pub(crate) const RAV1D_DECODEFRAMETYPE_REFERENCE: Rav1dDecodeFrameType =
    DAV1D_DECODEFRAMETYPE_REFERENCE;
pub(crate) const RAV1D_DECODEFRAMETYPE_ALL: Rav1dDecodeFrameType = DAV1D_DECODEFRAMETYPE_ALL;

pub type Dav1dEventFlags = c_uint;
pub const DAV1D_EVENT_FLAG_NEW_SEQUENCE: Dav1dEventFlags =
    Rav1dEventFlags::NEW_SEQUENCE.bits() as Dav1dEventFlags;
pub const DAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO: Dav1dEventFlags =
    Rav1dEventFlags::NEW_OP_PARAMS_INFO.bits() as Dav1dEventFlags;

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub(crate) struct Rav1dEventFlags: u8 {
        /// The last returned picture contains a reference
        /// to a new [`Rav1dSequenceHeader`],
        /// either because it's the start of a new coded sequence,
        /// or the decoder was flushed before it was generated.
        ///
        /// [`Rav1dSequenceHeader`]: crate::include::dav1d::headers::Rav1dSequenceHeader
        const NEW_SEQUENCE = 1 << 0;

        /// The last returned picture contains a reference to a
        /// [`Rav1dSequenceHeader`] with new [`Rav1dSequenceHeaderOperatingParameterInfo`]
        /// for the current coded sequence.
        ///
        /// [`Rav1dSequenceHeader`]: crate::include::dav1d::headers::Rav1dSequenceHeader
        /// [`Rav1dSequenceHeaderOperatingParameterInfo`]: crate::include::dav1d::headers::Rav1dSequenceHeaderOperatingParameterInfo
        const NEW_OP_PARAMS_INFO = 1 << 1;
    }
}

impl From<Rav1dEventFlags> for Dav1dEventFlags {
    fn from(value: Rav1dEventFlags) -> Self {
        value.bits().into()
    }
}

impl From<Dav1dEventFlags> for Rav1dEventFlags {
    fn from(value: Dav1dEventFlags) -> Self {
        Self::from_bits_retain(value as u8)
    }
}

#[repr(C)]
pub struct Dav1dSettings {
    pub n_threads: c_int,
    pub max_frame_delay: c_int,
    pub apply_grain: c_int,
    pub operating_point: c_int,
    pub all_layers: c_int,
    pub frame_size_limit: c_uint,
    pub allocator: Dav1dPicAllocator,
    pub logger: Dav1dLogger,
    pub strict_std_compliance: c_int,
    pub output_invisible_frames: c_int,
    pub inloop_filters: Dav1dInloopFilterType,
    pub decode_frame_type: Dav1dDecodeFrameType,
    pub reserved: [u8; 16],
}

#[repr(C)]
pub(crate) struct Rav1dSettings {
    pub n_threads: c_int,
    pub max_frame_delay: c_int,
    pub apply_grain: bool,
    pub operating_point: c_int,
    pub all_layers: bool,
    pub frame_size_limit: c_uint,
    pub allocator: Rav1dPicAllocator,
    pub logger: Option<Rav1dLogger>,
    pub strict_std_compliance: bool,
    pub output_invisible_frames: bool,
    pub inloop_filters: Rav1dInloopFilterType,
    pub decode_frame_type: Rav1dDecodeFrameType,
}

impl TryFrom<Dav1dSettings> for Rav1dSettings {
    type Error = Rav1dError;

    fn try_from(value: Dav1dSettings) -> Result<Self, Self::Error> {
        let Dav1dSettings {
            n_threads,
            max_frame_delay,
            apply_grain,
            operating_point,
            all_layers,
            frame_size_limit,
            allocator,
            logger,
            strict_std_compliance,
            output_invisible_frames,
            inloop_filters,
            decode_frame_type,
            reserved: _,
        } = value;
        Ok(Self {
            n_threads,
            max_frame_delay,
            apply_grain: apply_grain != 0,
            operating_point,
            all_layers: all_layers != 0,
            frame_size_limit,
            allocator: allocator.try_into()?,
            logger: logger.into(),
            strict_std_compliance: strict_std_compliance != 0,
            output_invisible_frames: output_invisible_frames != 0,
            inloop_filters,
            decode_frame_type,
        })
    }
}

impl From<Rav1dSettings> for Dav1dSettings {
    fn from(value: Rav1dSettings) -> Self {
        let Rav1dSettings {
            n_threads,
            max_frame_delay,
            apply_grain,
            operating_point,
            all_layers,
            frame_size_limit,
            allocator,
            logger,
            strict_std_compliance,
            output_invisible_frames,
            inloop_filters,
            decode_frame_type,
        } = value;
        Self {
            n_threads,
            max_frame_delay,
            apply_grain: apply_grain as c_int,
            operating_point,
            all_layers: all_layers as c_int,
            frame_size_limit,
            allocator: allocator.into(),
            logger: logger.into(),
            strict_std_compliance: strict_std_compliance as c_int,
            output_invisible_frames: output_invisible_frames as c_int,
            inloop_filters,
            decode_frame_type,
            reserved: Default::default(),
        }
    }
}
