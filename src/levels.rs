use std::ffi::c_uint;
use std::ops::Neg;

pub type ObuMetaType = c_uint;
pub const OBU_META_TIMECODE: ObuMetaType = 5;
pub const OBU_META_ITUT_T35: ObuMetaType = 4;
pub const OBU_META_SCALABILITY: ObuMetaType = 3;
pub const OBU_META_HDR_MDCV: ObuMetaType = 2;
pub const OBU_META_HDR_CLL: ObuMetaType = 1;

pub type TxfmSize = u8;
pub const N_TX_SIZES: usize = 5;
pub const TX_64X64: TxfmSize = 4;
pub const TX_32X32: TxfmSize = 3;
pub const TX_16X16: TxfmSize = 2;
pub const TX_8X8: TxfmSize = 1;
pub const TX_4X4: TxfmSize = 0;

pub type BlockLevel = u8;
pub const N_BL_LEVELS: usize = 5;
pub const BL_8X8: BlockLevel = 4;
pub const BL_16X16: BlockLevel = 3;
pub const BL_32X32: BlockLevel = 2;
pub const BL_64X64: BlockLevel = 1;
pub const BL_128X128: BlockLevel = 0;

pub type RectTxfmSize = u8;
pub const N_RECT_TX_SIZES: usize = 19; // TODO(kkysen) symbolicate in Dav1dFrameContext::qm once deduplicated
pub const RTX_64X16: RectTxfmSize = 18;
pub const RTX_16X64: RectTxfmSize = 17;
pub const RTX_32X8: RectTxfmSize = 16;
pub const RTX_8X32: RectTxfmSize = 15;
pub const RTX_16X4: RectTxfmSize = 14;
pub const RTX_4X16: RectTxfmSize = 13;
pub const RTX_64X32: RectTxfmSize = 12;
pub const RTX_32X64: RectTxfmSize = 11;
pub const RTX_32X16: RectTxfmSize = 10;
pub const RTX_16X32: RectTxfmSize = 9;
pub const RTX_16X8: RectTxfmSize = 8;
pub const RTX_8X16: RectTxfmSize = 7;
pub const RTX_8X4: RectTxfmSize = 6;
pub const RTX_4X8: RectTxfmSize = 5;

pub type TxfmType = u8;
pub const N_TX_TYPES_PLUS_LL: usize = 17;
pub const WHT_WHT: TxfmType = 16;
pub const _N_TX_TYPES: usize = 16;
pub const H_FLIPADST: TxfmType = 15;
pub const V_FLIPADST: TxfmType = 14;
pub const H_ADST: TxfmType = 13;
pub const V_ADST: TxfmType = 12;
pub const H_DCT: TxfmType = 11;
pub const V_DCT: TxfmType = 10;
pub const IDTX: TxfmType = 9;
pub const FLIPADST_ADST: TxfmType = 8;
pub const ADST_FLIPADST: TxfmType = 7;
pub const FLIPADST_FLIPADST: TxfmType = 6;
pub const DCT_FLIPADST: TxfmType = 5;
pub const FLIPADST_DCT: TxfmType = 4;
pub const ADST_ADST: TxfmType = 3;
pub const DCT_ADST: TxfmType = 2;
pub const ADST_DCT: TxfmType = 1;
pub const DCT_DCT: TxfmType = 0;

pub type TxClass = c_uint;
pub const TX_CLASS_V: TxClass = 2;
pub const TX_CLASS_H: TxClass = 1;
pub const TX_CLASS_2D: TxClass = 0;

