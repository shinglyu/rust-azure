// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::*;
use skia::SkiaGrContextRef;

pub type AzFontOptions = *mut c_void;
pub type AzFloat = c_float;

pub type enum_AzSurfaceType = c_uint;
pub static AZ_SURFACE_DATA: u32 = 0_u32;
pub static AZ_SURFACE_D2D1_BITMAP: u32 = 1_u32;
pub static AZ_SURFACE_D2D1_DRAWTARGET: u32 = 2_u32;
pub static AZ_SURFACE_CAIRO: u32 = 3_u32;
pub static AZ_SURFACE_CAIRO_IMAGE: u32 = 4_u32;
pub static AZ_SURFACE_COREGRAPHICS_IMAGE: u32 = 5_u32;
pub static AZ_SURFACE_COREGRAPHICS_CGCONTEXT: u32 = 6_u32;
pub static AZ_SURFACE_SKIA: u32 = 7_u32;
pub static AZ_SURFACE_DUAL_DT: u32 = 8_u32;
pub static AZ_SURFACE_D2D1_1_IMAGE: u32 = 9_u32;
pub static AZ_SURFACE_RECORDING: u32 = 10_u32;
pub static AZ_SURFACE_NVPR_TEXTURE: u32 = 11_u32;
pub static AZ_SURFACE_TILED: u32 = 12_u32;

pub type enum_AzSurfaceFormat = c_uint;
pub static AZ_FORMAT_B8G8R8A8: u32 = 0_u32;
pub static AZ_FORMAT_B8G8R8X8: u32 = 1_u32;
pub static AZ_FORMAT_R8G8B8A8: u32 = 2_u32;
pub static AZ_FORMAT_R8G8B8X8: u32 = 3_u32;
pub static AZ_FORMAT_R5G6B5: u32 = 4_u32;
pub static AZ_FORMAT_A8: u32 = 5_u32;
pub static AZ_FORMAT_YUV: u32 = 6_u32;
pub static AZ_FORMAT_UNKNOWN: u32 = 7_u32;

pub type AzSurfaceFormat = enum_AzSurfaceFormat;

pub type enum_AzBackendType = c_uint;
pub static AZ_BACKEND_NONE: u32 = 0_u32;
pub static AZ_BACKEND_DIRECT2D: u32 = 1_u32;
pub static AZ_BACKEND_COREGRAPHICS: u32 = 2_u32;
pub static AZ_BACKEND_COREGRAPHICS_ACCELERATED: u32 = 3_u32;
pub static AZ_BACKEND_CAIRO: u32 = 4_u32;
pub static AZ_BACKEND_SKIA: u32 = 5_u32;
pub static AZ_BACKEND_RECORDING: u32 = 6_u32;
pub static AZ_BACKEND_DIRECT2D1_1: u32 = 7_u32;
pub static AZ_BACKEND_NVP: u32 = 8_u32;

pub type AzBackendType = enum_AzBackendType;

pub type enum_AzFilterType = c_uint;
pub static AZ_FILTER_TYPE_BLEND: u32 = 0_u32;
pub static AZ_FILTER_TYPE_TRANSFORM: u32 = 1_u32;
pub static AZ_FILTER_TYPE_MORPHOLOGY: u32 = 2_u32;
pub static AZ_FILTER_TYPE_COLOR_MATRIX: u32 = 3_u32;
pub static AZ_FILTER_TYPE_FLOOD: u32 = 4_u32;
pub static AZ_FILTER_TYPE_TILE: u32 = 5_u32;
pub static AZ_FILTER_TYPE_TABLE_TRANSFER: u32 = 6_u32;
pub static AZ_FILTER_TYPE_DISCRETE_TRANSFER: u32 = 7_u32;
pub static AZ_FILTER_TYPE_LINEAR_TRANSFER: u32 = 8_u32;
pub static AZ_FILTER_TYPE_GAMMA_TRANSFER: u32 = 9_u32;
pub static AZ_FILTER_TYPE_CONVOLVE_MATRIX: u32 = 10_u32;
pub static AZ_FILTER_TYPE_DISPLACEMENT_MAP: u32 = 11_u32;
pub static AZ_FILTER_TYPE_TURBULENCE: u32 = 12_u32;
pub static AZ_FILTER_TYPE_ARITHMETIC_COMBINE: u32 = 13_u32;
pub static AZ_FILTER_TYPE_COMPOSITE: u32 = 14_u32;
pub static AZ_FILTER_TYPE_DIRECTIONAL_BLUR: u32 = 15_u32;
pub static AZ_FILTER_TYPE_GAUSSIAN_BLUR: u32 = 16_u32;
pub static AZ_FILTER_TYPE_POINT_DIFFUSE: u32 = 17_u32;
pub static AZ_FILTER_TYPE_POINT_SPECULAR: u32 = 18_u32;
pub static AZ_FILTER_TYPE_SPOT_DIFFUSE: u32 = 19_u32;
pub static AZ_FILTER_TYPE_SPOT_SPECULAR: u32 = 20_u32;
pub static AZ_FILTER_TYPE_DISTANT_DIFFUSE: u32 = 21_u32;
pub static AZ_FILTER_TYPE_DISTANT_SPECULAR: u32 = 22_u32;
pub static AZ_FILTER_TYPE_CROP: u32 = 23_u32;
pub static AZ_FILTER_TYPE_PREMULTIPLY: u32 = 24_u32;
pub static AZ_FILTER_TYPE_UNPREMULTIPLY: u32 = 25_u32;

