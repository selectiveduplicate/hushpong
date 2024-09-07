use crate::chunk_type::ChunkType;

const POLY: u32 = 0xEDB88320;
const SIZE: usize = 256;

fn init_crc() -> [u32; SIZE] {
    let mut crc_table: [u32; SIZE] = [0; SIZE];
    let mut c: u32;

    for (byte, crc) in crc_table.iter_mut().enumerate() {
        c = byte as u32;
        for _ in 0..8 {
            if c & 1 == 1 {
                c = POLY ^ (c >> 1);
            } else {
                c >>= 1;
            }
        }
        *crc = c;
    }
    crc_table
}

fn update_crc(buf: &[u8], crc_table: &[u32; 256]) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;

    for byte in buf {
        let index = (crc ^ (*byte as u32)) & 0xFF;
        crc = crc_table[index as usize] ^ (crc >> 8);
    }

    crc
}

pub(crate) fn calculate_crc(ctype: &ChunkType, data: &[u8]) -> u32 {
    let crc_table = init_crc();

    let crc = update_crc(&[ctype.bytes().as_slice(), data].concat(), &crc_table);

    crc ^ 0xFFFFFFFFu32
}
