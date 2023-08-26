use crate::bits::bitstream::BitStream;

use super::{
    decoder_model_info::DecoderModelInfo, operating_parameters_info::OperatingParamtersInfo,
    timing_info::TimingInfo,
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
}

impl ObuSequenceHeader {
    pub fn new(b: &mut BitStream) -> ObuSequenceHeader {
        let mut osh = ObuSequenceHeader::default();
        let seq_profile = b.f(3);
        let still_picture = b.f(1);
        let reduced_still_picture_header = b.f(1);

        if reduced_still_picture_header != 0 {
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

        osh
    }

    fn choose_operating_point() -> u64 {
        todo!("not implemented");
    }
}
