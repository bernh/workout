use std::mem;

// constants
const FIT_FILE_HDR_SIZE : usize = 14;

#[derive(Clone, Debug)]
#[repr(packed)]
pub struct Fit_File_Header {
    pub header_size: u8,
    pub protocol_version: u8,
    pub profile_version: u16,
    pub data_size: u32, // Does not include file header or crc.  Little endian format.
    pub data_type: [u8; 4],  // ".FIT"
    pub crc: u16 // CRC of this file header in little endian format.
}

impl Fit_File_Header {

    pub fn new() -> Fit_File_Header {
        let FIT_PROTOCOL_VERSION_MAJOR_SHIFT = 4;
        let FIT_PROTOCOL_VERSION_20 = (( 2 << FIT_PROTOCOL_VERSION_MAJOR_SHIFT ) | 0 );

        Fit_File_Header {
            header_size: 14,
            protocol_version: FIT_PROTOCOL_VERSION_20,
            profile_version: 2,
            data_size: 0,
            data_type: [46, 70, 73, 84 ], // ".FIT"
            crc: 0,
        }
    }

    pub fn bin(&self) -> [u8; FIT_FILE_HDR_SIZE] {
        unsafe { mem::transmute(self.clone()) }
    }
}

