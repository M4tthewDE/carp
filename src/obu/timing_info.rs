use crate::bits::bitstream::BitStream;

pub struct TimingInfo {
    num_units_in_display_tick: u64,
    time_scale: u64,
    equal_picture_interval: bool,
    num_ticks_per_picture_minus_1: u64,
}

impl TimingInfo {
    pub fn new(bitstream: &mut BitStream) -> TimingInfo {
        let num_units_in_display_tick = bitstream.f(32);
        let time_scale = bitstream.f(32);
        let equal_picture_interval = bitstream.f(1) != 0;

        let num_ticks_per_picture_minus_1 = match equal_picture_interval {
            true => bitstream.uvlc(),
            false => 0,
        };

        TimingInfo {
            num_units_in_display_tick,
            time_scale,
            equal_picture_interval,
            num_ticks_per_picture_minus_1,
        }
    }
}
