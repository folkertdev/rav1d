use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::iclip;
use crate::src::lf_mask::Av1FilterLUT;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;

#[cfg(feature = "asm")]
use crate::src::cpu::{rav1d_get_cpu_flags, CpuFlags};

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cfg(feature = "asm")]
use crate::include::common::bitdepth::BPC;

pub type loopfilter_sb_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    *const u32,
    *const [u8; 4],
    ptrdiff_t,
    *const Av1FilterLUT,
    c_int,
    c_int,
) -> ();

#[repr(C)]
pub struct Rav1dLoopFilterDSPContext {
    pub loop_filter_sb: [[loopfilter_sb_fn; 2]; 2],
}

#[cfg(all(
    feature = "asm",
    feature = "bitdepth_8",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
extern "C" {
    fn dav1d_lpf_v_sb_uv_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_uv_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_v_sb_y_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_y_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
}

#[cfg(all(feature = "asm", feature = "bitdepth_8", target_arch = "x86_64",))]
extern "C" {
    fn dav1d_lpf_v_sb_uv_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_uv_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_v_sb_y_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_y_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_v_sb_uv_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_uv_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_v_sb_y_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_y_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
}

#[cfg(all(
    feature = "asm",
    feature = "bitdepth_8",
    any(target_arch = "arm", target_arch = "aarch64")
))]
extern "C" {
    fn dav1d_lpf_h_sb_uv_8bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_v_sb_y_8bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_y_8bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_v_sb_uv_8bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
}

#[cfg(all(
    feature = "asm",
    feature = "bitdepth_16",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
extern "C" {
    fn dav1d_lpf_v_sb_uv_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_uv_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_v_sb_y_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_y_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
}

#[cfg(all(feature = "asm", feature = "bitdepth_16", target_arch = "x86_64",))]
extern "C" {
    fn dav1d_lpf_v_sb_uv_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_uv_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_v_sb_y_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_y_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_v_sb_uv_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_uv_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_v_sb_y_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_y_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
}

#[cfg(all(
    feature = "asm",
    feature = "bitdepth_16",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
extern "C" {
    fn dav1d_lpf_v_sb_uv_16bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_uv_16bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_v_sb_y_16bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_lpf_h_sb_y_16bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const u32,
        lvl: *const [u8; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: c_int,
        bitdepth_max: c_int,
    );
}