pub type AzFilterType = enum_AzFilterType;

pub type enum_AzFontType = c_uint;
pub static AZ_FONT_DWRITE: u32 = 0_u32;
pub static AZ_FONT_GDI: u32 = 1_u32;
pub static AZ_FONT_MAC: u32 = 2_u32;
pub static AZ_FONT_SKIA: u32 = 3_u32;
pub static AZ_FONT_CAIRO: u32 = 4_u32;
pub static AZ_FONT_COREGRAPHICS: u32 = 5_u32;
pub static AZ_FONT_NVPR: u32 = 6_u32;

pub type enum_AzNativeSurfaceType = c_uint;
pub static AZ_NATIVE_SURFACE_D3D10_TEXTURE: u32 = 0_u32;
pub static AZ_NATIVE_SURFACE_CAIRO_SURFACE: u32 = 1_u32;
pub static AZ_NATIVE_SURFACE_CAIRO_CONTEXT: u32 = 2_u32;
pub static AZ_NATIVE_SURFACE_CGCONTEXT: u32 = 3_u32;
pub static AZ_NATIVE_SURFACE_CGCONTEXT_ACCELERATED: u32 = 4_u32;
pub static AZ_NATIVE_SURFACE_OPENGL_TEXTUR: u32 = 5_u32;

pub type enum_AzNativeFontType = c_uint;
pub static AZ_NATIVE_FONT_DWRITE_FONT_FACE: u32 = 0_u32;
pub static AZ_NATIVE_FONT_GDI_FONT_FACE: u32 = 1_u32;
pub static AZ_NATIVE_FONT_MAC_FONT_FACE: u32 = 2_u32;
pub static AZ_NATIVE_FONT_SKIA_FONT_FACE: u32 = 3_u32;
pub static AZ_NATIVE_FONT_CAIRO_FONT_FACE: u32 = 4_u32;
pub static AZ_NATIVE_FONT_NVPR_FONT_FAC: u32 = 5_u32;

pub type enum_AzFontStyle = c_uint;
pub static AZ_FONT_STYLE_NORMAL: u32 = 0_u32;
pub static AZ_FONT_STYLE_ITALIC: u32 = 1_u32;
pub static AZ_FONT_STYLE_BOLD: u32 = 2_u32;
pub static AZ_FONT_STYLE_BOLD_ITALIC: u32 = 3_u32;

