use std::mem;

mod crc16;

// constants
pub const FIT_FILE_HDR_SIZE : usize = 14;

#[derive(Clone, Debug)]
#[repr(packed)]
pub struct FitFileHeader {
    pub header_size: u8,
    pub protocol_version: u8,
    pub profile_version: u16,
    pub data_size: u32, // Does not include file header or crc.  Little endian format.
    pub data_type: [u8; 4],  // ".FIT"
    pub crc: u16 // CRC of this file header in little endian format.
}

impl FitFileHeader {

    pub fn new() -> FitFileHeader {
        let fit_protocol_version_major_shift = 4;
        let fit_protocol_version_20 = 2 << fit_protocol_version_major_shift;

        FitFileHeader {
            header_size: FIT_FILE_HDR_SIZE as u8,  // TODO test is corresponding to sizeof()
            protocol_version: fit_protocol_version_20,
            profile_version: 2,
            data_size: 0,
            data_type: [46, 70, 73, 84 ], // ".FIT"
            crc: 0,
        }
    }

    pub fn bin(&self) -> [u8; FIT_FILE_HDR_SIZE] {
        unsafe { mem::transmute(self.clone()) }
    }

    pub fn calc_crc(&mut self) {
        let bytes = self.bin();
        self.crc = crc16::fit_crc_update16(0, &bytes[.. bytes.len() -2]);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn FitFileHeader_size() {
        assert_eq!(FIT_FILE_HDR_SIZE, mem::size_of::<FitFileHeader>());
    }
}