#[inline(never)]
unsafe fn loop_filter<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    mut E: c_int,
    mut I: c_int,
    mut H: c_int,
    stridea: ptrdiff_t,
    strideb: ptrdiff_t,
    wd: c_int,
    bd: BD,
) {
    let bitdepth_min_8 = bd.bitdepth() - 8;
    let F = 1 << bitdepth_min_8;
    E <<= bitdepth_min_8;
    I <<= bitdepth_min_8;
    H <<= bitdepth_min_8;
    let mut i = 0;
    while i < 4 {
        let mut p6 = 0;
        let mut p5 = 0;
        let mut p4 = 0;
        let mut p3 = 0;
        let mut p2 = 0;
        let p1 = (*dst.offset(strideb * -(2 as c_int) as isize)).as_::<c_int>();
        let p0 = (*dst.offset(strideb * -(1 as c_int) as isize)).as_::<c_int>();
        let q0 = (*dst.offset((strideb * 0) as isize)).as_::<c_int>();
        let q1 = (*dst.offset((strideb * 1) as isize)).as_::<c_int>();
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;
        let mut q5 = 0;
        let mut q6 = 0;
        let mut fm;
        let mut flat8out = 0;
        let mut flat8in = 0;
        fm = ((p1 - p0).abs() <= I
            && (q1 - q0).abs() <= I
            && (p0 - q0).abs() * 2 + ((p1 - q1).abs() >> 1) <= E) as c_int;
        if wd > 4 {
            p2 = (*dst.offset(strideb * -(3 as c_int) as isize)).as_::<c_int>();
            q2 = (*dst.offset((strideb * 2) as isize)).as_::<c_int>();
            fm &= ((p2 - p1).abs() <= I && (q2 - q1).abs() <= I) as c_int;
            if wd > 6 {
                p3 = (*dst.offset(strideb * -(4 as c_int) as isize)).as_::<c_int>();
                q3 = (*dst.offset((strideb * 3) as isize)).as_::<c_int>();
                fm &= ((p3 - p2).abs() <= I && (q3 - q2).abs() <= I) as c_int;
            }
        }
        if !(fm == 0) {
            if wd >= 16 {
                p6 = (*dst.offset(strideb * -(7 as c_int) as isize)).as_::<c_int>();
                p5 = (*dst.offset(strideb * -(6 as c_int) as isize)).as_::<c_int>();
                p4 = (*dst.offset(strideb * -(5 as c_int) as isize)).as_::<c_int>();
                q4 = (*dst.offset((strideb * 4) as isize)).as_::<c_int>();
                q5 = (*dst.offset((strideb * 5) as isize)).as_::<c_int>();
                q6 = (*dst.offset(strideb * 6)).as_::<c_int>();
                flat8out = ((p6 - p0).abs() <= F
                    && (p5 - p0).abs() <= F
                    && (p4 - p0).abs() <= F
                    && (q4 - q0).abs() <= F
                    && (q5 - q0).abs() <= F
                    && (q6 - q0).abs() <= F) as c_int;
            }
            if wd >= 6 {
                flat8in = ((p2 - p0).abs() <= F
                    && (p1 - p0).abs() <= F
                    && (q1 - q0).abs() <= F
                    && (q2 - q0).abs() <= F) as c_int;
            }
            if wd >= 8 {
                flat8in &= ((p3 - p0).abs() <= F && (q3 - q0).abs() <= F) as c_int;
            }
            if wd >= 16 && flat8out & flat8in != 0 {
                *dst.offset(strideb * -(6 as c_int) as isize) = (p6
                    + p6
                    + p6
                    + p6
                    + p6
                    + p6 * 2
                    + p5 * 2
                    + p4 * 2
                    + p3
                    + p2
                    + p1
                    + p0
                    + q0
                    + 8
                    >> 4)
                    .as_::<BD::Pixel>();
                *dst.offset(strideb * -(5 as c_int) as isize) = (p6
                    + p6
                    + p6
                    + p6
                    + p6
                    + p5 * 2
                    + p4 * 2
                    + p3 * 2
                    + p2
                    + p1
                    + p0
                    + q0
                    + q1
                    + 8
                    >> 4)
                    .as_::<BD::Pixel>();
                *dst.offset(strideb * -(4 as c_int) as isize) = (p6
                    + p6
                    + p6
                    + p6
                    + p5
                    + p4 * 2
                    + p3 * 2
                    + p2 * 2
                    + p1
                    + p0
                    + q0
                    + q1
                    + q2
                    + 8
                    >> 4)
                    .as_::<BD::Pixel>();
                *dst.offset(strideb * -(3 as c_int) as isize) = (p6
                    + p6
                    + p6
                    + p5
                    + p4
                    + p3 * 2
                    + p2 * 2
                    + p1 * 2
                    + p0
                    + q0
                    + q1
                    + q2
                    + q3
                    + 8
                    >> 4)
                    .as_::<BD::Pixel>();
                *dst.offset(strideb * -(2 as c_int) as isize) = (p6
                    + p6
                    + p5
                    + p4
                    + p3
                    + p2 * 2
                    + p1 * 2
                    + p0 * 2
                    + q0
                    + q1
                    + q2
                    + q3
                    + q4
                    + 8
                    >> 4)
                    .as_::<BD::Pixel>();
                *dst.offset(strideb * -(1 as c_int) as isize) = (p6
                    + p5
                    + p4
                    + p3
                    + p2
                    + p1 * 2
                    + p0 * 2
                    + q0 * 2
                    + q1
                    + q2
                    + q3
                    + q4
                    + q5
                    + 8
                    >> 4)
                    .as_::<BD::Pixel>();
                *dst.offset((strideb * 0) as isize) = (p5
                    + p4
                    + p3
                    + p2
                    + p1
                    + p0 * 2
                    + q0 * 2
                    + q1 * 2
                    + q2
                    + q3
                    + q4
                    + q5
                    + q6
                    + 8
                    >> 4)
                    .as_::<BD::Pixel>();
                *dst.offset((strideb * 1) as isize) = (p4
                    + p3
                    + p2
                    + p1
                    + p0
                    + q0 * 2
                    + q1 * 2
                    + q2 * 2
                    + q3
                    + q4
                    + q5
                    + q6
                    + q6
                    + 8
                    >> 4)
                    .as_::<BD::Pixel>();
                *dst.offset((strideb * 2) as isize) = (p3
                    + p2
                    + p1
                    + p0
                    + q0
                    + q1 * 2
                    + q2 * 2
                    + q3 * 2
                    + q4
                    + q5
                    + q6
                    + q6
                    + q6
                    + 8
                    >> 4)
                    .as_::<BD::Pixel>();
                *dst.offset((strideb * 3) as isize) = (p2
                    + p1
                    + p0
                    + q0
                    + q1
                    + q2 * 2
                    + q3 * 2
                    + q4 * 2
                    + q5
                    + q6
                    + q6
                    + q6
                    + q6
                    + 8
                    >> 4)
                    .as_::<BD::Pixel>();
                *dst.offset((strideb * 4) as isize) = (p1
                    + p0
                    + q0
                    + q1
                    + q2
                    + q3 * 2
                    + q4 * 2
                    + q5 * 2
                    + q6
                    + q6
                    + q6
                    + q6
                    + q6
                    + 8
                    >> 4)
                    .as_::<BD::Pixel>();
                *dst.offset((strideb * 5) as isize) = (p0
                    + q0
                    + q1
                    + q2
                    + q3
                    + q4 * 2
                    + q5 * 2
                    + q6 * 2
                    + q6
                    + q6
                    + q6
                    + q6
                    + q6
                    + 8
                    >> 4)
                    .as_::<BD::Pixel>();
            } else if wd >= 8 && flat8in != 0 {
                *dst.offset(strideb * -(3 as c_int) as isize) =
                    (p3 + p3 + p3 + 2 * p2 + p1 + p0 + q0 + 4 >> 3).as_::<BD::Pixel>();
                *dst.offset(strideb * -(2 as c_int) as isize) =
                    (p3 + p3 + p2 + 2 * p1 + p0 + q0 + q1 + 4 >> 3).as_::<BD::Pixel>();
                *dst.offset(strideb * -(1 as c_int) as isize) =
                    (p3 + p2 + p1 + 2 * p0 + q0 + q1 + q2 + 4 >> 3).as_::<BD::Pixel>();
                *dst.offset((strideb * 0) as isize) =
                    (p2 + p1 + p0 + 2 * q0 + q1 + q2 + q3 + 4 >> 3).as_::<BD::Pixel>();
                *dst.offset((strideb * 1) as isize) =
                    (p1 + p0 + q0 + 2 * q1 + q2 + q3 + q3 + 4 >> 3).as_::<BD::Pixel>();
                *dst.offset((strideb * 2) as isize) =
                    (p0 + q0 + q1 + 2 * q2 + q3 + q3 + q3 + 4 >> 3).as_::<BD::Pixel>();
            } else if wd == 6 && flat8in != 0 {
                *dst.offset(strideb * -(2 as c_int) as isize) =
                    (p2 + 2 * p2 + 2 * p1 + 2 * p0 + q0 + 4 >> 3).as_::<BD::Pixel>();
                *dst.offset(strideb * -(1 as c_int) as isize) =
                    (p2 + 2 * p1 + 2 * p0 + 2 * q0 + q1 + 4 >> 3).as_::<BD::Pixel>();
                *dst.offset((strideb * 0) as isize) =
                    (p1 + 2 * p0 + 2 * q0 + 2 * q1 + q2 + 4 >> 3).as_::<BD::Pixel>();
                *dst.offset((strideb * 1) as isize) =
                    (p0 + 2 * q0 + 2 * q1 + 2 * q2 + q2 + 4 >> 3).as_::<BD::Pixel>();
            } else {
                let hev = ((p1 - p0).abs() > H || (q1 - q0).abs() > H) as c_int;

                fn iclip_diff(v: c_int, bitdepth_min_8: u8) -> i32 {
                    iclip(
                        v,
                        -128 * (1 << bitdepth_min_8),
                        128 * (1 << bitdepth_min_8) - 1,
                    )
                }

                if hev != 0 {
                    let mut f = iclip_diff(p1 - q1, bitdepth_min_8);
                    f = iclip_diff(3 * (q0 - p0) + f, bitdepth_min_8);

                    let f1 = cmp::min(f + 4, ((128 as c_int) << bitdepth_min_8) - 1) >> 3;
                    let f2 = cmp::min(f + 3, ((128 as c_int) << bitdepth_min_8) - 1) >> 3;

                    *dst.offset(strideb * -(1 as c_int) as isize) = bd.iclip_pixel(p0 + f2);
                    *dst.offset((strideb * 0) as isize) = bd.iclip_pixel(q0 - f1);
                } else {
                    let mut f = iclip_diff(3 * (q0 - p0), bitdepth_min_8);

                    let f1 = cmp::min(f + 4, ((128 as c_int) << bitdepth_min_8) - 1) >> 3;
                    let f2 = cmp::min(f + 3, ((128 as c_int) << bitdepth_min_8) - 1) >> 3;

                    *dst.offset(strideb * -(1 as c_int) as isize) = bd.iclip_pixel(p0 + f2);
                    *dst.offset((strideb * 0) as isize) = bd.iclip_pixel(q0 - f1);

                    f = (f1 + 1) >> 1;
                    *dst.offset(strideb * -(2 as c_int) as isize) = bd.iclip_pixel(p1 + f);
                    *dst.offset((strideb * 1) as isize) = bd.iclip_pixel(q1 - f);
                }
            }
        }
        i += 1;
        dst = dst.offset(stridea as isize);
    }
}