pub type enum_AzCompositionOp = c_uchar;
pub static AZ_OP_OVER: u8 = 0_u8;
pub static AZ_OP_ADD: u8 = 1_u8;
pub static AZ_OP_ATOP: u8 = 2_u8;
pub static AZ_OP_OUT: u8 = 3_u8;
pub static AZ_OP_IN: u8 = 4_u8;
pub static AZ_OP_SOURCE: u8 = 5_u8;
pub static AZ_OP_DEST_IN: u8 = 6_u8;
pub static AZ_OP_DEST_OUT: u8 = 7_u8;
pub static AZ_OP_DEST_OVER: u8 = 8_u8;
pub static AZ_OP_DEST_ATOP: u8 = 9_u8;
pub static AZ_OP_XOR: u8 = 10_u8;
pub static AZ_OP_MULTIPLY: u8 = 11_u8;
pub static AZ_OP_SCREEN: u8 = 12_u8;
pub static AZ_OP_OVERLAY: u8 = 13_u8;
pub static AZ_OP_DARKEN: u8 = 14_u8;
pub static AZ_OP_LIGHTEN: u8 = 15_u8;
pub static AZ_OP_COLOR_DODGE: u8 = 16_u8;
pub static AZ_OP_COLOR_BURN: u8 = 17_u8;
pub static AZ_OP_HARD_LIGHT: u8 = 18_u8;
pub static AZ_OP_SOFT_LIGHT: u8 = 19_u8;
pub static AZ_OP_DIFFERENCE: u8 = 20_u8;
pub static AZ_OP_EXCLUSION: u8 = 21_u8;
pub static AZ_OP_HUE: u8 = 22_u8;
pub static AZ_OP_SATURATION: u8 = 23_u8;
pub static AZ_OP_COLOR: u8 = 24_u8;
pub static AZ_OP_LUMINOSITY: u8 = 25_u8;
pub static AZ_OP_COUNT: u8 = 26_u8;
pub type AzCompositionOp = enum_AzCompositionOp;

pub type enum_AzExtendMode = c_uint;
pub static AZ_EXTEND_CLAMP: u32 = 0_u32;
pub static AZ_EXTEND_REPEAT: u32 = 1_u32;
pub static AZ_EXTEND_REFLECT: u32 = 2_u32;

pub type enum_AzFillRule = c_uint;
pub static AZ_FILL_WINDING: u32 = 0_u32;
pub static AZ_FILL_EVEN_ODD: u32 = 1_u32;

pub type enum_AzAntialiasMode = c_uchar;
pub static AZ_AA_NONE: u8 = 0_u8;
pub static AZ_AA_GRAY: u8 = 1_u8;
pub static AZ_AA_SUBPIXEL: u8 = 2_u8;
pub static AZ_AA_DEFAULT: u8 = 3_u8;
pub type AzAntialiasMode = enum_AzCompositionOp;

pub type enum_AzFilter = c_uint;
pub static AZ_FILTER_GOOD: u32 = 0_u32;
pub static AZ_FILTER_LINEAR: u32 = 1_u32;
pub static AZ_FILTER_POINT: u32 = 2_u32;

pub type AzFilter = enum_AzFilter;

pub type enum_AzPatternType = c_uint;
pub static AZ_PATTERN_COLOR: u32 = 0_u32;
pub static AZ_PATTERN_SURFACE: u32 = 1_u32;
pub static AZ_PATTERN_LINEAR_GRADIENT: u32 = 2_u32;
pub static AZ_PATTERN_RADIAL_GRADIENT: u32 = 3_u32;

pub type enum_AzJoinStyle = c_uchar;
pub static AZ_JOIN_BEVEL: u8 = 0_u8;
pub static AZ_JOIN_ROUND: u8 = 1_u8;
pub static AZ_JOIN_MITER: u8 = 2_u8;
pub static AZ_JOIN_MITER_OR_BEVEL: u8 = 3_u8;

pub type AzJoinStyle = enum_AzJoinStyle;

pub type enum_AzCapStyle = c_uchar;
pub static AZ_CAP_BUTT: u8 = 0_u8;
pub static AZ_CAP_ROUND: u8 = 1_u8;
pub static AZ_CAP_SQUARE: u8 = 2_u8;

pub type AzCapStyle = enum_AzCapStyle;

pub type enum_AzSamplingBounds = c_uint;
pub static AZ_SAMPLING_UNBOUNDED: u32 = 0_u32;
pub static AZ_SAMPLING_BOUNDED: u32 = 1_u32;

pub type enum_AzSide = c_uint;
pub static AZ_eSideTop: u32 = 0_u32;
pub static AZ_eSideRight: u32 = 1_u32;
pub static AZ_eSideBottom: u32 = 2_u32;
pub static AZ_eSideLeft: u32 = 3_u32;

pub type enum_AzCompositeFilterAtts = u32;
pub static AZ_ATT_COMPOSITE_COMPOSITE_OPERATOR: u32 = 0;

pub type enum_AzTransformFilterAtts = u32;
pub static AZ_ATT_TRANSFORM_MATRIX: u32 = 0;

