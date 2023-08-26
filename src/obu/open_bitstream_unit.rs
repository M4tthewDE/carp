use crate::{bits::bitstream::BitStream, State};

use super::{
    obu_header::{ObuHeader, ObuType},
    obu_sequence_header::ObuSequenceHeader,
};

pub struct OpenBitstreamUnit {}

impl OpenBitstreamUnit {
    pub fn new(bitstream: &mut BitStream, sz: u64, state: &mut State) -> Option<OpenBitstreamUnit> {
        let header = ObuHeader::new(bitstream);

        let obu_size = match header.obu_has_size_field {
            true => bitstream.leb128(),
            false => sz - 1 - header.obu_extension_flag as u64,
        };

        let _start_position = bitstream.position;

        if !matches!(header.obu_type, ObuType::ObuSequenceHeader)
            && matches!(header.obu_type, ObuType::ObuTemporalDelimiter)
            && state.operating_point_idc != 0
            && header.obu_extension_flag
        {
            let in_temporal_layer = ((state.operating_point_idc
                >> header.obu_extension_header.clone().unwrap().temporal_id)
                & 1)
                != 0;

            let in_spatial_layer = (state.operating_point_idc
                >> (header.obu_extension_header.unwrap().spatial_id + 8)
                & 1)
                != 0;

            if !in_temporal_layer || !in_spatial_layer {
                OpenBitstreamUnit::drop_obu(bitstream, obu_size);
                return None;
            }
        }

        match header.obu_type {
            ObuType::ObuSequenceHeader => ObuSequenceHeader::new(bitstream, state),
            _ => todo!("not implemented"),
        };

        Some(OpenBitstreamUnit {})
    }

    fn drop_obu(bitstream: &mut BitStream, obu_size: u64) {
        bitstream.position += (obu_size * 8) as usize;
    }
}