unsafe extern "C" fn loop_filter_h_sb128y_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    vmask: *const u32,
    l: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: *const Av1FilterLUT,
    h: c_int,
    bitdepth_max: c_int,
) {
    loop_filter_h_sb128y_rust(
        dst.cast(),
        stride,
        vmask,
        l,
        b4_stride,
        lut,
        h,
        BD::from_c(bitdepth_max),
    )
}

unsafe fn loop_filter_h_sb128y_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    vmask: *const u32,
    mut l: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: *const Av1FilterLUT,
    _h: c_int,
    bd: BD,
) {
    let vm: c_uint = *vmask.offset(0) | *vmask.offset(1) | *vmask.offset(2);
    let mut y: c_uint = 1 as c_int as c_uint;
    while vm & !y.wrapping_sub(1 as c_int as c_uint) != 0 {
        if vm & y != 0 {
            let L = if (*l.offset(0))[0] as c_int != 0 {
                (*l.offset(0))[0] as c_int
            } else {
                (*l.offset(-(1 as c_int) as isize))[0] as c_int
            };
            if !(L == 0) {
                let H = L >> 4;
                let E = (*lut).e[L as usize] as c_int;
                let I = (*lut).i[L as usize] as c_int;
                let idx = if *vmask.offset(2) & y != 0 {
                    2 as c_int
                } else {
                    (*vmask.offset(1) & y != 0) as c_int
                };
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    BD::pxstride(stride as usize) as isize,
                    1 as c_int as ptrdiff_t,
                    (4 as c_int) << idx,
                    bd,
                );
            }
        }
        y <<= 1;
        dst = dst.offset(4 * BD::pxstride(stride as usize) as isize);
        l = l.offset(b4_stride as isize);
    }
}

