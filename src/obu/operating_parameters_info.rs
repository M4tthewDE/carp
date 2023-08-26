use crate::bits::bitstream::BitStream;

#[derive(Default)]
pub struct OperatingParamtersInfo {
    pub decoder_buffer_delay: Vec<u64>,
    pub encoder_buffer_delay: Vec<u64>,
    pub low_delay_mode_flag: Vec<bool>,
}

impl OperatingParamtersInfo {
    pub fn new(
        b: &mut BitStream,
        op: usize,
        buffer_delay_length_minus_1: u64,
    ) -> OperatingParamtersInfo {
        let n = buffer_delay_length_minus_1 + 1;

        let mut opi = OperatingParamtersInfo::default();

        opi.decoder_buffer_delay.insert(op, b.f(n));
        opi.encoder_buffer_delay.insert(op, b.f(n));
        opi.low_delay_mode_flag.insert(op, b.f(1) != 0);

        opi
    }
}
