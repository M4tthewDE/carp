use crate::bits::bitstream::BitStream;

#[derive(Clone)]
pub struct DecoderModelInfo {
    pub buffer_delay_length_minus_1: u64,
    num_units_in_decoding_tick: u64,
    buffer_removal_time_length_minus_1: u64,
    frame_presentation_time_length_minus_1: u64,
}

impl DecoderModelInfo {
    pub fn new(b: &mut BitStream) -> DecoderModelInfo {
        let buffer_delay_length_minus_1 = b.f(5);
        let num_units_in_decoding_tick = b.f(32);
        let buffer_removal_time_length_minus_1 = b.f(5);
        let frame_presentation_time_length_minus_1 = b.f(5);

        DecoderModelInfo {
            buffer_delay_length_minus_1,
            num_units_in_decoding_tick,
            buffer_removal_time_length_minus_1,
            frame_presentation_time_length_minus_1,
        }
    }
}
