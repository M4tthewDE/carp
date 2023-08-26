use bits::bitstream::BitStream;
use obu::open_bitstream_unit::OpenBitstreamUnit;

mod bits;
mod obu;

fn main() {
    let mut b = BitStream::new(vec![]);

    let mut p = Parser::default();

    p.bitstream(&mut b);
}

#[derive(Default)]
struct Parser {
    state: State,
}

#[derive(Default)]
pub struct State {
    operating_point_idc: u64,
    order_hint: u64,
    order_hint_bits: u64,
    bit_depth: u64,
    num_planes: u64,
    seen_frame_header: bool,
    tile_num: bool,
    frame_is_intra: bool,
    ref_frame_type: Vec<u64>,
    ref_valid: Vec<bool>,
    ref_order_hint: Vec<u64>,
    order_hints: Vec<u64>,
    prev_frame_id: u64,
    upscaled_width: u64,
    frame_width: u64,
    delta_frame_id: u64,
    ref_frame_sign_bias: Vec<u64>,
}

impl Parser {
    fn bitstream(&mut self, b: &mut BitStream) {
        while b.more_data_in_bitstream() {
            let temporal_unit_size = b.leb128();

            self.temporal_unit(b, temporal_unit_size);
        }
    }

    fn temporal_unit(&mut self, b: &mut BitStream, size: u64) {
        let mut sz = size;
        while sz > 0 {
            let frame_unit_size = b.leb128();
            sz -= b.leb_128_bytes;
            self.frame_unit(b, sz);
            sz -= frame_unit_size;
        }
    }

    fn frame_unit(&mut self, b: &mut BitStream, size: u64) {
        let mut sz = size;
        while sz > 0 {
            let obu_length = b.leb128();
            sz -= b.leb_128_bytes;
            let _obu = OpenBitstreamUnit::new(b, sz, &mut self.state);
            sz -= obu_length;
        }
    }
}