pub type enum_AzColorMatrixFilterAtts = u32;
pub static AZ_ATT_COLOR_MATRIX_MATRIX: u32 = 0;

pub type enum_AzTransformFilterInputs = u32;
pub static AZ_IN_TRANSFORM_IN: u32 = 0;

pub type enum_AzColorMatrixFilterInputs = u32;
pub static AZ_IN_COLOR_MATRIX_IN: u32 = 0;

pub type enum_AzCompositeFilterInputs = u32;
pub static AZ_IN_COMPOSITE_IN: u32 = 0;

pub type enum_AzFloodFilterAtts = u32;
pub static AZ_ATT_FLOOD_COLOR: u32 = 0;

pub type enum_AzFloodFilterInputs = u32;
pub static AZ_IN_FLOOD_IN: u32 = 0;

pub type enum_AzGaussianBlurAtts = u32;
pub static AZ_ATT_GAUSSIAN_BLUR_STD_DEVIATION: u32 = 0;

pub type enum_AzGaussianBlurInputs = u32;
pub static AZ_IN_GAUSSIAN_BLUR_IN: u32 = 0;

pub type enum_LinearTransferAtts = u32;
pub static AZ_ATT_LINEAR_TRANSFER_SLOPE_R: u32 = 4;
pub static AZ_ATT_LINEAR_TRANSFER_SLOPE_G: u32 = 5;
pub static AZ_ATT_LINEAR_TRANSFER_SLOPE_B: u32 = 6;
pub static AZ_ATT_LINEAR_TRANSFER_SLOPE_A: u32 = 7;
pub static AZ_ATT_LINEAR_TRANSFER_INTERCEPT_R: u32 = 8;
pub static AZ_ATT_LINEAR_TRANSFER_INTERCEPT_G: u32 = 9;
pub static AZ_ATT_LINEAR_TRANSFER_INTERCEPT_B: u32 = 10;
pub static AZ_ATT_LINEAR_TRANSFER_INTERCEPT_A: u32 = 11;

pub type enum_AzLinearTransferInputs = u32;
pub static AZ_IN_LINEAR_TRANSFER_IN: u32 = 0;

pub type enum_DiscreteTransferAtts = u32;
pub static AZ_ATT_DISCRETE_TRANSFER_SLOPE_R: u32 = 4;
pub static AZ_ATT_DISCRETE_TRANSFER_SLOPE_G: u32 = 5;
pub static AZ_ATT_DISCRETE_TRANSFER_SLOPE_B: u32 = 6;
pub static AZ_ATT_DISCRETE_TRANSFER_SLOPE_A: u32 = 7;
pub static AZ_ATT_DISCRETE_TRANSFER_INTERCEPT_R: u32 = 8;
pub static AZ_ATT_DISCRETE_TRANSFER_INTERCEPT_G: u32 = 9;
pub static AZ_ATT_DISCRETE_TRANSFER_INTERCEPT_B: u32 = 10;
pub static AZ_ATT_DISCRETE_TRANSFER_INTERCEPT_A: u32 = 11;

pub type enum_AzDiscreteTransferInputs = u32;
pub static AZ_IN_DISCRETE_TRANSFER_IN: u32 = 0;

pub type enum_AzTableTransferAtts = u32;
pub static AZ_ATT_TABLE_TRANSFER_TABLE_R: u32 = 4;
pub static AZ_ATT_TABLE_TRANSFER_TABLE_G: u32 = 5;
pub static AZ_ATT_TABLE_TRANSFER_TABLE_B: u32 = 6;
pub static AZ_ATT_TABLE_TRANSFER_TABLE_A: u32 = 7;

pub type enum_AzTableTransferInputs = u32;
pub static AZ_IN_TABLE_TRANSFER_IN: u32 = 0;

pub type enum_TransferAtts = u32;
pub static AZ_ATT_TRANSFER_DISABLE_R: u32 = 0;
pub static AZ_ATT_TRANSFER_DISABLE_G: u32 = 1;
pub static AZ_ATT_TRANSFER_DISABLE_B: u32 = 2;
pub static AZ_ATT_TRANSFER_DISABLE_A: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct struct__AzColor {
    pub r: AzFloat,
    pub g: AzFloat,
    pub b: AzFloat,
    pub a: AzFloat,
}

pub type AzColor = struct__AzColor;

