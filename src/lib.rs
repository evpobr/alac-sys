#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const ALAC_noErr: i32 = 0;

#[repr(i32)]
pub enum ELEMENT_TYPE {
    ID_SCE = 0,
    ID_CPE = 1,
    ID_CCE = 2,
    ID_LFE = 3,
    ID_DSE = 4,
    ID_PCE = 5,
    ID_FIL = 6,
    ID_END = 7,
}

#[repr(C)]
#[derive(Debug)]
pub struct BitBuffer {
    pub cur: *mut u8,
    pub end: *mut u8,
    pub bitIndex: u32,
    pub byteSize: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct AGParamRec {
    pub mb: u32,
    pub mb0: u32,
    pub pb: u32,
    pub kb: u32,
    pub wb: u32,
    pub qb: u32,
    pub fw: u32,
    pub sw: u32,

    pub maxrun: u32,
}

pub type AGParamRecPtr = *mut AGParamRec;

pub const kChannelAtomSize: usize = 12;

pub const kALAC_UnimplementedError: i32 = -4;
pub const kALAC_FileNotFoundError: i32 = -43;
pub const kALAC_ParamError: i32 = -50;
pub const kALAC_MemFullError: i32 = -108;

pub static kALACFormatAppleLossless: &[u8; 4] = b"alac";
pub static kALACFormatLinearPCM: &[u8; 4] = b"lpcm";

pub const kALACMaxChannels: u32 = 8;
pub const kALACMaxEscapeHeaderBytes: u32 = 8;
pub const kALACMaxSearches: u32 = 16;
pub const kALACMaxCoefs: u32 = 16;
pub const kALACDefaultFramesPerPacket: u32 = 4096;

pub type ALACChannelLayoutTag = u32;

pub const kALACFormatFlagIsFloat: u32 = 1 << 0;
pub const kALACFormatFlagIsBigEndian: u32 = 1 << 1;
pub const kALACFormatFlagIsSignedInteger: u32 = 1 << 2;
pub const kALACFormatFlagIsPacked: u32 = 1 << 3;
pub const kALACFormatFlagIsAlignedHigh: u32 = 1 << 4;

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
pub const kALACFormatFlagsNativeEndian: u32 =
    if cfg!(any(target_arch = "powerpc", target_arch = "powerpc64")) {
        kALACFormatFlagIsBigEndian
    } else {
        0
    };

pub type alac_float64_t = f64;

pub const kALACChannelLayoutTag_Mono: u32 = (100 << 16) | 1;
pub const kALACChannelLayoutTag_Stereo: u32 = (101 << 16) | 2;
pub const kALACChannelLayoutTag_MPEG_3_0_B: u32 = (113 << 16) | 3;
pub const kALACChannelLayoutTag_MPEG_4_0_B: u32 = (116 << 16) | 4;
pub const kALACChannelLayoutTag_MPEG_5_0_D: u32 = (120 << 16) | 5;
pub const kALACChannelLayoutTag_MPEG_5_1_D: u32 = (124 << 16) | 6;
pub const kALACChannelLayoutTag_AAC_6_1: u32 = (142 << 16) | 7;
pub const kALACChannelLayoutTag_MPEG_7_1_B: u32 = (127 << 16) | 8;

pub static ALACChannelLayoutTags: [ALACChannelLayoutTag; kALACMaxChannels as usize] = [
    kALACChannelLayoutTag_Mono,
    kALACChannelLayoutTag_Stereo,
    kALACChannelLayoutTag_MPEG_3_0_B,
    kALACChannelLayoutTag_MPEG_4_0_B,
    kALACChannelLayoutTag_MPEG_5_0_D,
    kALACChannelLayoutTag_MPEG_5_1_D,
    kALACChannelLayoutTag_AAC_6_1,
    kALACChannelLayoutTag_MPEG_7_1_B,
];

#[repr(C)]
#[derive(Debug)]
pub struct ALACAudioChannelLayout {
    pub mChannelLayoutTag: ALACChannelLayoutTag,
    pub mChannelBitmap: u32,
    pub mNumberChannelDescriptions: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct AudioFormatDescription {
    pub mSampleRate: alac_float64_t,
    pub mFormatID: u32,
    pub mFormatFlags: u32,
    pub mBytesPerPacket: u32,
    pub mFramesPerPacket: u32,
    pub mBytesPerFrame: u32,
    pub mChannelsPerFrame: u32,
    pub mBitsPerChannel: u32,
    pub mReserved: u32,
}

pub static kALACCodecFormat: &[u8; 4] = b"alac";
pub const kALACVersion: u32 = 0;
pub const kALACCompatibleVersion: u32 = kALACVersion;
pub const kALACDefaultFrameSize: u32 = 4096;

#[repr(C)]
#[derive(Debug)]
pub struct ALACSpecificConfig {
    pub frameLength: u32,
    pub compatibleVersion: u8,
    pub bitDepth: u8,
    pub pb: u8,
    pub mb: u8,
    pub kb: u8,
    pub numChannels: u8,
    pub maxRun: u16,
    pub maxFrameBytes: u32,
    pub avgBitRate: u32,
    pub sampleRate: u32,
}

pub static AudioChannelLayoutAID: &[u8; 4] = b"chan";

pub const DENSHIFT_MAX: u32 = 15;
pub const DENSHIFT_DEFAULT: u32 = 9;
pub const AINIT: i32 = 38;
pub const BINIT: i32 = -29;
pub const CINIT: i32 = -2;
pub const NUMCOEPAIRS: i32 = 16;

extern "C" {
    pub fn BitBufferInit(bits: *mut BitBuffer, buffer: *mut u8, byteSize: u32);
    pub fn BitBufferRead(bits: *mut BitBuffer, numBits: u8) -> u32;
    pub fn BitBufferReadSmall(bits: *mut BitBuffer, numBits: u8) -> u8;
    pub fn BitBufferReadOne(bits: *mut BitBuffer) -> u8;
    pub fn BitBufferPeek(bits: *mut BitBuffer, numBits: u8) -> u8;
    pub fn BitBufferPeekOne(bits: *mut BitBuffer) -> u32;
    pub fn BitBufferUnpackBERSize(bits: *mut BitBuffer) -> u32;
    pub fn BitBufferGetPosition(bits: *mut BitBuffer) -> u32;
    pub fn BitBufferByteAlign(bits: *mut BitBuffer, addZeros: i32);
    pub fn BitBufferAdvance(bits: *mut BitBuffer, numBits: u32);
    pub fn BitBufferRewind(bits: *mut BitBuffer, numBits: u32);
    pub fn BitBufferWrite(bits: *mut BitBuffer, value: u32, numBits: u32);
    pub fn BitBufferReset(bits: *mut BitBuffer);

    pub fn set_standard_ag_params(params: AGParamRecPtr, fullwidth: u32, sectorwidth: u32);
    pub fn set_ag_params(
        params: AGParamRecPtr,
        m: u32,
        p: u32,
        k: u32,
        f: u32,
        s: u32,
        maxrun: u32,
    );
    pub fn dyn_comp(
        params: AGParamRecPtr,
        pc: *mut i32,
        bitstream: *mut BitBuffer,
        numSamples: i32,
        bitSize: i32,
        outNumBits: *mut u32,
    ) -> i32;
    pub fn dyn_decomp(
        params: AGParamRecPtr,
        bitstream: *mut BitBuffer,
        pc: *mut i32,
        numSamples: i32,
        maxSize: i32,
        outNumBits: *mut u32,
    ) -> i32;

    pub fn init_coefs(coefs: *mut i16, denshift: u32, numPairs: i32);
    pub fn copy_coefs(srcCoefs: *mut i16, dstCoefs: *mut i16, numPairs: i32);
    pub fn pc_block(
        r#in: *mut i32,
        pc: *mut i32,
        num: i32,
        coefs: *mut i16,
        numactive: i32,
        chanbits: u32,
        denshift: u32,
    );
    pub fn unpc_block(
        pc: *mut i32,
        out: *mut i32,
        num: i32,
        coefs: *mut i16,
        numactive: i32,
        chanbits: u32,
        denshift: u32,
    );

    pub fn Swap16NtoB(inUInt16: u16) -> u16;
    pub fn Swap16BtoN(inUInt16: u16) -> u16;
    pub fn Swap32NtoB(inUInt32: u32) -> u32;
    pub fn Swap32BtoN(inUInt32: u32) -> u32;
    pub fn Swap64BtoN(inUInt64: u64) -> u64;
    pub fn Swap64NtoB(inUInt64: u64) -> u64;
    pub fn SwapFloat32BtoN(r#in: f32) -> f32;
    pub fn SwapFloat32NtoB(r#in: f32) -> f32;
    pub fn SwapFloat64BtoN(r#in: f64) -> f64;
    pub fn SwapFloat64NtoB(r#in: f64) -> f64;
    pub fn Swap16(inUInt16: *mut u16);
    pub fn Swap24(inUInt24: *mut u8);
    pub fn Swap32(inUInt32: *mut u32);

    pub fn mix16(
        r#in: *mut i16,
        stride: u32,
        u: *mut i32,
        v: *mut i32,
        numSamples: i32,
        mixbits: i32,
        mixres: i32,
    );
    pub fn unmix16(
        u: *mut i32,
        v: *mut i32,
        out: *mut i16,
        stride: u32,
        numSamples: i32,
        mixbits: i32,
        mixres: i32,
    );
    pub fn mix20(
        r#in: *mut u8,
        stride: u32,
        u: *mut i32,
        v: *mut i32,
        numSamples: i32,
        mixbits: i32,
        mixres: i32,
    );
    pub fn unmix20(
        u: *mut i32,
        v: *mut i32,
        out: *mut u8,
        stride: u32,
        numSamples: i32,
        mixbits: i32,
        mixres: i32,
    );
    pub fn mix24(
        r#in: *mut u8,
        stride: u32,
        u: *mut i32,
        v: *mut i32,
        numSamples: i32,
        mixbits: i32,
        mixres: i32,
        shiftUV: *mut u16,
        bytesShifted: i32,
    );
    pub fn unmix24(
        u: *mut i32,
        v: *mut i32,
        out: *mut u8,
        stride: u32,
        numSamples: i32,
        mixbits: i32,
        mixres: i32,
        shiftUV: *mut u16,
        bytesShifted: i32,
    );
    pub fn mix32(
        r#in: *mut i32,
        stride: u32,
        u: *mut i32,
        v: *mut i32,
        numSamples: i32,
        mixbits: i32,
        mixres: i32,
        shiftUV: *mut u16,
        bytesShifted: i32,
    );
    pub fn unmix32(
        u: *mut i32,
        v: *mut i32,
        out: *mut i32,
        stride: u32,
        numSamples: i32,
        mixbits: i32,
        mixres: i32,
        shiftUV: *mut u16,
        bytesShifted: i32,
    );
    pub fn copy20ToPredictor(r#in: *mut u8, stride: u32, out: *mut i32, numSamples: i32);
    pub fn copy24ToPredictor(r#in: *mut u8, stride: u32, out: *mut i32, numSamples: i32);
    pub fn copyPredictorTo24(r#in: i32, out: *mut u8, stride: u32, numSamples: i32);
    pub fn copyPredictorTo24Shift(
        r#in: *mut i32,
        shift: *mut u16,
        out: *mut u8,
        stride: u32,
        numSamples: i32,
        bytesShifted: i32,
    );
    pub fn copyPredictorTo20(r#in: i32, out: *mut u8, stride: u32, numSamples: i32);

    pub fn copyPredictorTo32(r#in: *mut i32, out: *mut i32, stride: u32, numSamples: i32);
    pub fn copyPredictorTo32Shift(
        r#in: *mut i32,
        shift: *mut u16,
        out: *mut i32,
        stride: u32,
        numSamples: i32,
        bytesShifted: i32,
    );
}