pub type IntraPredMode = u8;
pub const FILTER_PRED: IntraPredMode = 13;
pub const Z3_PRED: IntraPredMode = 8;
pub const Z2_PRED: IntraPredMode = 7;
pub const Z1_PRED: IntraPredMode = 6;
pub const DC_128_PRED: IntraPredMode = 5;
pub const TOP_DC_PRED: IntraPredMode = 4;
pub const LEFT_DC_PRED: IntraPredMode = 3;
pub const N_IMPL_INTRA_PRED_MODES: usize = 14; // TODO(kkysen) symbolicate in struct Rav1dIntraPredDSPContext::intra_pred once deduplicated
pub const N_UV_INTRA_PRED_MODES: usize = 14;
pub const CFL_PRED: IntraPredMode = 13;
pub const N_INTRA_PRED_MODES: usize = 13;
pub const PAETH_PRED: IntraPredMode = 12;
pub const SMOOTH_H_PRED: IntraPredMode = 11;
pub const SMOOTH_V_PRED: IntraPredMode = 10;
pub const SMOOTH_PRED: IntraPredMode = 9;
pub const VERT_LEFT_PRED: IntraPredMode = 8;
pub const HOR_UP_PRED: IntraPredMode = 7;
pub const HOR_DOWN_PRED: IntraPredMode = 6;
pub const VERT_RIGHT_PRED: IntraPredMode = 5;
pub const DIAG_DOWN_RIGHT_PRED: IntraPredMode = 4;
pub const DIAG_DOWN_LEFT_PRED: IntraPredMode = 3;
pub const HOR_PRED: IntraPredMode = 2;
pub const VERT_PRED: IntraPredMode = 1;
pub const DC_PRED: IntraPredMode = 0;

pub type InterIntraPredMode = c_uint;
pub const N_INTER_INTRA_PRED_MODES: usize = 4;
pub const II_SMOOTH_PRED: InterIntraPredMode = 3;
pub const II_HOR_PRED: InterIntraPredMode = 2;
pub const II_VERT_PRED: InterIntraPredMode = 1;
pub const II_DC_PRED: InterIntraPredMode = 0;

pub type BlockPartition = u8;
pub const N_SUB8X8_PARTITIONS: usize = 4;
pub const N_PARTITIONS: usize = 10;
pub const PARTITION_V4: BlockPartition = 9;
pub const PARTITION_H4: BlockPartition = 8;
pub const PARTITION_T_RIGHT_SPLIT: BlockPartition = 7;
pub const PARTITION_T_LEFT_SPLIT: BlockPartition = 6;
pub const PARTITION_T_BOTTOM_SPLIT: BlockPartition = 5;
pub const PARTITION_T_TOP_SPLIT: BlockPartition = 4;
pub const PARTITION_SPLIT: BlockPartition = 3;
pub const PARTITION_V: BlockPartition = 2;
pub const PARTITION_H: BlockPartition = 1;
pub const PARTITION_NONE: BlockPartition = 0;

pub type BlockSize = u8;
pub const N_BS_SIZES: usize = 22;
pub const BS_4x4: BlockSize = 21;
pub const BS_4x8: BlockSize = 20;
pub const BS_4x16: BlockSize = 19;
pub const BS_8x4: BlockSize = 18;
pub const BS_8x8: BlockSize = 17;
pub const BS_8x16: BlockSize = 16;
pub const BS_8x32: BlockSize = 15;
pub const BS_16x4: BlockSize = 14;
pub const BS_16x8: BlockSize = 13;
pub const BS_16x16: BlockSize = 12;
pub const BS_16x32: BlockSize = 11;
pub const BS_16x64: BlockSize = 10;
pub const BS_32x8: BlockSize = 9;
pub const BS_32x16: BlockSize = 8;
pub const BS_32x32: BlockSize = 7;
pub const BS_32x64: BlockSize = 6;
pub const BS_64x16: BlockSize = 5;
pub const BS_64x32: BlockSize = 4;
pub const BS_64x64: BlockSize = 3;
pub const BS_64x128: BlockSize = 2;
pub const BS_128x64: BlockSize = 1;
pub const BS_128x128: BlockSize = 0;

pub type Filter2d = c_uint;
pub const N_2D_FILTERS: usize = 10; // TODO(kkysen) symbolicate in struct Rav1dMCDSPContext once deduplicated
pub const FILTER_2D_BILINEAR: Filter2d = 9;
pub const FILTER_2D_8TAP_SMOOTH_SHARP: Filter2d = 8;
pub const FILTER_2D_8TAP_SMOOTH: Filter2d = 7;
pub const FILTER_2D_8TAP_SMOOTH_REGULAR: Filter2d = 6;
pub const FILTER_2D_8TAP_SHARP: Filter2d = 5;
pub const FILTER_2D_8TAP_SHARP_SMOOTH: Filter2d = 4;
pub const FILTER_2D_8TAP_SHARP_REGULAR: Filter2d = 3;
pub const FILTER_2D_8TAP_REGULAR_SHARP: Filter2d = 2;
pub const FILTER_2D_8TAP_REGULAR_SMOOTH: Filter2d = 1;
pub const FILTER_2D_8TAP_REGULAR: Filter2d = 0;

