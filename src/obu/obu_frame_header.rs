use crate::{bits::bitstream::BitStream, State};

use super::{
    obu_header::ObuHeader,
    obu_sequence_header::{ObuSequenceHeader, SELECT_INTEGER_MV, SELECT_SCREEN_CONTENT_TOOLS},
};

pub struct ObuFrameHeader {
    uncompressed_header: UncompressedHeader,
}

impl ObuFrameHeader {
    pub fn new(
        b: &mut BitStream,
        state: &mut State,
        sh: ObuSequenceHeader,
        h: ObuHeader,
        // Taken from previous uncompressed_header
        old_frame_id: Option<u64>,
    ) -> ObuFrameHeader {
        let uh: UncompressedHeader;
        if state.seen_frame_header {
            todo!("frame_header_copy()");
        } else {
            state.seen_frame_header = true;
            uh = UncompressedHeader::new(b, sh, h, state, old_frame_id);

            if uh.show_existing_frame {
                todo!("decode_frame_wrapup()");
                state.seen_frame_header = false;
            } else {
                state.tile_num = false;
                state.seen_frame_header = true;
            }
        }

        ObuFrameHeader {
            uncompressed_header: uh,
        }
    }
}

#[derive(Default)]
pub struct UncompressedHeader {
    show_existing_frame: bool,
    frame_type: u64,
    show_frame: bool,
    showable_frame: bool,
    frame_to_show_map_idx: usize,
    refresh_frame_flags: u64,
    display_frame_id: u64,
    error_resilient_mode: bool,
    disable_cdf_update: bool,
    allow_screen_content_tools: bool,
    force_integer_mv: u64,
    current_frame_id: u64,
    frame_size_override_flag: bool,
    primary_ref_frame: u64,
    buffer_removal_time: Vec<u64>,
}

impl UncompressedHeader {
    pub fn new(
        b: &mut BitStream,
        sh: ObuSequenceHeader,
        header: ObuHeader,
        state: &mut State,
        old_frame_id: Option<u64>,
    ) -> UncompressedHeader {
        let mut uh = UncompressedHeader::default();

        let id_len = match sh.frame_id_numbers_present_flag {
            true => sh.additional_frame_id_length_minus_1 + sh.delta_frame_id_length_minus_2 + 3,
            false => 0,
        };

        let all_frames = (1 << NUM_REF_FRAMES) - 1;

        if sh.reduced_still_picture_header {
            uh.show_existing_frame = false;
            uh.frame_type = KEY_FRAME;

            state.frame_is_intra = true;

            uh.show_frame = true;
            uh.showable_frame = false;
        } else {
            uh.show_existing_frame = b.f(1) != 0;

            if uh.show_existing_frame {
                uh.frame_to_show_map_idx = b.f(3) as usize;

                if sh.decoder_model_info_present_flag
                    && !sh.timing_info.unwrap().equal_picture_interval
                {
                    todo!("temporal_point_info()");
                }

                uh.refresh_frame_flags = 0;
                if sh.frame_id_numbers_present_flag {
                    uh.display_frame_id = b.f(id_len);
                }

                uh.frame_type = state.ref_frame_type[uh.frame_to_show_map_idx];

                if uh.frame_type == KEY_FRAME {
                    uh.refresh_frame_flags = all_frames;
                }

                if sh.film_grain_params_present {
                    todo!("load_grain_params( frame_to_show_map_idx )");
                }

                return uh;
            }

            uh.frame_type = b.f(2);

            state.frame_is_intra = uh.frame_type == INTRA_ONLY_FRAME || uh.frame_type == KEY_FRAME;
            uh.show_frame = b.f(1) != 0;

            if uh.show_frame
                && sh.decoder_model_info_present_flag
                && !sh.timing_info.unwrap().equal_picture_interval
            {
                todo!("temporal_point_info()");
            }

            if uh.show_frame {
                uh.showable_frame = uh.frame_type != KEY_FRAME;
            } else {
                uh.showable_frame = b.f(1) != 0;
            }

            if uh.frame_type == SWITCH_FRAME || uh.frame_type == KEY_FRAME && uh.show_frame {
                uh.error_resilient_mode = true;
            } else {
                uh.error_resilient_mode = b.f(1) != 0;
            }
        }

        if uh.frame_type == KEY_FRAME && uh.show_frame {
            for i in 0..NUM_REF_FRAMES {
                state.ref_valid.insert(i, 0);
                state.ref_order_hint.insert(i, 0);
            }

            for i in 0..REFS_PER_FRAME {
                state.order_hints.insert(LAST_FRAME + i, 0);
            }
        }

        uh.disable_cdf_update = b.f(1) != 0;

        if sh.seq_force_screen_content_tools == SELECT_SCREEN_CONTENT_TOOLS {
            uh.allow_screen_content_tools = b.f(1) != 0;
        } else {
            uh.allow_screen_content_tools = sh.seq_choose_screen_content_tools;
        }

        if uh.allow_screen_content_tools {
            if sh.seq_force_integer_mv == SELECT_INTEGER_MV {
                uh.force_integer_mv = b.f(1);
            } else {
                uh.force_integer_mv = sh.seq_force_integer_mv;
            }
        } else {
            uh.force_integer_mv = 0;
        }

        if state.frame_is_intra {
            uh.force_integer_mv = 1;
        }

        if sh.frame_id_numbers_present_flag {
            state.prev_frame_id = old_frame_id.unwrap_or(0);
            uh.current_frame_id = b.f(id_len);
            todo!("mark_ref_frames( idLen )");
        } else {
            uh.current_frame_id = 0;
        }

        if uh.frame_type == SWITCH_FRAME {
            uh.frame_size_override_flag = true;
        } else if sh.reduced_still_picture_header {
            uh.frame_size_override_flag = false;
        } else {
            uh.frame_size_override_flag = b.f(1) != 0;
        }

        state.order_hint = b.f(state.order_hint_bits);

        if state.frame_is_intra || uh.error_resilient_mode {
            uh.primary_ref_frame = PRIMARY_REF_NONE;
        } else {
            uh.primary_ref_frame = b.f(3);
        }

        if sh.decoder_model_info_present_flag {
            let buffer_removal_time_present_flag = b.f(1) != 0;
            if buffer_removal_time_present_flag {
                for op_num in 0..=sh.operating_points_cnt_minus_1 {
                    if sh.decoder_model_present_for_this_op[op_num] {
                        let op_pt_idc = sh.operating_point_idc[op_num];
                        let in_temporal_layer = ((op_pt_idc
                            >> header.obu_extension_header.clone().unwrap().temporal_id)
                            & 1)
                            != 0;
                        let in_spatial_layer = ((op_pt_idc
                            >> header.obu_extension_header.clone().unwrap().spatial_id + 8)
                            & 1)
                            != 0;

                        if op_pt_idc == 0 || (in_temporal_layer && in_spatial_layer) {
                            uh.buffer_removal_time.insert(
                                op_num,
                                b.f(sh
                                    .decoder_model_info
                                    .clone()
                                    .unwrap()
                                    .buffer_removal_time_length_minus_1
                                    + 1),
                            );
                        }
                    }
                }
            }
        }

        uh
    }
}

const NUM_REF_FRAMES: usize = 8;
const KEY_FRAME: u64 = 0;
const INTER_FRAME: u64 = 1;
const INTRA_ONLY_FRAME: u64 = 2;
const SWITCH_FRAME: u64 = 3;

const REFS_PER_FRAME: usize = 7;

const LAST_FRAME: usize = 1;

const PRIMARY_REF_NONE: u64 = 7;