unsafe extern "C" fn loop_filter_v_sb128y_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    vmask: *const u32,
    l: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: *const Av1FilterLUT,
    w: c_int,
    bitdepth_max: c_int,
) {
    loop_filter_v_sb128y_rust(
        dst.cast(),
        stride,
        vmask,
        l,
        b4_stride,
        lut,
        w,
        BD::from_c(bitdepth_max),
    );
}

unsafe fn loop_filter_v_sb128y_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    vmask: *const u32,
    mut l: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: *const Av1FilterLUT,
    _w: c_int,
    bd: BD,
) {
    let vm: c_uint = *vmask.offset(0) | *vmask.offset(1) | *vmask.offset(2);
    let mut x: c_uint = 1 as c_int as c_uint;
    while vm & !x.wrapping_sub(1 as c_int as c_uint) != 0 {
        if vm & x != 0 {
            let L = if (*l.offset(0))[0] as c_int != 0 {
                (*l.offset(0))[0] as c_int
            } else {
                (*l.offset(-b4_stride as isize))[0] as c_int
            };
            if !(L == 0) {
                let H = L >> 4;
                let E = (*lut).e[L as usize] as c_int;
                let I = (*lut).i[L as usize] as c_int;
                let idx = if *vmask.offset(2) & x != 0 {
                    2 as c_int
                } else {
                    (*vmask.offset(1) & x != 0) as c_int
                };
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    1 as c_int as ptrdiff_t,
                    BD::pxstride(stride as usize) as isize,
                    (4 as c_int) << idx,
                    bd,
                );
            }
        }
        x <<= 1;
        dst = dst.offset(4);
        l = l.offset(1);
    }
}

