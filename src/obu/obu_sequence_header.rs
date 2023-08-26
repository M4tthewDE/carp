use crate::bits::bitstream::BitStream;

use super::{
    color_config::ColorConfig, decoder_model_info::DecoderModelInfo,
    operating_parameters_info::OperatingParamtersInfo, timing_info::TimingInfo,
};

#[derive(Default)]
pub struct ObuSequenceHeader {
    timing_info_present_flag: bool,
    decoder_model_info_present_flag: bool,
    initial_display_delay_present_flag: bool,
    initial_display_delay_minus_1: Vec<u64>,
    operating_points_cnt_minus_1: usize,
    operating_point_idc: Vec<u64>,
    seq_level_idx: Vec<u64>,
    seq_tier: Vec<u64>,
    decoder_model_present_for_this_op: Vec<bool>,
    initial_display_delay_present_for_this_op: Vec<bool>,

    timing_info: Option<TimingInfo>,
    decoder_model_info: Option<DecoderModelInfo>,
    operating_parameters_info: Option<OperatingParamtersInfo>,

    frame_width_bits_minus_1: u64,
    frame_height_bits_minus_1: u64,
    max_frame_width_minus_1: u64,
    max_frame_height_minus_1: u64,
    frame_id_numbers_present_flag: bool,
    delta_frame_id_length_minus_2: u64,
    additional_frame_id_length_minus_1: u64,

    use_128x128_superblock: bool,
    enable_filter_intra: bool,
    enable_intra_edge_filter: bool,

    enable_interintra_compound: bool,
    enable_masked_compound: bool,
    enable_warped_motion: bool,
    enable_dual_filter: bool,
    enable_order_hint: bool,
    enable_jnt_comp: bool,
    enable_ref_frame_mvs: bool,
    seq_force_screen_content_tools: u64,
    seq_force_integer_mv: u64,
    seq_choose_screen_content_tools: bool,
    seq_choose_integer_mv: bool,

    enable_superres: bool,
    enable_cdef: bool,
    enable_restoration: bool,
    film_grain_params_present: bool,

    color_config: ColorConfig,
}

