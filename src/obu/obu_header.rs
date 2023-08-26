use crate::bits::bitstream::BitStream;

pub struct ObuHeader {
    pub obu_forbidden_bit: bool,
    pub obu_type: ObuType,
    pub obu_extension_flag: bool,
    pub obu_has_size_field: bool,
    pub obu_reserved_1bit: bool,
    pub obu_extension_header: Option<ObuExtensionHeader>,
}

impl ObuHeader {
    pub fn new(bitstream: &mut BitStream) -> ObuHeader {
        let obu_forbidden_bit = bitstream.f(1) != 0;
        let obu_type = ObuType::new(bitstream.f(4));
        let obu_extension_flag = bitstream.f(1) != 0;
        let obu_has_size_field = bitstream.f(1) != 0;
        let obu_reserved_1bit = bitstream.f(1) != 0;

        let obu_extension_header = match obu_extension_flag {
            true => Some(ObuExtensionHeader::new(bitstream)),
            false => None,
        };

        ObuHeader {
            obu_forbidden_bit,
            obu_type,
            obu_extension_flag,
            obu_has_size_field,
            obu_reserved_1bit,
            obu_extension_header,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ObuType {
    Reserved,
    ObuSequenceHeader,
    ObuTemporalDelimiter,
    ObuFrameHeader,
    ObuTileGroup,
    ObuMetadata,
    ObuFrame,
    ObuRedundantFrameHeader,
    ObuTileList,
    ObuPadding,
}

impl ObuType {
    fn new(i: u64) -> ObuType {
        match i {
            0 => ObuType::Reserved,
            1 => ObuType::ObuSequenceHeader,
            2 => ObuType::ObuTemporalDelimiter,
            3 => ObuType::ObuFrameHeader,
            4 => ObuType::ObuFrameHeader,
            5 => ObuType::ObuMetadata,
            6 => ObuType::ObuFrame,
            7 => ObuType::ObuRedundantFrameHeader,
            8 => ObuType::ObuTileList,
            9..=14 => ObuType::Reserved,
            15 => ObuType::ObuPadding,
            _ => panic!("Invalid value for obu_type {i}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObuExtensionHeader {
    pub temporal_id: u64,
    pub spatial_id: u64,
    extension_header_reserved_3bits: u64,
}

impl ObuExtensionHeader {
    pub fn new(bitstream: &mut BitStream) -> ObuExtensionHeader {
        let temporal_id = bitstream.f(3);
        let spatial_id = bitstream.f(2);
        let extension_header_reserved_3bits = bitstream.f(3);

        ObuExtensionHeader {
            temporal_id,
            spatial_id,
            extension_header_reserved_3bits,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bits::bitstream::BitStream,
        obu::obu_header::{ObuExtensionHeader, ObuType},
    };

    use super::ObuHeader;

    #[test]
    fn obu_header() {
        let mut bs = BitStream::new(vec![8, 15]);
        let obu_header = ObuHeader::new(&mut bs);

        assert_eq!(false, obu_header.obu_forbidden_bit);
        assert_eq!(ObuType::ObuSequenceHeader, obu_header.obu_type);
        assert_eq!(false, obu_header.obu_extension_flag);
        assert_eq!(false, obu_header.obu_has_size_field);
        assert_eq!(false, obu_header.obu_reserved_1bit);
        assert_eq!(None, obu_header.obu_extension_header);
    }

    #[test]
    fn obu_extension_header() {
        let mut bs = BitStream::new(vec![22]);
        let obu_extension_header = ObuExtensionHeader::new(&mut bs);

        assert_eq!(0, obu_extension_header.temporal_id);
        assert_eq!(2, obu_extension_header.spatial_id);
        assert_eq!(6, obu_extension_header.extension_header_reserved_3bits);
    }
}