unsafe extern "C" fn loop_filter_h_sb128uv_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    vmask: *const u32,
    l: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: *const Av1FilterLUT,
    h: c_int,
    bitdepth_max: c_int,
) {
    loop_filter_h_sb128uv_rust(
        dst.cast(),
        stride,
        vmask,
        l,
        b4_stride,
        lut,
        h,
        BD::from_c(bitdepth_max),
    )
}

unsafe fn loop_filter_h_sb128uv_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    vmask: *const u32,
    mut l: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: *const Av1FilterLUT,
    _h: c_int,
    bd: BD,
) {
    let vm: c_uint = *vmask.offset(0) | *vmask.offset(1);
    let mut y: c_uint = 1 as c_int as c_uint;
    while vm & !y.wrapping_sub(1 as c_int as c_uint) != 0 {
        if vm & y != 0 {
            let L = if (*l.offset(0))[0] as c_int != 0 {
                (*l.offset(0))[0] as c_int
            } else {
                (*l.offset(-(1 as c_int) as isize))[0] as c_int
            };
            if !(L == 0) {
                let H = L >> 4;
                let E = (*lut).e[L as usize] as c_int;
                let I = (*lut).i[L as usize] as c_int;
                let idx = (*vmask.offset(1) & y != 0) as c_int;
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    BD::pxstride(stride as usize) as isize,
                    1 as c_int as ptrdiff_t,
                    4 + 2 * idx,
                    bd,
                );
            }
        }
        y <<= 1;
        dst = dst.offset(4 * BD::pxstride(stride as usize) as isize);
        l = l.offset(b4_stride as isize);
    }
}

unsafe extern "C" fn loop_filter_v_sb128uv_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    vmask: *const u32,
    l: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: *const Av1FilterLUT,
    w: c_int,
    bitdepth_max: c_int,
) {
    loop_filter_v_sb128uv_rust(
        dst.cast(),
        stride,
        vmask,
        l,
        b4_stride,
        lut,
        w,
        BD::from_c(bitdepth_max),
    )
}

