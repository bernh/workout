mod crc16;
mod wtree;
mod parse;
mod jd_grammar;
mod config;

use wtree::{DistanceAndTime};

#[macro_use]
extern crate log;

extern crate byteorder;
use byteorder::{LittleEndian, WriteBytesExt};

// constants
pub const FIT_FILE_HDR_SIZE: usize = 14;

#[derive(Clone, Debug)]
pub struct FitFileHeader {
    header_size: u8,
    protocol_version: u8,
    profile_version: u16,
    data_size: u32, // Does not include file header or crc.  Little endian format.
    data_type: [u8; 4], // ".FIT"
    crc: u16, // CRC of this file header in little endian format.
}

impl FitFileHeader {
    pub fn new() -> FitFileHeader {
        let fit_protocol_version_major_shift = 4;
        let fit_protocol_version_10 = 1 << fit_protocol_version_major_shift;

        FitFileHeader {
            header_size: FIT_FILE_HDR_SIZE as u8,
            protocol_version: fit_protocol_version_10,
            profile_version: 2014,
            data_size: 0,
            data_type: [46, 70, 73, 84], // ".FIT"
            crc: 0,
        }
    }

    pub fn bin(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u8(self.header_size).unwrap();
        wtr.write_u8(self.protocol_version).unwrap();
        wtr.write_u16::<LittleEndian>(self.profile_version).unwrap();
        wtr.write_u32::<LittleEndian>(self.data_size).unwrap();
        wtr.extend(self.data_type.iter().cloned());
        wtr.write_u16::<LittleEndian>(self.crc).unwrap();
        wtr
    }

    pub fn size(&mut self, size : u32) -> &mut FitFileHeader {
        self.data_size = size;
        self
    }

    pub fn calc_crc(&mut self) -> &mut FitFileHeader {
        let bytes = self.bin();
        self.crc = crc16::fit_crc_update16(0, &bytes[..bytes.len() - 2]);
        self
    }
}

pub trait DataRecord {
    fn header_byte(&self) -> Vec<u8>;
}

pub trait FITMessage {
    fn definition_message(&self) -> Vec<u8>;
    fn data_message(&self) -> Vec<u8>;
    fn bin(&self) -> Vec<u8>;
    // TODO create default implementation for bin
}






pub fn simple_parse(input: String) {
    let w = jd_grammar::parse_Workout_main(&parse::preprocess_input(&input)).unwrap();
    info!("{}", input);
    info!("({:.*} km, {}:{:02} h)", 1, w.distance() as f32 / 1000.0,
          w.time() as i32 / 3600, w.time() as i32 % 3600 / 60);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fit_file_header_size() {
        let fh = FitFileHeader::new();
        assert_eq!(FIT_FILE_HDR_SIZE, fh.bin().len());
    }

    #[test]
    fn file_header() {
        let header = FitFileHeader::new()
                            .size(360)
                            .calc_crc()
                            .bin();
        assert_eq!(vec![0x0e, 0x10, 0xde, 0x07, 0x68, 0x01, 0x0, 0x0, 
                        0x2e, 0x46, 0x49, 0x54, 0x37, 0xa7],
                   header);
    }
}