#[inline]
impl PartialEq for AzColor {
    fn eq(&self, other: &AzColor) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}

#[repr(C)]
pub struct struct__AzGradientStop {
    pub offset: AzFloat,
    pub color: AzColor,
}

#[repr(C)]
pub type AzGradientStop = struct__AzGradientStop;

pub struct struct__AzIntRect {
    pub x: int32_t,
    pub y: int32_t,
    pub width: int32_t,
    pub height: int32_t,
}

pub type AzIntRect = struct__AzIntRect;

#[repr(C)]
pub struct struct__AzRect {
    pub x: AzFloat,
    pub y: AzFloat,
    pub width: AzFloat,
    pub height: AzFloat,
}

pub type AzRect = struct__AzRect;

pub struct struct__AzIntPoint {
    pub x: int32_t,
    pub y: int32_t,
}

pub type AzIntPoint = struct__AzIntPoint;

#[repr(C)]
pub struct struct__AzPoint {
    pub x: AzFloat,
    pub y: AzFloat,
}

pub type AzPoint = struct__AzPoint;

#[repr(C)]
pub struct struct__AzIntSize {
    pub width: int32_t,
    pub height: int32_t,
}

pub type AzIntSize = struct__AzIntSize;

#[repr(C)]
pub struct struct__AzSize {
    pub width: AzFloat,
    pub height: AzFloat,
}

pub type AzSize = struct__AzSize;

#[repr(C)]
pub struct struct__AzMatrix {
    pub _11: AzFloat,
    pub _12: AzFloat,
    pub _21: AzFloat,
    pub _22: AzFloat,
    pub _31: AzFloat,
    pub _32: AzFloat,
}

pub type AzMatrix = struct__AzMatrix;

#[repr(C)]
#[derive(PartialEq, Clone)]
pub struct struct__AzMatrix5x4 {
    pub _11: AzFloat, pub _12: AzFloat, pub _13: AzFloat, pub _14: AzFloat,
    pub _21: AzFloat, pub _22: AzFloat, pub _23: AzFloat, pub _24: AzFloat,
    pub _31: AzFloat, pub _32: AzFloat, pub _33: AzFloat, pub _34: AzFloat,
    pub _41: AzFloat, pub _42: AzFloat, pub _43: AzFloat, pub _44: AzFloat,
    pub _51: AzFloat, pub _52: AzFloat, pub _53: AzFloat, pub _54: AzFloat,
}

pub type AzMatrix5x4 = struct__AzMatrix5x4;

#[repr(C)]
pub struct struct__AzDrawOptions {
    pub mAlpha: AzFloat,
    pub mCompositionOp: AzCompositionOp,
    pub mAntialiasMode: AzAntialiasMode,
}

pub type AzDrawOptions = struct__AzDrawOptions;

#[repr(C)]
pub struct struct__AzStrokeOptions {
    pub mLineWidth: AzFloat,
    pub mMiterLimit: AzFloat,
    pub mDashPattern: *const AzFloat,
    pub mDashLength: size_t,
    pub mDashOffset: AzFloat,
    pub mLineJoin: AzJoinStyle,
    pub mLineCap: AzCapStyle,
}

pub type AzStrokeOptions = struct__AzStrokeOptions;

#[repr(C)]
pub struct struct__AzDrawSurfaceOptions {
    pub filter: i8,
    pub sampling_bounds: i8,
}

#[repr(C)]
pub type AzDrawSurfaceOptions = struct__AzDrawSurfaceOptions;

#[repr(C)]
pub struct struct__AzGlyph {
    pub mIndex: uint32_t,
    pub mPosition: AzPoint,
}

#[repr(C)]
pub type AzGlyph = struct__AzGlyph;

#[repr(C)]
pub struct struct__AzGlyphBuffer {
    pub mGlyphs: *mut AzGlyph,
    pub mNumGlyphs: uint32_t,
}

#[repr(C)]
pub type AzGlyphBuffer = struct__AzGlyphBuffer;

#[repr(C)]
pub struct struct__AzNativeFont {
    pub mType: enum_AzNativeFontType,
    pub mFont: *mut c_void,
}

pub type AzNativeFont = struct__AzNativeFont;

pub type AzGradientStopsRef = *mut c_void;

pub type AzDrawTargetRef = *mut c_void;

pub type AzPatternRef = *mut c_void;

pub type AzColorPatternRef = *mut c_void;