unsafe fn loop_filter_v_sb128uv_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    vmask: *const u32,
    mut l: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: *const Av1FilterLUT,
    _w: c_int,
    bd: BD,
) {
    let vm: c_uint = *vmask.offset(0) | *vmask.offset(1);
    let mut x: c_uint = 1 as c_int as c_uint;
    while vm & !x.wrapping_sub(1 as c_int as c_uint) != 0 {
        if vm & x != 0 {
            let L = if (*l.offset(0))[0] as c_int != 0 {
                (*l.offset(0))[0] as c_int
            } else {
                (*l.offset(-b4_stride as isize))[0] as c_int
            };
            if !(L == 0) {
                let H = L >> 4;
                let E = (*lut).e[L as usize] as c_int;
                let I = (*lut).i[L as usize] as c_int;
                let idx = (*vmask.offset(1) & x != 0) as c_int;
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    1 as c_int as ptrdiff_t,
                    BD::pxstride(stride as usize) as isize,
                    4 + 2 * idx,
                    bd,
                );
            }
        }
        x <<= 1;
        dst = dst.offset(4);
        l = l.offset(1);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe fn loop_filter_dsp_init_x86<BD: BitDepth>(c: *mut Rav1dLoopFilterDSPContext) {
    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::SSSE3) {
        return;
    }
    match BD::BPC {
        BPC::BPC8 => {
            (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_8bpc_ssse3;
            (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_8bpc_ssse3;
            (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_8bpc_ssse3;
            (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_8bpc_ssse3;

            #[cfg(target_arch = "x86_64")]
            {
                if !flags.contains(CpuFlags::AVX2) {
                    return;
                }

                (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_8bpc_avx2;
                (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_8bpc_avx2;
                (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_8bpc_avx2;
                (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_8bpc_avx2;

                if !flags.contains(CpuFlags::AVX512ICL) {
                    return;
                }

                (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_8bpc_avx512icl;
                (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_8bpc_avx512icl;
                (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_8bpc_avx512icl;
                (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_8bpc_avx512icl;
            }
        }
        BPC::BPC16 => {
            (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_16bpc_ssse3;
            (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_16bpc_ssse3;
            (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_16bpc_ssse3;
            (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_16bpc_ssse3;

            #[cfg(target_arch = "x86_64")]
            {
                if !flags.contains(CpuFlags::AVX2) {
                    return;
                }

                (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_16bpc_avx2;
                (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_16bpc_avx2;
                (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_16bpc_avx2;
                (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_16bpc_avx2;

                if !flags.contains(CpuFlags::AVX512ICL) {
                    return;
                }

                (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_16bpc_avx512icl;
                (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_16bpc_avx512icl;
                (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_16bpc_avx512icl;
                (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_16bpc_avx512icl;
            }
        }
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
unsafe fn loop_filter_dsp_init_arm<BD: BitDepth>(c: *mut Rav1dLoopFilterDSPContext) {
    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::NEON) {
        return;
    }

    match BD::BPC {
        BPC::BPC8 => {
            (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_8bpc_neon;
            (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_8bpc_neon;
            (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_8bpc_neon;
            (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_8bpc_neon;
        }
        BPC::BPC16 => {
            (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_16bpc_neon;
            (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_16bpc_neon;
            (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_16bpc_neon;
            (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_16bpc_neon;
        }
    }
}

#[cold]
pub unsafe fn rav1d_loop_filter_dsp_init<BD: BitDepth>(c: *mut Rav1dLoopFilterDSPContext) {
    (*c).loop_filter_sb[0][0] = loop_filter_h_sb128y_c_erased::<BD>;
    (*c).loop_filter_sb[0][1] = loop_filter_v_sb128y_c_erased::<BD>;
    (*c).loop_filter_sb[1][0] = loop_filter_h_sb128uv_c_erased::<BD>;
    (*c).loop_filter_sb[1][1] = loop_filter_v_sb128uv_c_erased::<BD>;

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            loop_filter_dsp_init_x86::<BD>(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            loop_filter_dsp_init_arm::<BD>(c);
        }
    }
}
