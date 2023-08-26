use crate::{bits::bitstream::BitStream, State};

use super::obu_sequence_header::ObuSequenceHeader;

pub struct ObuFrameHeader {
    uncompressed_header: UncompressedHeader,
}

impl ObuFrameHeader {
    pub fn new(b: &mut BitStream, state: &mut State, sh: ObuSequenceHeader) -> ObuFrameHeader {
        let uh: UncompressedHeader;
        if state.seen_frame_header {
            todo!("frame_header_copy()");
        } else {
            state.seen_frame_header = true;
            uh = UncompressedHeader::new(b, sh, state);

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
}

impl UncompressedHeader {
    pub fn new(b: &mut BitStream, sh: ObuSequenceHeader, state: &mut State) -> UncompressedHeader {
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
        }

        uh
    }
}

const NUM_REF_FRAMES: u64 = 8;
const KEY_FRAME: u64 = 0;
