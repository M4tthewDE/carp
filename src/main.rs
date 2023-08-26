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
    operating_point_idc: u64,
    order_hint_bits: u64,
    bit_depth: u64,
    num_planes: u64,
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
            let _obu = OpenBitstreamUnit::new(
                b,
                sz,
                &mut self.operating_point_idc,
                &mut self.order_hint_bits,
                &mut self.bit_depth,
                &mut self.num_planes,
            );
            sz -= obu_length;
        }
    }
}
