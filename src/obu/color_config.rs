use crate::bits::bitstream::BitStream;

#[derive(Default)]
pub struct ColorConfig {
    high_bitdepth: bool,
    twelve_bit: bool,
    mono_chrome: bool,
    color_primaries: u64,
    transfer_characteristics: u64,
    matrix_coefficients: u64,
    color_range: bool,
    subsampling_x: bool,
    subsampling_y: bool,
    chroma_sample_position: u64,
    separate_uv_delta_q: bool,
}

impl ColorConfig {
    pub fn new(
        b: &mut BitStream,
        seq_profile: u64,
        bit_depth: &mut u64,
        num_planes: &mut u64,
    ) -> ColorConfig {
        let mut cc = ColorConfig::default();

        cc.high_bitdepth = b.f(1) != 0;

        if seq_profile == 2 && cc.high_bitdepth {
            cc.twelve_bit = b.f(1) != 0;
            if cc.twelve_bit {
                *bit_depth = 12;
            } else {
                *bit_depth = 10;
            }
        } else if seq_profile <= 2 {
            if cc.high_bitdepth {
                *bit_depth = 10;
            } else {
                *bit_depth = 8;
            }
        }

        if seq_profile == 1 {
            cc.mono_chrome = false;
        } else {
            cc.mono_chrome = b.f(1) != 0;
        }

        if cc.mono_chrome {
            *num_planes = 1;
        } else {
            *num_planes = 3;
        }

        let color_description_present_flag = b.f(1) != 0;
        if color_description_present_flag {
            cc.color_primaries = b.f(8);
            cc.transfer_characteristics = b.f(8);
            cc.matrix_coefficients = b.f(8);
        } else {
            cc.color_primaries = CP_UNSPECIFIED;
            cc.transfer_characteristics = TC_UNSPECIFIED;
            cc.matrix_coefficients = MC_UNSPECIFIED;
        }

        if cc.mono_chrome {
            cc.color_range = b.f(1) != 0;
            cc.subsampling_x = true;
            cc.subsampling_y = true;
            cc.chroma_sample_position = CSP_UNKNOWN;
            cc.separate_uv_delta_q = false;

            return cc;
        } else if cc.color_primaries == CP_BT_709
            && cc.transfer_characteristics == TC_SRGB
            && cc.matrix_coefficients == MC_IDENTITY
        {
            cc.color_range = true;
            cc.subsampling_x = false;
            cc.subsampling_y = false;
        } else {
            cc.color_range = b.f(1) != 0;
            if seq_profile == 0 {
                cc.subsampling_x = true;
                cc.subsampling_y = true;
            } else if seq_profile == 1 {
                cc.subsampling_x = false;
                cc.subsampling_y = false;
            } else {
                if *bit_depth == 12 {
                    cc.subsampling_x = b.f(1) != 0;
                    if cc.subsampling_x {
                        cc.subsampling_y = b.f(1) != 0;
                    } else {
                        cc.subsampling_y = false;
                    }
                } else {
                    cc.subsampling_x = true;
                    cc.subsampling_y = false;
                }
            }

            if cc.subsampling_x && cc.subsampling_y {
                cc.chroma_sample_position = b.f(2);
            }
        }

        cc.separate_uv_delta_q = b.f(1) != 0;

        cc
    }
}

const CP_BT_709: u64 = 1;
const CP_UNSPECIFIED: u64 = 2;

const TC_UNSPECIFIED: u64 = 2;
const TC_SRGB: u64 = 13;

const MC_IDENTITY: u64 = 0;
const MC_UNSPECIFIED: u64 = 2;

const CSP_UNKNOWN: u64 = 0;