pub type MVJoint = c_uint;
pub const N_MV_JOINTS: usize = 4;
pub const MV_JOINT_HV: MVJoint = 3;
pub const MV_JOINT_V: MVJoint = 2;
pub const MV_JOINT_H: MVJoint = 1;
pub const _MV_JOINT_ZERO: MVJoint = 0;

pub type InterPredMode = u8;
pub const _N_INTER_PRED_MODES: usize = 4;
pub const NEWMV: InterPredMode = 3;
pub const GLOBALMV: InterPredMode = 2;
pub const NEARMV: InterPredMode = 1;
pub const NEARESTMV: InterPredMode = 0;

pub type DRL_PROXIMITY = u8;
pub const NEARISH_DRL: DRL_PROXIMITY = 3;
pub const NEAR_DRL: DRL_PROXIMITY = 2;
pub const NEARER_DRL: DRL_PROXIMITY = 1;
pub const NEAREST_DRL: DRL_PROXIMITY = 0;

pub type CompInterPredMode = u8;
pub const N_COMP_INTER_PRED_MODES: usize = 8;
pub const NEWMV_NEWMV: CompInterPredMode = 7;
pub const GLOBALMV_GLOBALMV: CompInterPredMode = 6;
pub const NEWMV_NEARMV: CompInterPredMode = 5;
pub const NEARMV_NEWMV: CompInterPredMode = 4;
pub const NEWMV_NEARESTMV: CompInterPredMode = 3;
pub const NEARESTMV_NEWMV: CompInterPredMode = 2;
pub const NEARMV_NEARMV: CompInterPredMode = 1;
pub const NEARESTMV_NEARESTMV: CompInterPredMode = 0;

pub type CompInterType = u8;
pub const COMP_INTER_WEDGE: CompInterType = 4;
pub const COMP_INTER_SEG: CompInterType = 3;
pub const COMP_INTER_AVG: CompInterType = 2;
pub const COMP_INTER_WEIGHTED_AVG: CompInterType = 1;
pub const COMP_INTER_NONE: CompInterType = 0;

pub type InterIntraType = u8;
pub const INTER_INTRA_WEDGE: InterIntraType = 2;
pub const INTER_INTRA_BLEND: InterIntraType = 1;
pub const INTER_INTRA_NONE: InterIntraType = 0;

/// Note that this is legitimately [`Copy`]
/// (unlike other transpiled types that are [`Copy`] due to being from C).
/// This is needed because [`mv`] is used within packed structs like [`refmvs_block`],
/// meaning a reference to [`mv`] cannot always be take,
/// which includes `&self` methods, including autogenerated ones like [`PartialEq::eq`].
///
/// [`refmvs_block`]: crate::src::refmvs::refmvs_block
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct mv {
    pub y: i16,
    pub x: i16,
}

impl mv {
    pub const ZERO: Self = Self { y: 0, x: 0 };

    pub const INVALID: Self = Self {
        y: i16::MIN,
        x: i16::MIN,
    };

    pub fn is_invalid(self) -> bool {
        self == Self::INVALID
    }

    #[allow(dead_code)]
    pub fn is_valid(self) -> bool {
        !self.is_invalid()
    }
}

impl Neg for mv {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            y: -self.y,
            x: -self.x,
        }
    }
}

