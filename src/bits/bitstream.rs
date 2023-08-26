pub struct BitStream {
    bits: Vec<u8>,
    position: usize,
}

impl BitStream {
    pub fn new(bits: Vec<u8>) -> BitStream {
        BitStream { bits, position: 0 }
    }

    pub fn f(&mut self, n: u64) -> u64 {
        let mut x = 0;
        for _ in 0..n {
            x = 2 * x + self.read_bit() as u64;
        }

        x
    }

    fn read_bit(&mut self) -> u8 {
        let bit = (self.bits.get(self.position / 8).unwrap() >> (7 - self.position % 8)) & 1;
        self.position += 1;

        bit
    }
}

#[cfg(test)]
mod tests {
    use super::BitStream;

    #[test]
    fn read_bit() {
        let mut bs = BitStream::new(vec![3, 5]);

        assert_eq!(0, bs.read_bit());
        assert_eq!(0, bs.read_bit());
        assert_eq!(0, bs.read_bit());
        assert_eq!(0, bs.read_bit());
        assert_eq!(0, bs.read_bit());
        assert_eq!(0, bs.read_bit());
        assert_eq!(1, bs.read_bit());
        assert_eq!(1, bs.read_bit());

        assert_eq!(0, bs.read_bit());
        assert_eq!(0, bs.read_bit());
        assert_eq!(0, bs.read_bit());
        assert_eq!(0, bs.read_bit());
        assert_eq!(0, bs.read_bit());
        assert_eq!(1, bs.read_bit());
        assert_eq!(0, bs.read_bit());
        assert_eq!(1, bs.read_bit());
    }

    #[test]
    fn f() {
        let mut bs = BitStream::new(vec![5, 6]);

        assert_eq!(5, bs.f(8));
        assert_eq!(6, bs.f(8));
    }

    #[test]
    fn f_more_than_one_byte() {
        let mut bs = BitStream::new(vec![5, 6]);

        assert_eq!(1286, bs.f(16));
    }
}
