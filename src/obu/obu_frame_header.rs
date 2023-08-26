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
            uh = UncompressedHeader::new(sh);

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
}

impl UncompressedHeader {
    pub fn new(sh: ObuSequenceHeader) -> UncompressedHeader {
        let mut uh = UncompressedHeader::default();

        let id_len: u64;
        if sh.frame_id_numbers_present_flag {
            id_len = sh.additional_frame_id_length_minus_1 + sh.delta_frame_id_length_minus_2 + 3;
        }

        let all_frames = (1 << NUM_REF_FRAMES) - 1;

        if sh.reduced_still_picture_header {
            uh.show_existing_frame = false;
        }

        todo!("implement");

        uh
    }
}

const NUM_REF_FRAMES: u64 = 8;
