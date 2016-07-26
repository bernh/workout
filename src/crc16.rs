fn fit_ctc_get16(mut crc: u16, byte: &u8) -> u16 {
    let crc_table = [0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401, 0xA001,
                     0x6C00, 0x7800, 0xB401, 0x5000, 0x9C01, 0x8801, 0x4400];
    let mut tmp: u16;

    // compute checksum of lower four bits of byte
    tmp = crc_table[(crc & 0xF) as usize];
    crc = (crc >> 4) & 0x0FFF;
    crc = crc ^ tmp ^ crc_table[(byte & 0xF) as usize];

    // now compute checksum of upper four bits of byte
    tmp = crc_table[(crc & 0xF) as usize];
    crc = (crc >> 4) & 0x0FFF;
    crc = crc ^ tmp ^ crc_table[(((byte >> 4) as u8) & 0xF) as usize];

    crc
}

pub fn fit_crc_update16(mut crc: u16, data: &[u8]) -> u16 {
    for byte in data {
        crc = fit_ctc_get16(crc, &byte);
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc() {
        let bytes = [14, 32, 2, 0, 0, 0, 0, 0, 46, 70, 73, 84];
        assert_eq!(fit_crc_update16(0, &bytes), 0x4f3d);
    }
}