pub type AzLinearGradientPatternRef = *mut c_void;

pub type AzRadialGradientPatternRef = *mut c_void;

pub type AzSurfacePatternRef = *mut c_void;

pub type AzScaledFontRef = *mut c_void;

pub type AzGlyphRenderingOptionsRef = *mut c_void;

pub type AzSourceSurfaceRef = *mut c_void;

pub type AzDataSourceSurfaceRef = *mut c_void;

#[repr(C)]
pub type AzDrawSurfaceOptionsRef = *mut AzDrawSurfaceOptions;

pub type AzGLContext = *mut c_void;

pub type AzGLContextMetadataRef = *mut c_void;

pub type AzPathRef = *mut c_void;

pub type AzPathBuilderRef = *mut c_void;

pub type AzFilterNodeRef = *mut c_void;

pub type AzExtendMode = i32;

#[link(name = "azure")]
extern {

pub fn AzSanityCheck(/* FIXME: variadic function */);

pub fn AzCreateColorPattern(aColor: *mut AzColor) -> AzColorPatternRef;

pub fn AzReleasePattern(aPattern: AzPatternRef);

pub fn AzCreateDrawTarget(aBackend: AzBackendType, aSize: *mut AzIntSize, aFormat: AzSurfaceFormat) -> AzDrawTargetRef;

pub fn AzCreateDrawTargetForData(aBackend: AzBackendType, aData: *mut c_uchar, aSize: *mut AzIntSize, aStride: i32, aFormat: AzSurfaceFormat) -> AzDrawTargetRef;

pub fn AzCreateDrawTargetSkiaWithGrContextAndFBO(aGrContext: SkiaGrContextRef, aFBOID: u32, aSize: *mut AzIntSize, aFormat: AzSurfaceFormat) -> AzDrawTargetRef;

pub fn AzRetainDrawTarget(aTarget: AzDrawTargetRef);

pub fn AzReleaseDrawTarget(aTarget: AzDrawTargetRef);

pub fn AzDrawTargetGetSize(aDrawTarget: AzDrawTargetRef) -> AzIntSize;

pub fn AzDrawTargetGetFormat(aDrawTarget: AzDrawTargetRef) -> AzSurfaceFormat;

pub fn AzDrawTargetGetTransform(aDrawTarget: AzDrawTargetRef, aOutMatrix: *mut AzMatrix);

pub fn AzDrawTargetFlush(aDrawTarget: AzDrawTargetRef);

pub fn AzDrawTargetClearRect(aDrawTarget: AzDrawTargetRef, aRect: *mut AzRect);

pub fn AzDrawTargetFillRect(aDrawTarget: AzDrawTargetRef,
                            aRect: *mut AzRect,
                            aPattern: AzPatternRef,
                            aDrawOptions: *mut AzDrawOptions);

pub fn AzDrawTargetStrokeRect(aDrawTarget: AzDrawTargetRef, aRect: *mut AzRect, aPattern: AzPatternRef, aStrokeOptions: *mut AzStrokeOptions, aDrawOptions: *mut AzDrawOptions);

pub fn AzDrawTargetStrokeLine(aDrawTarget: AzDrawTargetRef, aStart: *mut AzPoint, aEnd: *mut AzPoint, aPattern: AzPatternRef, aStrokeOptions: *mut AzStrokeOptions, aDrawOptions: *mut AzDrawOptions);

pub fn AzDrawTargetFill(aDrawTarget: AzDrawTargetRef, aPath: AzPathRef, aPattern: AzPatternRef, aOptions: *mut AzDrawOptions);

pub fn AzDrawTargetStroke(aDrawTarget: AzDrawTargetRef, aPath: AzPathRef, aPattern: AzPatternRef, aStrokeOptions: *const AzStrokeOptions, aDrawOptions: *const AzDrawOptions);

pub fn AzDrawTargetPushClip(aDrawTarget: AzDrawTargetRef, aPath: AzPathRef);

pub fn AzDrawTargetPushClipRect(aDrawTarget: AzDrawTargetRef, aRect: *const AzRect);

pub fn AzDrawTargetPopClip(aDrawTarget: AzDrawTargetRef);

pub fn AzDrawTargetFillGlyphs(aDrawTarget: AzDrawTargetRef, aFont: AzScaledFontRef, aGlyphBuffer: *mut AzGlyphBuffer, aPattern: AzPatternRef, aOptions: *mut AzDrawOptions, aRenderingOptions: AzGlyphRenderingOptionsRef);

pub fn AzDrawTargetDrawSurface(aDrawTarget: AzDrawTargetRef, aSurface: AzSourceSurfaceRef, aDest: *mut AzRect, aSource: *mut AzRect, aSurfOptions: AzDrawSurfaceOptionsRef, aOptions: *mut AzDrawOptions);

pub fn AzDrawTargetDrawFilter(aDrawTarget: AzDrawTargetRef,
                              aFilter: AzFilterNodeRef,
                              aSourceRect: *const AzRect,
                              aDestPoint: *const AzPoint,
                              aOptions: *const AzDrawOptions);

pub fn AzDrawTargetDrawSurfaceWithShadow(aDrawTarget: AzDrawTargetRef,
                                         aSurface: AzSourceSurfaceRef,
                                         aDest: *const AzPoint,
                                         aColor: *const AzColor,
                                         aOffset: *const AzPoint,
                                         aSigma: AzFloat,
                                         aOperator: AzCompositionOp);

pub fn AzDrawTargetGetSnapshot(aDrawTarget: AzDrawTargetRef) -> AzSourceSurfaceRef;

pub fn AzDrawTargetCreateSourceSurfaceFromData(aDrawTarget: AzDrawTargetRef, aData: *const u8, aSize: *mut AzIntSize, aStride: i32, aFormat: AzSurfaceFormat) -> AzSourceSurfaceRef;

pub fn AzDrawTargetCreateSimilarDrawTarget(aDrawTarget: AzDrawTargetRef,
                                           aSize: *const AzIntSize,
                                           aFormat: AzSurfaceFormat)
                                           -> AzSourceSurfaceRef;

pub fn AzDrawTargetCreateShadowDrawTarget(aDrawTarget: AzDrawTargetRef,
                                          aSize: *const AzIntSize,
                                          aFormat: AzSurfaceFormat,
                                          aSigma: AzFloat)
                                          -> AzSourceSurfaceRef;

pub fn AzDrawTargetCreateGradientStops(aDrawTarget: AzDrawTargetRef,
                                       aStops: *const AzGradientStop,
                                       aNumStops: u32,
                                       aExtendMode: AzExtendMode)
                                       -> AzGradientStopsRef;

pub fn AzDrawTargetCreateFilter(aDrawTarget: AzDrawTargetRef, aFilterType: AzFilterType)
                                -> AzFilterNodeRef;

pub fn AzCreateLinearGradientPattern(aBegin: *const AzPoint,
                                     aEnd: *const AzPoint,
                                     aStops: AzGradientStopsRef,
                                     aMatrix: *const AzMatrix)
                                     -> AzLinearGradientPatternRef;

pub fn AzCloneLinearGradientPattern(aPattern: AzLinearGradientPatternRef)
                                    -> AzLinearGradientPatternRef;

pub fn AzCreateRadialGradientPattern(aCenter1: *const AzPoint,
                                     aCenter2: *const AzPoint,
                                     aRadius1: AzFloat,
                                     aRadius2: AzFloat,
                                     aStops: AzGradientStopsRef,
                                     aMatrix: *const AzMatrix)
                                     -> AzRadialGradientPatternRef;

pub fn AzCloneRadialGradientPattern(aPattern: AzRadialGradientPatternRef)
                                    -> AzRadialGradientPatternRef;

pub fn AzCreateSurfacePattern(aSurface: AzSourceSurfaceRef, aExtendMode: AzExtendMode)
                              -> AzSurfacePatternRef;

pub fn AzCloneSurfacePattern(aPattern: AzSurfacePatternRef)
                             -> AzSurfacePatternRef;

pub fn AzSurfacePatternGetSize(aPattern: AzSurfacePatternRef) -> AzIntSize;

pub fn AzReleaseSourceSurface(aSurface: AzSourceSurfaceRef);

pub fn AzSourceSurfaceGetSize(aSurface: AzSourceSurfaceRef) -> AzIntSize;

pub fn AzSourceSurfaceGetFormat(aSurface: AzSourceSurfaceRef) -> AzSurfaceFormat;

pub fn AzSourceSurfaceGetDataSurface(aSurface: AzSourceSurfaceRef) -> AzDataSourceSurfaceRef;

pub fn AzDataSourceSurfaceGetData(aSurface: AzDataSourceSurfaceRef) -> *mut u8;

pub fn AzDataSourceSurfaceGetStride(aSurface: AzDataSourceSurfaceRef) -> i32;

pub fn AzCreateScaledFontForNativeFont(aNativeFont: *mut AzNativeFont, aSize: AzFloat) -> AzScaledFontRef;

pub fn AzCreateScaledFontForTrueTypeData(aFontData: *const u8, aFontDataSize: u32, aFaceIndex: u32, aGlyphSize: f32, aType: enum_AzFontType) -> AzScaledFontRef;

pub fn AzReleaseScaledFont(aFont: AzScaledFontRef);

pub fn AzDrawTargetSetTransform(aDrawTarget: AzDrawTargetRef, aTransform: *mut AzMatrix);

pub fn AzCreateFontOptionsForName(aName: *const c_char, aStyle: enum_AzFontStyle) -> *mut AzFontOptions;

pub fn AzDestroyFontOptions(aOptions: *mut AzFontOptions);

pub fn AzCreatePathBuilder(aDrawTarget: AzDrawTargetRef) -> AzPathBuilderRef;

pub fn AzReleasePathBuilder(aPathBuilder: AzPathBuilderRef);

pub fn AzPathBuilderMoveTo(aPathBuilder: AzPathBuilderRef, aPoint: *mut AzPoint);

pub fn AzPathBuilderLineTo(aPathBuilder: AzPathBuilderRef, aPoint: *mut AzPoint);

pub fn AzPathBuilderArc(aPathBuilder: AzPathBuilderRef,
                        aOrigin: *const AzPoint,
                        aRadius: AzFloat,
                        aStartAngle: AzFloat,
                        aEndAngle: AzFloat,
                        aAntiClockwise: bool);

pub fn AzPathBuilderBezierTo(aPathBuilder: AzPathBuilderRef,
                             aControlPoint1: *const AzPoint,
                             aControlPoint2: *const AzPoint,
                             aControlPoint3: *const AzPoint);

pub fn AzPathBuilderQuadraticBezierTo(aPathBuilder: AzPathBuilderRef,
                                      aControlPoint: *const AzPoint,
                                      aEndPoint: *const AzPoint);

pub fn AzPathBuilderCurrentPoint(aPathBuilder: AzPathBuilderRef) -> AzPoint;

pub fn AzPathBuilderClose(aPathBuilder: AzPathBuilderRef);

pub fn AzPathBuilderFinish(aPathBuilder: AzPathBuilderRef) -> AzPathRef;

pub fn AzReleasePath(aPath: AzPathRef);

pub fn AzReleaseGradientStops(aFont: AzScaledFontRef);

pub fn AzReleaseFilterNode(aFilter: AzFilterNodeRef);
pub fn AzFilterNodeSetSourceSurfaceInput(aFilter: AzFilterNodeRef,
                                         aIndex: u32,
                                         aSurface: AzSourceSurfaceRef);
pub fn AzFilterNodeSetFilterNodeInput(aFilter: AzFilterNodeRef,
                                      aIndex: u32,
                                      aInputFilter: AzFilterNodeRef);
pub fn AzFilterNodeSetUintAttribute(aFilter: AzFilterNodeRef, aIndex: u32, aValue: u32);
pub fn AzFilterNodeSetFloatAttribute(aFilter: AzFilterNodeRef, aIndex: u32, aValue: AzFloat);
pub fn AzFilterNodeSetFloatArrayAttribute(aFilter: AzFilterNodeRef,
                                          aIndex: u32,
                                          aFloats: *const AzFloat,
                                          aSize: u32);
pub fn AzFilterNodeSetColorAttribute(aFilter: AzFilterNodeRef,
                                     aIndex: u32,
                                     aValue: *const AzColor);
pub fn AzFilterNodeSetMatrixAttribute(aFilter: AzFilterNodeRef,
                                         aIndex: u32,
                                         aValue: *const AzMatrix);
pub fn AzFilterNodeSetMatrix5x4Attribute(aFilter: AzFilterNodeRef,
                                         aIndex: u32,
                                         aValue: *const AzMatrix5x4);

pub fn AzFilterNodeSetBoolAttribute(aFilter: AzFilterNodeRef, aIndex: u32, aValue: bool);

}