impl ObuSequenceHeader {
    pub fn new(
        b: &mut BitStream,
        operating_point_idc: &mut u64,
        order_hint_bits: &mut u64,
        bit_depth: &mut u64,
        num_planes: &mut u64,
    ) -> ObuSequenceHeader {
        let mut osh = ObuSequenceHeader::default();
        let seq_profile = b.f(3);
        let still_picture = b.f(1);
        let reduced_still_picture_header = b.f(1) != 0;

        if reduced_still_picture_header {
            osh.operating_point_idc.push(0);
            osh.seq_level_idx.push(0);
            osh.seq_tier.push(0);
            osh.decoder_model_present_for_this_op.push(false);
            osh.initial_display_delay_present_for_this_op.push(false);
        } else {
            osh.timing_info_present_flag = b.f(1) != 0;

            if osh.timing_info_present_flag {
                osh.timing_info = Some(TimingInfo::new(b));
                osh.decoder_model_info_present_flag = b.f(1) != 0;

                if osh.decoder_model_info_present_flag {
                    osh.decoder_model_info = Some(DecoderModelInfo::new(b));
                }
            } else {
                osh.decoder_model_info_present_flag = false;
            }

            osh.initial_display_delay_present_flag = b.f(1) != 0;
            osh.operating_points_cnt_minus_1 = b.f(5) as usize;

            // these insertions are wrong if values are supposed to be overwritten
            for i in 0..=osh.operating_points_cnt_minus_1 {
                osh.operating_point_idc.insert(i, b.f(12));
                osh.seq_level_idx.insert(i, b.f(5));

                if *osh.seq_level_idx.get(i).unwrap() > 7 {
                    osh.seq_tier.insert(i, b.f(1));
                }

                if osh.decoder_model_info_present_flag {
                    osh.decoder_model_present_for_this_op.insert(i, b.f(1) != 0);

                    if *osh.decoder_model_present_for_this_op.get(1).unwrap() {
                        osh.operating_parameters_info = Some(OperatingParamtersInfo::new(
                            b,
                            i,
                            osh.decoder_model_info
                                .clone()
                                .unwrap()
                                .buffer_delay_length_minus_1,
                        ));
                    }
                } else {
                    osh.decoder_model_present_for_this_op.insert(i, false);
                }

                if osh.initial_display_delay_present_flag {
                    osh.initial_display_delay_present_for_this_op
                        .insert(i, b.f(1) != 0);
                    if *osh
                        .initial_display_delay_present_for_this_op
                        .get(i)
                        .unwrap()
                    {
                        osh.initial_display_delay_minus_1.insert(i, b.f(4));
                    }
                }
            }
        }

        let operating_point = ObuSequenceHeader::choose_operating_point();

        *operating_point_idc = *osh.operating_point_idc.get(operating_point).unwrap();

        osh.frame_width_bits_minus_1 = b.f(4);
        osh.frame_height_bits_minus_1 = b.f(4);

        osh.max_frame_width_minus_1 = b.f(osh.frame_width_bits_minus_1 + 1);
        osh.max_frame_height_minus_1 = b.f(osh.frame_height_bits_minus_1 + 1);

        if reduced_still_picture_header {
            osh.frame_id_numbers_present_flag = false;
        } else {
            osh.frame_id_numbers_present_flag = b.f(1) != 0;
        }

        if osh.frame_id_numbers_present_flag {
            osh.delta_frame_id_length_minus_2 = b.f(4);
            osh.additional_frame_id_length_minus_1 = b.f(3);
        }

        osh.use_128x128_superblock = b.f(1) != 0;
        osh.enable_filter_intra = b.f(1) != 0;
        osh.enable_intra_edge_filter = b.f(1) != 0;

        if reduced_still_picture_header {
            osh.seq_force_screen_content_tools = SELECT_SCREEN_CONTENT_TOOLS;
            osh.seq_force_integer_mv = SELECT_INTEGER_MV;
            *order_hint_bits = 0;
        } else {
            osh.enable_interintra_compound = b.f(1) != 0;
            osh.enable_masked_compound = b.f(1) != 0;
            osh.enable_warped_motion = b.f(1) != 0;
            osh.enable_dual_filter = b.f(1) != 0;
            osh.enable_order_hint = b.f(1) != 0;

            if osh.enable_order_hint {
                osh.enable_jnt_comp = b.f(1) != 0;
                osh.enable_ref_frame_mvs = b.f(1) != 0;
            }
            osh.seq_choose_screen_content_tools = b.f(1) != 0;
            if osh.seq_choose_screen_content_tools {
                osh.seq_force_screen_content_tools = SELECT_SCREEN_CONTENT_TOOLS;
            } else {
                osh.seq_force_screen_content_tools = b.f(1);
            }

            if osh.seq_force_screen_content_tools > 0 {
                osh.seq_choose_integer_mv = b.f(1) != 0;

                if osh.seq_choose_integer_mv {
                    osh.seq_force_integer_mv = SELECT_INTEGER_MV;
                } else {
                    osh.seq_force_integer_mv = b.f(1);
                }
            }

            if osh.enable_order_hint {
                let order_hint_bits_minus_1 = b.f(3);
                *order_hint_bits = order_hint_bits_minus_1 + 1;
            } else {
                *order_hint_bits = 0;
            }
        }

        osh.enable_superres = b.f(1) != 0;
        osh.enable_cdef = b.f(1) != 0;
        osh.enable_restoration = b.f(1) != 0;
        osh.film_grain_params_present = b.f(1) != 0;

        osh.color_config = ColorConfig::new(b, seq_profile, bit_depth, num_planes);

        osh
    }

    fn choose_operating_point() -> usize {
        todo!("not implemented");
    }
}

const SELECT_SCREEN_CONTENT_TOOLS: u64 = 2;
const SELECT_INTEGER_MV: u64 = 2;