pub type MotionMode = c_uint;
pub const MM_WARP: MotionMode = 2;
pub const MM_OBMC: MotionMode = 1;
pub const MM_TRANSLATION: MotionMode = 0;

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Av1Block_intra {
    pub y_mode: u8,
    pub uv_mode: u8,
    pub tx: u8,
    pub pal_sz: [u8; 2],
    pub y_angle: i8,
    pub uv_angle: i8,
    pub cfl_alpha: [i8; 2],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Av1Block_inter_1d {
    pub mv: [mv; 2],
    pub wedge_idx: u8,
    pub mask_sign: u8,
    pub interintra_mode: u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Av1Block_inter_2d {
    pub mv2d: mv,
    pub matrix: [i16; 4],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union Av1Block_inter_nd {
    pub c2rust_unnamed: Av1Block_inter_1d,
    pub c2rust_unnamed_0: Av1Block_inter_2d,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Av1Block_inter {
    pub c2rust_unnamed: Av1Block_inter_nd,
    pub comp_type: u8,
    pub inter_mode: u8,
    pub motion_mode: u8,
    pub drl_idx: u8,
    pub r#ref: [i8; 2],
    pub max_ytx: u8,
    pub filter2d: u8,
    pub interintra_type: u8,
    pub tx_split0: u8,
    pub tx_split1: u16,
}

#[repr(C)]
pub union Av1Block_intra_inter {
    pub c2rust_unnamed: Av1Block_intra,
    pub c2rust_unnamed_0: Av1Block_inter,
}

impl Default for Av1Block_intra_inter {
    fn default() -> Self {
        Av1Block_intra_inter {
            c2rust_unnamed: Default::default(),
        }
    }
}

#[derive(Default)]
#[repr(C)]
pub struct Av1Block {
    pub bl: u8,
    pub bs: u8,
    pub bp: u8,
    pub intra: u8,
    pub seg_id: u8,
    pub skip_mode: u8,
    pub skip: u8,
    pub uvtx: u8,
    pub c2rust_unnamed: Av1Block_intra_inter,
}

impl Av1Block {
    pub unsafe fn cfl_alpha(&self) -> &[i8; 2] {
        &self.c2rust_unnamed.c2rust_unnamed.cfl_alpha
    }

    pub unsafe fn cfl_alpha_mut(&mut self) -> &mut [i8; 2] {
        &mut self.c2rust_unnamed.c2rust_unnamed.cfl_alpha
    }

    pub unsafe fn comp_type(&self) -> u8 {
        self.c2rust_unnamed.c2rust_unnamed_0.comp_type
    }

    pub unsafe fn comp_type_mut(&mut self) -> &mut u8 {
        &mut self.c2rust_unnamed.c2rust_unnamed_0.comp_type
    }

    pub unsafe fn drl_idx(&self) -> u8 {
        self.c2rust_unnamed.c2rust_unnamed_0.drl_idx
    }

    pub unsafe fn drl_idx_mut(&mut self) -> &mut u8 {
        &mut self.c2rust_unnamed.c2rust_unnamed_0.drl_idx
    }

    pub unsafe fn inter_mode(&self) -> u8 {
        self.c2rust_unnamed.c2rust_unnamed_0.inter_mode
    }

    pub unsafe fn inter_mode_mut(&mut self) -> &mut u8 {
        &mut self.c2rust_unnamed.c2rust_unnamed_0.inter_mode
    }

    pub unsafe fn mask_sign(&self) -> u8 {
        self.c2rust_unnamed
            .c2rust_unnamed_0
            .c2rust_unnamed
            .c2rust_unnamed
            .mask_sign
    }

    pub unsafe fn mask_sign_mut(&mut self) -> &mut u8 {
        &mut self
            .c2rust_unnamed
            .c2rust_unnamed_0
            .c2rust_unnamed
            .c2rust_unnamed
            .mask_sign
    }

    pub unsafe fn pal_sz(&self) -> &[u8; 2] {
        &self.c2rust_unnamed.c2rust_unnamed.pal_sz
    }

    pub unsafe fn pal_sz_mut(&mut self) -> &mut [u8; 2] {
        &mut self.c2rust_unnamed.c2rust_unnamed.pal_sz
    }

    pub unsafe fn tx(&self) -> u8 {
        self.c2rust_unnamed.c2rust_unnamed.tx
    }

    pub unsafe fn tx_mut(&mut self) -> &mut u8 {
        &mut self.c2rust_unnamed.c2rust_unnamed.tx
    }

    pub unsafe fn y_mode(&self) -> u8 {
        self.c2rust_unnamed.c2rust_unnamed.y_mode
    }

    pub unsafe fn y_mode_mut(&mut self) -> &mut u8 {
        &mut self.c2rust_unnamed.c2rust_unnamed.y_mode
    }

    pub unsafe fn y_angle(&self) -> i8 {
        self.c2rust_unnamed.c2rust_unnamed.y_angle
    }

    pub unsafe fn y_angle_mut(&mut self) -> &mut i8 {
        &mut self.c2rust_unnamed.c2rust_unnamed.y_angle
    }

    #[allow(dead_code)]
    pub unsafe fn uv_angle(&self) -> i8 {
        self.c2rust_unnamed.c2rust_unnamed.uv_angle
    }

    pub unsafe fn uv_angle_mut(&mut self) -> &mut i8 {
        &mut self.c2rust_unnamed.c2rust_unnamed.uv_angle
    }

    pub unsafe fn uv_mode(&self) -> u8 {
        self.c2rust_unnamed.c2rust_unnamed.uv_mode
    }

    pub unsafe fn uv_mode_mut(&mut self) -> &mut u8 {
        &mut self.c2rust_unnamed.c2rust_unnamed.uv_mode
    }

    pub unsafe fn wedge_idx(&self) -> u8 {
        self.c2rust_unnamed
            .c2rust_unnamed_0
            .c2rust_unnamed
            .c2rust_unnamed
            .wedge_idx
    }

    pub unsafe fn wedge_idx_mut(&mut self) -> &mut u8 {
        &mut self
            .c2rust_unnamed
            .c2rust_unnamed_0
            .c2rust_unnamed
            .c2rust_unnamed
            .wedge_idx
    }

    pub unsafe fn matrix(&self) -> &[i16; 4] {
        &self
            .c2rust_unnamed
            .c2rust_unnamed_0
            .c2rust_unnamed
            .c2rust_unnamed_0
            .matrix
    }

    pub unsafe fn matrix_mut(&mut self) -> &mut [i16; 4] {
        &mut self
            .c2rust_unnamed
            .c2rust_unnamed_0
            .c2rust_unnamed
            .c2rust_unnamed_0
            .matrix
    }

    pub unsafe fn mv(&self) -> &[mv; 2] {
        &self
            .c2rust_unnamed
            .c2rust_unnamed_0
            .c2rust_unnamed
            .c2rust_unnamed
            .mv
    }

    pub unsafe fn mv_mut(&mut self) -> &mut [mv; 2] {
        &mut self
            .c2rust_unnamed
            .c2rust_unnamed_0
            .c2rust_unnamed
            .c2rust_unnamed
            .mv
    }

    pub unsafe fn mv2d(&self) -> &mv {
        &self
            .c2rust_unnamed
            .c2rust_unnamed_0
            .c2rust_unnamed
            .c2rust_unnamed_0
            .mv2d
    }

    pub unsafe fn filter2d(&self) -> u8 {
        self.c2rust_unnamed.c2rust_unnamed_0.filter2d
    }

    pub unsafe fn filter2d_mut(&mut self) -> &mut u8 {
        &mut self.c2rust_unnamed.c2rust_unnamed_0.filter2d
    }

    pub unsafe fn r#ref(&self) -> [i8; 2] {
        self.c2rust_unnamed.c2rust_unnamed_0.r#ref
    }

    pub unsafe fn ref_mut(&mut self) -> &mut [i8; 2] {
        &mut self.c2rust_unnamed.c2rust_unnamed_0.r#ref
    }

    pub unsafe fn max_ytx(&self) -> u8 {
        self.c2rust_unnamed.c2rust_unnamed_0.max_ytx
    }

    pub unsafe fn max_ytx_mut(&mut self) -> &mut u8 {
        &mut self.c2rust_unnamed.c2rust_unnamed_0.max_ytx
    }

    pub unsafe fn interintra_type(&self) -> u8 {
        self.c2rust_unnamed.c2rust_unnamed_0.interintra_type
    }

    pub unsafe fn interintra_type_mut(&mut self) -> &mut u8 {
        &mut self.c2rust_unnamed.c2rust_unnamed_0.interintra_type
    }

    pub unsafe fn interintra_mode(&self) -> u8 {
        self.c2rust_unnamed
            .c2rust_unnamed_0
            .c2rust_unnamed
            .c2rust_unnamed
            .interintra_mode
    }

    pub unsafe fn interintra_mode_mut(&mut self) -> &mut u8 {
        &mut self
            .c2rust_unnamed
            .c2rust_unnamed_0
            .c2rust_unnamed
            .c2rust_unnamed
            .interintra_mode
    }

    pub unsafe fn motion_mode(&self) -> u8 {
        self.c2rust_unnamed.c2rust_unnamed_0.motion_mode
    }

    pub unsafe fn motion_mode_mut(&mut self) -> &mut u8 {
        &mut self.c2rust_unnamed.c2rust_unnamed_0.motion_mode
    }

    pub unsafe fn tx_split0(&self) -> u8 {
        self.c2rust_unnamed.c2rust_unnamed_0.tx_split0
    }

    pub unsafe fn tx_split1(&self) -> u16 {
        self.c2rust_unnamed.c2rust_unnamed_0.tx_split1
    }
}
